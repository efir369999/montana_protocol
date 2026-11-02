#!/usr/bin/env python3
"""iOS 2311 pipe formulas, re-implemented from the Swift source with the standard library only.

Source of truth: upstream main at c7e11da
  ("apps/messenger/ios: build 1.0 (2311), source 648c3225b9ae"; RELEASES.md:18).
The source before it: 2308 = 59d30f3 (the iOS tree is byte-identical at d5c6448: empty diff).
CHANGELOG.md:5-13 (2311: "changes neither security nor anonymity", only the order of a long message's pieces),
CHANGELOG.md:15-23 (2310: the day walk, the day tag, the 249-byte floor), CHANGELOG.md:25-33 (2309: one owner, wire unchanged).

Citation form: "2311 F:L" is apps/messenger/ios/Code/Montana/F at c7e11da, "2308 F:L" the same path at 59d30f3;
MontanaTests.swift lies under Code/MontanaTests. "new" means the line has no counterpart in 2308.

Run:  python -I vectors_2311.py [--json OUT.json]
Exit: 0 when every vector the 2311 tree states (code self-checks, XCTest vectors, the audit's coin vectors, the Canon
      lines the code cites) reproduces and every property check holds; 1 otherwise. The wrong implementations a comment
      names only by an 8-hex prefix are tried and reported, never counted.
"""
import base64
import datetime
import hashlib
import hmac
import json
import sys
from collections import namedtuple

# ── constants ────────────────────────────────────────────────────────────────────────────────────────────────────────
WALK_ANCHOR_DAY = 20754              # 28.10.2026 UTC. 2311 MTPipeKey.swift:50 | 2308 MTNodeWire.swift:72
MAX_WALK = 36_600                    # 2311 MTPipeKey.swift:51 | 2308 MTNodeWire.swift:88
TAG_WALK_DAY = WALK_ANCHOR_DAY + 4   # 20758, 01.11.2026 UTC. 2311 MTPipeKey.swift:60 | new
ERASE_DAY = TAG_WALK_DAY + 2         # 20760, 03.11.2026 UTC. 2311 MTPipeKey.swift:61 | new
FLOOR_LAG = 2                        # F = D - 2. 2311 MTPipeKey.swift:62 | new
MINUTES_PER_DAY = 1440               # 2311 MTNodeWire.swift:89 | 2308 MTNodeWire.swift:73
FLOOR_FORM = 0x01                    # 2311 MTPipeKey.swift:235 | new
FLOOR_FORM_SIZE = 1 + 8 + 32 * 7 + 16  # 249. 2311 MTPipeKey.swift:236 | new
FINGERPRINT_ITERATIONS = 5200        # 2311 MTNodeWire.swift:59 | 2308 MTPipe.swift:127
AUTHOR_FIELDS = 7                    # 2311 MTNodeWire.swift:138 | 2308 MTNodeWire.swift:153
ENVELOPE_SIZE = 2048                 # 2311 MTNodeWire.swift:107 | 2308 MTNodeWire.swift:122
# The clocks: a minute window = unix seconds / 60 (2311 MTPipe.swift:27-31, MTNodeWire.swift:243 | 2308 MTPipe.swift:34-38,
# MTNodeWire.swift:254); a day = unix seconds / 86400 (2311 MTNodeWire.swift:244 | 2308 MTNodeWire.swift:255); the day of a
# minute window = window / 1440 (2311 MTPipeKey.swift:97,106,112). The UTC day boundary is therefore 00:00:00 UTC.


# ── the primitives (MTNodeWire, the one owner of the compositions) ───────────────────────────────────────────────────
def sha256(b):
    return hashlib.sha256(b).digest()


def domained(domain, parts):
    """SHA-256(domain || 0x00 || parts...). 2311 MTNodeWire.swift:41-48 | 2308 MTPipe.swift:16-24."""
    return sha256(domain.encode() + b"\x00" + b"".join(parts))


def window_le(w):
    """W_8B_LE. 2311 MTNodeWire.swift:49-52 | 2308 MTPipe.swift:26-29."""
    return int(w).to_bytes(8, "little")


def tag_of(shared, w):
    """SHA-256("mt-tag" || 0x00 || shared || W_LE)[0..16]. 2311 MTNodeWire.swift:53-56 | 2308 MTPipe.swift:46-49."""
    return domained("mt-tag", [shared, window_le(w)])[:16]


def step_label(handshake, w):
    """2311 MTPipe.swift:43-46 | 2308 MTPipe.swift:52-54."""
    return domained("mt-step", [handshake, window_le(w)])[:16]


def relay_seal(owner, label):
    """2311 MTPipe.swift:80-84 | 2308 MTPipe.swift:93-95."""
    return domained("mt-relay-seal", [owner, label])[:16]


def owner_ref(owner, shared):
    """2311 MTPipe.swift:86-90 | 2308 MTPipe.swift:99-101."""
    return domained("mt-owner-ref", [owner, shared])[:16]


def holder(tag, commitments):
    """The least commitment above the tag, else the lowest. 2311 MTPipe.swift:92-101 | 2308 MTPipe.swift:106-112."""
    if not commitments:
        return None
    key = tag[:16]
    ordered = sorted(commitments, key=lambda c: c[:16])
    for c in ordered:
        if key < c[:16]:
            return c
    return ordered[0]


def seal_key(step_secret, w):
    """SHA-256("mt-pipe-key" || 0x00 || step_secret || W_LE). 2311 MTNodeWire.swift:72-77 | 2308 MTNodeWire.swift:41-46."""
    return sha256(b"mt-pipe-key\x00" + step_secret + window_le(w))


def conv_tag(day_secret, w):
    """hex SHA-256("mt-wake-conv" || 0x00 || day_secret || W_LE). 2311 MTNodeWire.swift:78-86 | 2308 MTNodeWire.swift:55-63."""
    return hashlib.sha256(b"mt-wake-conv\x00" + day_secret + window_le(w)).hexdigest()


def sub_id(conv, ref):
    """2311 MTNodeWire.swift:96-102 | 2308 MTNodeWire.swift:111-117."""
    return hashlib.sha256(b"mt-wake-sub\x00" + conv.encode() + b"\x00" + ref.encode()).hexdigest()


def fingerprint_material(key):
    """h = SHA-256("mt-safety" || 0x00 || h), 5200 times. 2311 MTNodeWire.swift:57-64 | 2308 MTPipe.swift:127-136."""
    h = key
    for _ in range(FINGERPRINT_ITERATIONS):
        h = domained("mt-safety", [h])
    return h


def fingerprint_digits(h):
    """Six big-endian groups of five bytes, each mod 100000, "%05llu". 2311 MTPipe.swift:130-138 | 2308 MTPipe.swift:144-152."""
    if len(h) != 32:
        return None
    return "".join("%05d" % (int.from_bytes(h[i * 5:i * 5 + 5], "big") % 100000) for i in range(6))


def fingerprint_of_secret(secret):
    """2311 MTPipe.swift:124-127 | 2308 MTPipe.swift:138-141."""
    return fingerprint_digits(fingerprint_material(secret)) if len(secret) == 32 else None


def first_confirm(secret, ct):
    """SHA-256("mt-first-conf" || 0x00 || secret || ct)[0..16], base64url without padding.
    2311 MTNodeWire.swift:65-70 | 2308 MontanaFirstContact.swift:62-64."""
    d = domained("mt-first-conf", [secret, ct])[:16]
    return base64.b64encode(d).decode().replace("+", "-").replace("/", "_").replace("=", "")


def author_tag(person_key, secret, mid, text):
    """base64(HMAC-SHA-256(key = person, "mt-letter-author" || 0x00 || secret || mid || 0x00 || text)[0..16]).
    2311 MTNodeWire.swift:139-144 | 2308 MTNodeWire.swift:154-159."""
    m = b"mt-letter-author\x00" + secret + mid.encode() + b"\x00" + text.encode()
    return base64.b64encode(hmac.new(person_key, m, hashlib.sha256).digest()[:16]).decode()


def with_author_tag(body, tag):
    """The eighth field. 2311 MTNodeWire.swift:145-155 | 2308 MTNodeWire.swift:170-181 (where the tag was made inside)."""
    b = bytearray(body)
    if b and b[-1] != 0:
        b.append(0)
    closed = b.count(0)
    while closed < AUTHOR_FIELDS:
        b.append(0)
        closed += 1
    if closed != AUTHOR_FIELDS:
        return bytes(body)
    b += tag.encode()
    b.append(0)
    return bytes(b) if len(b) <= ENVELOPE_SIZE else bytes(body)


def coin_blind(secret, nonce):
    """HMAC-SHA-256(key = secret, "mt-coin-blind" || nonce). 2311 MTWalletCore.swift:3973-3976 | 2308 MTWalletCore.swift:3974."""
    return hmac.new(secret, b"mt-coin-blind" + nonce, hashlib.sha256).digest()


def coin_commitment(frm, blind):
    """SHA-256("mt-coin-to" || from || blind), no separator. 2311 MTWalletCore.swift:3977-3978 | 2308 MTWalletCore.swift:3978."""
    return sha256(b"mt-coin-to" + frm + blind)


def coin_pair_tag(secret):
    """HMAC-SHA-256(key = secret, "mt-coin-pair-v1"). 2311 MTPipeKey.swift:213 | 2308 MTWalletCore.swift:4140,4197-4199."""
    return hmac.new(secret, b"mt-coin-pair-v1", hashlib.sha256).digest()


def unhex(s):
    """2311 MTPipeKey.swift:216-225."""
    if len(s) % 2:
        return None
    try:
        return bytes.fromhex(s)
    except ValueError:
        return None


# ── MTPipeKey: the one owner of a pipe's secret (2311 MTPipeKey.swift, new file) ─────────────────────────────────────
Floor = namedtuple("Floor", "day value root rank fingerprint face_bid face_key pair ref")   # 2311 MTPipeKey.swift:26-31
Kept = namedtuple("Kept", "root fingerprint face_bid face_key pair ref")                     # 2311 MTPipeKey.swift:384-391


class PipeKey:
    def __init__(self, secret=b"", floor=None):            # 2311 MTPipeKey.swift:33-34
        self.secret = b"" if floor is not None else bytes(secret)
        self.floor = floor

    def __eq__(self, other):                                 # Swift's synthesized Equatable over (secret, floor)
        return isinstance(other, PipeKey) and (self.secret, self.floor) == (other.secret, other.floor)

    def __hash__(self):
        return hash((self.secret, self.floor))

    @property
    def is_whole(self):                                      # 2311 MTPipeKey.swift:38
        return self.floor is not None or len(self.secret) == 32

    @property
    def holds_origin(self):                                  # 2311 MTPipeKey.swift:40
        return self.floor is None and len(self.secret) == 32

    @staticmethod
    def steps(day):                                          # 2311 MTPipeKey.swift:65 | 2308 MTNodeWire.swift:92
        return min(day - WALK_ANCHOR_DAY if day > WALK_ANCHOR_DAY else 0, MAX_WALK)

    def walked(self, day):
        """2311 MTPipeKey.swift:66-81 (the cache of :64,:74,:78-79 saves hashes only and is left out) |
        2308 MTNodeWire.swift:91-104 (pipeSecret, from the established secret only)."""
        if self.floor is not None:
            at, value = self.steps(self.floor.day), self.floor.value
        else:
            at, value = 0, self.secret
        target = self.steps(day)
        if target <= at:
            return value
        while at < target:
            value = domained("mt-pipe-ratchet", [value])   # 2311 MTPipeKey.swift:76 | 2308 MTNodeWire.swift:98-99
            at += 1
        return value

    def floored(self, day):
        """2311 MTPipeKey.swift:83-93 | new."""
        if not (day >= ERASE_DAY and self.is_whole):
            return self
        f = day - FLOOR_LAG
        if self.floor is not None and self.floor.day >= f:
            return self
        k = self.kept()
        ref = self.ref_bytes()
        bid = unhex(k.face_bid)
        if ref is None or bid is None or len(bid) != 32:
            return self
        return PipeKey(floor=Floor(f, self.walked(f), self.root_key(), self.same_rank(), k.fingerprint, bid,
                                   k.face_key, k.pair, ref))

    def body_key(self, window):                              # 2311 MTPipeKey.swift:95-98 | 2308 MTNodeWire.swift:47-50
        return seal_key(self.walked(window // MINUTES_PER_DAY), window)

    def conv_w(self, day):                                   # 2311 MTPipeKey.swift:99-102 | 2308 MTNodeWire.swift:51-54
        return conv_tag(self.walked(day), day)

    def tag(self, window):
        """2311 MTPipeKey.swift:103-108 | 2308 MTPipe.swift:47-49 over the established secret, always."""
        day = window // MINUTES_PER_DAY
        return tag_of(self.walked(day) if (day >= TAG_WALK_DAY or not self.holds_origin) else self.secret, window)

    def tags(self, window):
        """2311 MTPipeKey.swift:109-115 | new (2308 accepted one tag per window: MTPipe.swift:784-790)."""
        day = window // MINUTES_PER_DAY
        now = self.tag(window)
        if not (self.holds_origin and TAG_WALK_DAY <= day < ERASE_DAY):
            return [now]
        return [now, tag_of(self.secret, window)]

    @staticmethod
    def root(secret):                                        # 2311 MTPipeKey.swift:122 | 2308 MTNodeWire.swift:78-81
        return domained("mt-conv-root", [secret])

    def root_key(self):                                      # 2311 MTPipeKey.swift:123 | new (2308 computed convRoot each time)
        return self.floor.root if self.floor is not None else self.root(self.secret)

    def item_key(self, day):                                 # 2311 MTPipeKey.swift:126 | 2308 MTNodeWire.swift:84
        return self.root_key() if (day > WALK_ANCHOR_DAY or not self.holds_origin) else self.secret

    def item_keys(self):                                     # 2311 MTPipeKey.swift:129 | 2308 MTNodeWire.swift:87 (always both)
        return [self.root_key(), self.secret] if self.holds_origin else [self.root_key()]

    def blind(self, nonce, day):                             # 2311 MTPipeKey.swift:133-135 | 2308 MTWalletCore.swift:4842
        return coin_blind(self.item_key(day), nonce)

    def blinds(self, nonce):                                 # 2311 MTPipeKey.swift:137 | 2308 MTWalletCore.swift:4881
        return [coin_blind(k, nonce) for k in self.item_keys()]

    def with_author(self, body, person, mid, text, day):     # 2311 MTPipeKey.swift:141-144 | 2308 MTNodeWire.swift:172-181
        if person is None or len(person) != 32:
            return bytes(body)
        return with_author_tag(body, author_tag(person, self.item_key(day), mid, text))

    def author_holds(self, tag, person, mid, text):          # 2311 MTPipeKey.swift:147-155 | 2308 MTNodeWire.swift:161-169
        if person is None or len(person) != 32 or not tag:
            return False
        try:
            got = base64.b64decode(tag, validate=True)
        except ValueError:
            return False
        if len(got) != 16:
            return False
        return any(hmac.compare_digest(got, base64.b64decode(author_tag(person, k, mid, text))) for k in self.item_keys())

    def author_tag(self, person, mid, text, day):
        """The group word's tag (MontanaGroup.ownTag). 2311 MTPipeKey.swift:156-161, MontanaGroup.swift:2124-2127 |
        2308 MontanaGroup.swift:2124-2127 (over the established secret, every day)."""
        return author_tag(person, self.item_key(day), mid, text)

    def first_confirm(self, ct):                             # 2311 MTPipeKey.swift:164-166 | 2308 MontanaFirstContact.swift:62-64
        return first_confirm(self.secret, ct) if self.holds_origin else ""

    def same_tag(self, domain, newer, day):
        """2311 MTPipeKey.swift:167-172 | 2308 MontanaChatStore.swift:7532-7534 (over the two secrets, every day)."""
        if day < TAG_WALK_DAY and self.holds_origin and newer.holds_origin:
            return self.same_origin(domain, self, newer)
        return domained(domain, [self.root_key(), newer.root_key()])[:16].hex()

    def same_tags(self, domain, newer, day):                 # 2311 MTPipeKey.swift:173-178 | new
        now = self.same_tag(domain, newer, day)
        if not (TAG_WALK_DAY <= day < ERASE_DAY and self.holds_origin and newer.holds_origin):
            return [now]
        return [now, self.same_origin(domain, self, newer)]

    @staticmethod
    def same_origin(domain, older, newer):                   # 2311 MTPipeKey.swift:179-182 | 2308 MontanaChatStore.swift:7532-7534
        return domained(domain, [older.secret, newer.secret])[:16].hex()

    def same_rank(self):                                     # 2311 MTPipeKey.swift:183-185 | 2308 MontanaChatStore.swift:7541
        return self.floor.rank if self.floor is not None else domained("mt-same-rank", [self.secret])

    def ref_bytes(self):                                     # 2311 MTPipeKey.swift:189-194 | 2308 MTPipe.swift:551-555
        if self.floor is not None:
            return self.floor.ref
        if len(self.secret) != 32:
            return None
        return sha256(self.secret)[:16]

    def reference(self):                                     # 2311 MTPipeKey.swift:195-198
        r = self.ref_bytes()
        return r.hex() if (self.is_whole and r is not None) else None

    def kept(self):
        """2311 MTPipeKey.swift:202-215 | 2308 MTPipe.swift:262-269 (MTPipeKept.of), MontanaFirstContact.swift:218-225."""
        f = self.floor
        if f is not None:
            return Kept(f.root, f.fingerprint, f.face_bid.hex(), f.face_key, f.pair, self.reference())
        s = self.secret
        return Kept(self.root(s), fingerprint_material(s), sha256(b"mt-pipe-f\x00" + s).hex(),
                    sha256(b"mt-pipe-fk\x00" + s), coin_pair_tag(s), self.reference())

    def sealed_form(self):
        """0x01 || F (8, LE) || walk(F) || root || rank || fingerprint || face name || face key || pair (32 each) || ref (16).
        2311 MTPipeKey.swift:233-244 | new (2308 stored the 32 bytes: MTPipe.swift MTPipeBook.store)."""
        f = self.floor
        if f is None:
            return self.secret
        return (bytes([FLOOR_FORM]) + f.day.to_bytes(8, "little") + f.value + f.root + f.rank + f.fingerprint
                + f.face_bid + f.face_key + f.pair + f.ref)

    @classmethod
    def from_sealed(cls, raw):                               # 2311 MTPipeKey.swift:245-253
        d = bytes(raw)
        if len(d) != FLOOR_FORM_SIZE or d[0] != FLOOR_FORM:
            return cls(d)
        day = int.from_bytes(d[1:9], "little")
        part = lambda n: d[9 + 32 * n:9 + 32 * n + 32]
        return cls(floor=Floor(day, part(0), part(1), part(2), part(3), part(4), part(5), part(6),
                               d[9 + 32 * 7:FLOOR_FORM_SIZE]))

    def archive_head(self):                                  # 2311 MTPipeKey.swift:272-275 | 2308 MontanaPQ.swift (s: + 32 bytes)
        return b"s:" + base64.b64encode(self.sealed_form())

    @classmethod
    def from_archive_head(cls, head, day):                   # 2311 MTPipeKey.swift:276-281
        if not head.startswith("s:"):
            return None
        try:
            s = base64.b64decode(head[2:], validate=True)
        except ValueError:
            return None
        k = cls.from_sealed(s)
        return k.floored(day) if k.is_whole else None


def stored(keys, day):                                       # 2311 MTPipeKey.swift:261-263
    return {c: k.floored(day).sealed_form() for c, k in keys.items()}


def held(forms, day):                                        # 2311 MTPipeKey.swift:264-266
    return {c: PipeKey.from_sealed(s).floored(day) for c, s in forms.items()}


def lags(forms, day):                                        # 2311 MTPipeKey.swift:267-271
    return stored(held(forms, day), day) != forms


# ── the check harness ────────────────────────────────────────────────────────────────────────────────────────────────
RESULTS = []        # (group, cite, what, ok, got)
CONFIRMED = {}      # key -> (value, cite): values a stated vector of the 2311 tree pins
NEGATIVES = []      # (cite, what, prefix, matched candidate or None)


def hx(v):
    if isinstance(v, (bytes, bytearray)):
        return bytes(v).hex()
    if isinstance(v, (list, tuple)):
        return [hx(x) for x in v]
    return v


def check(group, cite, what, got, want=None, ok=None, pin=None):
    if ok is None:
        ok = got == want
    RESULTS.append((group, cite, what, bool(ok), hx(got)))
    if ok and pin is not None:
        CONFIRMED[pin] = (hx(got), cite)
    return ok


def negative(cite, what, prefix, candidates):
    hit = None
    for name, value in candidates:
        if hx(value).startswith(prefix):
            hit = name
            break
    NEGATIVES.append((cite, what, prefix, hit))


S32 = bytes(range(32))                   # bodySecret: 2311 MTPipeKey.swift:314, MTPipe.swift:179 | 2308 MTPipe.swift:193
PERSON = bytes([0x33]) * 32              # 2311 MTPipeKey.swift:342 | 2308 MTPipe.swift:228
A, Y, E, MPD = WALK_ANCHOR_DAY, TAG_WALK_DAY, ERASE_DAY, MINUTES_PER_DAY


def selfchecks():
    k = PipeKey(S32)
    G = "MTPipeKey.agreesWithCanon"
    # Block 1: the walk by the day, the body key and the daily label. 2311 MTPipeKey.swift:317-328 | 2308 MTPipe.swift:196-214
    check(G, "2311 MTPipeKey.swift:317-318 | 2308 MTPipe.swift:196-197", "bodyKey(window 1000)", k.body_key(1000),
          bytes.fromhex("e25542e65a84d1a5b5c72a13172ecb62319c18f21a59978a53e7dac2c22a966a"), pin=("body_key", "A", 1000))
    check(G, "2311 MTPipeKey.swift:319 | 2308 MTPipe.swift:205", "walked(anchor) == bodySecret", k.walked(A), S32,
          pin=("walked", "A", A))
    check(G, "2311 MTPipeKey.swift:320-321 | 2308 MTPipe.swift:206-207", "walked(anchor+1)", k.walked(A + 1),
          bytes.fromhex("5dcfd87692fcd444a4344d756dfa37abe670d2eb252aa9b402646be97cf3c50a"), pin=("walked", "A", A + 1))
    check(G, "2311 MTPipeKey.swift:322-323 | 2308 MTPipe.swift:208-209", "walked(anchor+2)", k.walked(A + 2),
          bytes.fromhex("0c31775e80ae463de61fecee68bee8bb4ebbcee8e6b2045f8714e93ef18cb1b0"), pin=("walked", "A", A + 2))
    check(G, "2311 MTPipeKey.swift:324-325 | 2308 MTPipe.swift:210-211", "bodyKey(window (anchor+2)*1440)",
          k.body_key((A + 2) * MPD),
          bytes.fromhex("925378a4bb83e7137cbb1f8cdaba8b79fc37e30d3159e4158da6e0fee8518bb5"), pin=("body_key", "A", (A + 2) * MPD))
    check(G, "2311 MTPipeKey.swift:326 | 2308 MTPipe.swift:212", "convW(day anchor)", k.conv_w(A),
          "d29425fcf807a325b74d8bb017420cfe2250cc47e8bdc6351f6006cd4a6bdc9b", pin=("conv_w", "A", A))
    check(G, "2311 MTPipeKey.swift:327 | 2308 MTPipe.swift:213", "convW(day anchor+2)", k.conv_w(A + 2),
          "c665fdca8978b9c6e54b9a4f16a06ccac657ba7da259df0a8f3e2550d7c2d518", pin=("conv_w", "A", A + 2))
    # Block 2: the root and the item keys. 2311 MTPipeKey.swift:333-337 | 2308 MTPipe.swift:219-223
    root = PipeKey.root(S32)
    check(G, "2311 MTPipeKey.swift:333 | 2308 MTPipe.swift:219", "root(bodySecret)", root,
          bytes.fromhex("d9c3e7f533da2d0ed10b6729601c478ab29d1b0cafed60443113ee6a63824e11"), pin=("root", "A"))
    check(G, "2311 MTPipeKey.swift:334 | 2308 MTPipe.swift:220", "itemKey(anchor) == bodySecret", k.item_key(A), S32,
          pin=("item_key", "A", A))
    check(G, "2311 MTPipeKey.swift:335 | 2308 MTPipe.swift:221", "itemKey(anchor+1) == root", k.item_key(A + 1), root,
          pin=("item_key", "A", A + 1))
    check(G, "2311 MTPipeKey.swift:336 | 2308 MTPipe.swift:222", "itemKeys == [root, bodySecret]", k.item_keys(), [root, S32])
    # Block 3: the author field. 2311 MTPipeKey.swift:342-352 | 2308 MTPipe.swift:228-238
    root_tag = author_tag(PERSON, root, "m", "t")
    anchor_tag = author_tag(PERSON, S32, "m", "t")
    check(G, "2311 MTPipeKey.swift:345 | 2308 MTPipe.swift:231", "rootTag != anchorTag", root_tag != anchor_tag, True)
    check(G, "2311 MTPipeKey.swift:346-347 | 2308 MTPipe.swift:232-233", "withAuthor(empty, day anchor+1) == 7x00 rootTag 00",
          k.with_author(b"", PERSON, "m", "t", A + 1), bytes(7) + root_tag.encode() + b"\x00")
    check(G, "2311 MTPipeKey.swift:348-349 | 2308 MTPipe.swift:234-235", "withAuthor(empty, day anchor) == 7x00 anchorTag 00",
          k.with_author(b"", PERSON, "m", "t", A), bytes(7) + anchor_tag.encode() + b"\x00")
    check(G, "2311 MTPipeKey.swift:350 | 2308 MTPipe.swift:236", "authorHolds(rootTag)", k.author_holds(root_tag, PERSON, "m", "t"), True)
    check(G, "2311 MTPipeKey.swift:351 | 2308 MTPipe.swift:237", "authorHolds(anchorTag)", k.author_holds(anchor_tag, PERSON, "m", "t"), True)
    # Block 4: the passage by the day (OZ1, decision 126). 2311 MTPipeKey.swift:360-377 | new
    fl = k.floored(E + 1)
    w_y1 = (Y - 1) * MPD + 5
    w_y = Y * MPD + 5
    w_e = E * MPD + 5
    w_e1 = (E + 1) * MPD + 5
    check(G, "2311 MTPipeKey.swift:362", "tag(window (Y-1)*1440+5): over the established secret", k.tag(w_y1),
          bytes.fromhex("ec0c1a23e2450dd465356cd0b8175c4f"), pin=("tag_new", "A", w_y1))
    if tag_of(S32, w_y1) == k.tag(w_y1):
        CONFIRMED[("tag_old", "A", w_y1)] = (k.tag(w_y1).hex(), "2311 MTPipeKey.swift:362 (before Y the tag is the old one, :107)")
    check(G, "2311 MTPipeKey.swift:363", "tag(window Y*1440+5): over the walk of Y", k.tag(w_y),
          bytes.fromhex("96d8937ada4908bcba9fd12c78b9c11c"), pin=("tag_new", "A", w_y))
    check(G, "2311 MTPipeKey.swift:364", "tags(window Y*1440+5) == [new, old]", [t.hex() for t in k.tags(w_y)],
          ["96d8937ada4908bcba9fd12c78b9c11c", "cc6997487bfa3ed9c4a7542be4fc2bbb"], pin=("tags", "A", w_y))
    if k.tags(w_y)[1] == tag_of(S32, w_y):
        CONFIRMED[("tag_old", "A", w_y)] = (tag_of(S32, w_y).hex(), "2311 MTPipeKey.swift:364 (the second tag, :114)")
    if k.tag(w_y).hex() == "96d8937ada4908bcba9fd12c78b9c11c":
        CONFIRMED[("walked", "A", Y)] = (k.walked(Y).hex(), "implied: the stated tag of 2311 MTPipeKey.swift:363 stands on it")
    check(G, "2311 MTPipeKey.swift:365", "tags(window E*1440+5).count == 1", len(k.tags(w_e)), 1)
    check(G, "2311 MTPipeKey.swift:366", "floored(E-1) == k and k.holdsOrigin", k.floored(E - 1) == k and k.holds_origin, True)
    check(G, "2311 MTPipeKey.swift:367", "floored(E+1): !holdsOrigin, isWhole, walked(20759) == walked(0)",
          (not fl.holds_origin) and fl.is_whole and fl.walked(20759) == fl.walked(0), True)
    check(G, "2311 MTPipeKey.swift:368", "floored(E+1).walked(20759) (F = 20759, five steps)", fl.walked(20759),
          bytes.fromhex("047f100cb7db75364efcefe4b58cf0038b1e1bc441884b06b5674eaf7b857523"), pin=("floor_value", "A", E + 1))
    if fl.walked(20759) == k.walked(20759):
        CONFIRMED[("walked", "A", 20759)] = (k.walked(20759).hex(), "2311 MTPipeKey.swift:368 (the floor's value is walk(F), :91)")
    check(G, "2311 MTPipeKey.swift:369", "floored(E+1).sealedForm.count == 249", len(fl.sealed_form()), FLOOR_FORM_SIZE)
    check(G, "2311 MTPipeKey.swift:370", "SHA-256(floored(E+1).sealedForm)", sha256(fl.sealed_form()),
          bytes.fromhex("da6e42f21c5935263ba7339fdc38adfd36db060a69d335f97e21f0dfbf1ae119"), pin=("floor_form_sha256", "A", E + 1))
    if sha256(fl.sealed_form()).hex() == "da6e42f21c5935263ba7339fdc38adfd36db060a69d335f97e21f0dfbf1ae119":
        pinned = "2311 MTPipeKey.swift:370 (a field of the record its SHA-256 pins)"
        CONFIRMED[("floor_form", "A", E + 1)] = (fl.sealed_form().hex(), "2311 MTPipeKey.swift:369-370 (its SHA-256 and size)")
        CONFIRMED[("rank", "A")] = (fl.floor.rank.hex(), pinned)
        CONFIRMED[("kept.fingerprint", "A")] = (fl.floor.fingerprint.hex(), pinned)
    check(G, "2311 MTPipeKey.swift:371", "floored(E+1).tag(window (E+1)*1440+5)", fl.tag(w_e1),
          bytes.fromhex("9e0c92b8f91b94880bf15eee1ca672c7"), pin=("tag_floored", "A", w_e1))
    check(G, "2311 MTPipeKey.swift:372", "floored tag == origin tag at (E+1)*1440+5", fl.tag(w_e1) == k.tag(w_e1), True)
    if fl.tag(w_e1) == k.tag(w_e1):
        CONFIRMED[("tag_new", "A", w_e1)] = (k.tag(w_e1).hex(), "2311 MTPipeKey.swift:371-372")
    if fl.tag(w_e1).hex() == "9e0c92b8f91b94880bf15eee1ca672c7":
        CONFIRMED[("walked", "A", E + 1)] = (k.walked(E + 1).hex(), "implied: the stated tag of 2311 MTPipeKey.swift:371 stands on it")
    check(G, "2311 MTPipeKey.swift:373", "floored bodyKey == origin bodyKey at (E+3)*1440",
          fl.body_key((E + 3) * MPD) == k.body_key((E + 3) * MPD), True)
    check(G, "2311 MTPipeKey.swift:374", "floored convW == origin convW on E+1", fl.conv_w(E + 1) == k.conv_w(E + 1), True)
    check(G, "2311 MTPipeKey.swift:375", "floored kept/reference/sameRank == origin's",
          fl.kept() == k.kept() and fl.reference() == k.reference() and fl.same_rank() == k.same_rank(), True)
    check(G, "2311 MTPipeKey.swift:376", "sealed form round trip and idempotent floor",
          PipeKey.from_sealed(fl.sealed_form()) == fl and fl.floored(E + 1) == fl, True)

    G = "MTPipe.agreesWithCanon"
    shared = bytes([0x44]) * 32
    owner = bytes([0x99]) * 32
    check(G, "2311 MTPipe.swift:161 | 2308 MTPipe.swift:175 | core/docs/Montana Canon.md:3063", "tag(0x44x32, 1000)",
          tag_of(shared, 1000), bytes.fromhex("1d99467017fe1d0cc7c7ba29df6fd1b6"))
    check(G, "2311 MTPipe.swift:162 | 2308 MTPipe.swift:176 | core/docs/Montana Canon.md:3064", "tag(0x44x32, 1001), unwalked",
          tag_of(shared, 1001), bytes.fromhex("c146445f67c990e25256dd88dfb39a49"))
    check(G, "2311 MTPipe.swift:163 | 2308 MTPipe.swift:177 | core/docs/Montana Canon.md:3072", "tag(0x55x32, 1000)",
          tag_of(bytes([0x55]) * 32, 1000), bytes.fromhex("783b2b17dffaab016d1aca852f60ea3e"))
    check(G, "2311 MTPipe.swift:165 | 2308 MTPipe.swift:179 | core/docs/Montana Canon.md:3067", "stepLabel(0x51x32, 1000)",
          step_label(bytes([0x51]) * 32, 1000), bytes.fromhex("4b8cb5c6442a52d0750bfb4fac0ba39d"))
    check(G, "2311 MTPipe.swift:166 | 2308 MTPipe.swift:180 | core/docs/Montana Canon.md:3068", "stepLabel(0x52x32, 1000)",
          step_label(bytes([0x52]) * 32, 1000), bytes.fromhex("401d8ab71a2e2554ab30c11aab1ae97d"))
    check(G, "2311 MTPipe.swift:167 | 2308 MTPipe.swift:181 | core/docs/Montana Canon.md:3069", "stepLabel(0x51x32, 1001)",
          step_label(bytes([0x51]) * 32, 1001), bytes.fromhex("1cc3ac539a8b1d2701e489455a8b04d5"))
    check(G, "2311 MTPipe.swift:170 | 2308 MTPipe.swift:184", "relaySeal(0x99x32, 0xAAx16)",
          relay_seal(owner, bytes([0xAA]) * 16), bytes.fromhex("2c2231f087682d4daef313938a2da5e0"))
    check(G, "2311 MTPipe.swift:171 | 2308 MTPipe.swift:185", "relaySeal(0x99x32, 0xBBx16)",
          relay_seal(owner, bytes([0xBB]) * 16), bytes.fromhex("3e83b1e1610e48bfc13929a602617d79"))
    check(G, "2311 MTPipe.swift:180-181 | 2308 MTPipe.swift:194-195", "sealKey(bodySecret, 0x0102030405060708)",
          seal_key(S32, 0x0102030405060708), bytes.fromhex("e55fbc1e0f95c4fe012b476c4dd1a51c9455e24641248f4f1557ed57f8c14f35"))
    check(G, "2311 MTPipe.swift:187 | 2308 MTPipe.swift:240", "ownerRef(0x99x32, 0x44x32)",
          owner_ref(owner, shared), bytes.fromhex("f4f72dafb742470e49defbd3c2326c27"))
    check(G, "2311 MTPipe.swift:188 | 2308 MTPipe.swift:241", "ownerRef(0x99x32, 0x45x32)",
          owner_ref(owner, bytes([0x45]) * 32), bytes.fromhex("d24f63a8c1220ea9c176ec706b8a256f"))
    ring = [bytes([0x10]) * 32, bytes([0x90]) * 32]
    check(G, "2311 MTPipe.swift:190-192 | 2308 MTPipe.swift:243-245", "holder(45468cdd..., ring) == ring[1]",
          holder(bytes.fromhex("45468cdd9bcdebe6f622c46257c27fe3"), ring) == ring[1], True)
    check(G, "2311 MTPipe.swift:193 | 2308 MTPipe.swift:246", "holder(0xFFx16, ring) == ring[0]",
          holder(bytes([0xFF]) * 16, ring) == ring[0], True)

    G = "MontanaWakePush.agreesWithCanon"
    check(G, "2311 MontanaWakePush.swift:816-817 | 2308 MontanaWakePush.swift:816-817", 'subId("mt-conv-Q7", "mtAddrZ93kLmNoPq")',
          sub_id("mt-conv-Q7", "mtAddrZ93kLmNoPq"), "e5438bfedc8b1fdcdadfb9f4d3995fac7722c0074f3a20ccd3e911ce5beb35de")
    check(G, "2311 MontanaWakePush.swift:818-819 | 2308 same lines", 'subId("ab", "c")', sub_id("ab", "c"),
          "a279974b24ff1b84299330076b5f5858684d56733458de21b152aa8521c7a230")
    check(G, "2311 MontanaWakePush.swift:820-821 | 2308 same lines", 'subId("a", "bc")', sub_id("a", "bc"),
          "c04b7d58e4c7aaca35800d119fd6fde021f95f7d3c9438e3ca46cd855afe6a8a")
    check(G, "2311 MontanaWakePush.swift:822-823 | 2308 MontanaWakePush.swift:822-823", "convTag(0..31, 29737), unwalked",
          conv_tag(S32, 29737), "43eaf2811da7947166d4e851e52c4ab89e09c8c92247370d3c734ef08ca1e6ea")
    check(G, "2311 MontanaWakePush.swift:824-825 | 2308 MontanaWakePush.swift:824-825", "convTag(0..31, 29738), unwalked",
          conv_tag(S32, 29738), "f5f2941435d76c0a80dd229722ef42382695e88fdd1fabf89d71f317bef05430")

    G = "MontanaTests (XCTest)"
    s41, s42 = bytes([0x41]) * 32, bytes([0x42]) * 32
    fp = fingerprint_of_secret(s41)
    check(G, "2311 MontanaTests.swift:57-58 | 2308 MontanaTests.swift:57-58", "fingerprint(0x41x32)", fp,
          "237630090224408202482973514653")
    check(G, "2311 MontanaTests.swift:59 | 2308 MontanaTests.swift:59", "fingerprint(0x42x32)", fingerprint_of_secret(s42),
          "600510040272855766213192827488")
    check(G, "2311 MontanaTests.swift:62 | 2308 MontanaTests.swift:62", "fingerprint(31 bytes) == nil",
          fingerprint_of_secret(bytes([0x41]) * 31), None)
    check(G, "2311 MontanaTests.swift:64-65 | 2308 MontanaTests.swift:64-65 (MTPipeKept.of)", "fingerprint(material: kept)",
          fingerprint_digits(PipeKey(s41).kept().fingerprint) == fp
          and fingerprint_digits(PipeKey(s42).kept().fingerprint) == fingerprint_of_secret(s42), True)
    check(G, "2311 MontanaTests.swift:127-129 | 2308 MontanaTests.swift:127-129", "tag(0x44x32, 1000)",
          tag_of(bytes([0x44]) * 32, 1000).hex(), "1d99467017fe1d0cc7c7ba29df6fd1b6")
    ra = PipeKey(bytes([0x21]) * 32).reference()
    check(G, "2311 MontanaTests.swift:148-152 | 2308 MontanaTests.swift:145-153 (MTPipeBook.reference)",
          "reference: 32 hex, two secrets two names, 31 bytes nil",
          ra is not None and len(ra) == 32 and ra != PipeKey(bytes([0x22]) * 32).reference()
          and PipeKey(bytes([0x21]) * 31).reference() is None, True)
    kept = PipeKey(S32).kept()
    check(G, "2311 MontanaTests.swift:160 | 2308 MontanaTests.swift:155-167 compared to convRoot", "kept.root", kept.root,
          bytes.fromhex("d9c3e7f533da2d0ed10b6729601c478ab29d1b0cafed60443113ee6a63824e11"), pin=("kept.root", "A"))
    check(G, "2311 MontanaTests.swift:161", "kept.fingerprint == fingerprintMaterial(secret)", kept.fingerprint,
          fingerprint_material(S32))
    check(G, "2311 MontanaTests.swift:162 | 2308 compared to pipeFaceBid", "kept.faceBid", kept.face_bid,
          "cbcc3fb667e739f6787aafba0f00e233642b9b0dfeba9f00a0c572588a8afb12", pin=("kept.face_bid", "A"))
    check(G, "2311 MontanaTests.swift:163 | 2308 compared to pipeFaceKey", "kept.faceKey", kept.face_key,
          bytes.fromhex("0e092e31844c94bfbd6e28fd41126c1be4515bc5ee9c60ebc7a5b232da51842a"), pin=("kept.face_key", "A"))
    check(G, "2311 MontanaTests.swift:164 | 2308 compared to MTCoinTx.pairTag", "kept.pair", kept.pair,
          bytes.fromhex("8e1d2e90a6f5d72fd89f56f2a8a8a8f801608689990466545a4d0e8d62fa586f"), pin=("kept.pair", "A"))
    check(G, "2311 MontanaTests.swift:165 | 2308 compared to MTPipeBook.reference", "kept.ref", kept.ref,
          "630dcd2966c4336691125448bbb25b4f", pin=("kept.ref", "A"))
    check(G, "2311 MontanaTests.swift:166", "kept != kept(reversed secret)", kept != PipeKey(S32[::-1]).kept(), True)

    G = "Audit/App/COIN-TX-V3.md (cited by 2311 MTWalletCore.swift:4479)"
    frm, blind_in = bytes(range(32)), bytes(range(0x20, 0x40))
    check(G, "Audit/App/COIN-TX-V3.md:329-330", "V1 commitment(from 00..1f, blind 20..3f)", coin_commitment(frm, blind_in),
          bytes.fromhex("b08fb9c67b693f0cfe394de25474d2b79ae990a87a2f6a24bb7727560a8d88a4"))
    check(G, "Audit/App/COIN-TX-V3.md:331 (negative)", "V1 refused: blind hashed before from",
          sha256(b"mt-coin-to" + blind_in + frm),
          bytes.fromhex("8d9016dfed6ea8b2c6c03daaf943afe73dfc25ed46a4bc9049bf5afe15de9c3f"))
    sec40, nonce60 = bytes(range(0x40, 0x60)), bytes(range(0x60, 0x70))
    check(G, "Audit/App/COIN-TX-V3.md:333-334", "V2 blind(secret 40..5f, nonce 60..6f)", coin_blind(sec40, nonce60),
          bytes.fromhex("530efcc186fe26cb48fb4023c7a7e884c94516d14e62feaa06e81c76f89d7be4"))
    check(G, "Audit/App/COIN-TX-V3.md:335 (negative)", "V2 refused: key and message swapped",
          hmac.new(b"mt-coin-blind" + nonce60, sec40, hashlib.sha256).digest(),
          bytes.fromhex("c9791acc18552bdd84114470faa6aa038af2f0ab245b4bfdb0ea9cd2e93ceb16"))
    check(G, "Audit/App/COIN-TX-V3.md:336-337", "V2r root(secret 40..5f)", PipeKey.root(sec40),
          bytes.fromhex("191ebd35309f514aa4c9c26b9c7eacbb3398997ee055e2776f9c1c6f40463334"))
    check(G, "Audit/App/COIN-TX-V3.md:337-338", "V2r blind under the root (= MTPipeKey.blind past the anchor)",
          PipeKey(sec40).blind(nonce60, A + 1),
          bytes.fromhex("e276ac263988cf1c47f8786fa4c39b5e19a77f0e7682d078baad82498b162c34"))
    check(G, "Audit/App/COIN-TX-V3.md:338-341 (negative)", "V2r refused: root without the zero separator (secret 00..1f)",
          sha256(b"mt-conv-root" + S32), bytes.fromhex("3be774152000edefa3660fe4cb309a46cae72881bec79e34f6dd153d5beea92a"))
    check(G, "Audit/App/COIN-TX-V3.md:340-341", "V2r: the anchor day blinds under the established secret (V2's value)",
          PipeKey(sec40).blind(nonce60, A),
          bytes.fromhex("530efcc186fe26cb48fb4023c7a7e884c94516d14e62feaa06e81c76f89d7be4"))

    # The Canon section the code cites by name (2311 MTPipeKey.swift:42-43: «The secret a tag stands on walks with the
    # window»; :24 «Invariants of the walk»). In this tree it stands only in the Protocol-13 copy of the Canon, as a WINDOW walk
    # from the first letter's window W0; iOS walks by UTC day from one anchor. The step itself is the same primitive.
    G = "Canon walk vectors (Audit/Protocol-13/source/docs/Montana Canon.md, window walk)"
    C = "Audit/Protocol-13/source/docs/Montana Canon.md:"
    ratchet = lambda x: domained("mt-pipe-ratchet", [x])   # Canon :1344
    p1 = ratchet(shared)
    p2 = ratchet(p1)
    check(G, C + "3215-3216", "tag(0x44x32, W=1000)", tag_of(shared, 1000), bytes.fromhex("1d99467017fe1d0cc7c7ba29df6fd1b6"))
    check(G, C + "3217", "pipe_secret(W+1)", p1, bytes.fromhex("1defe2877d6630f056ebaff2b6b94cd31baeb24747346b542ff5f31fe35db94d"))
    check(G, C + "3218", "pipe_secret(W+2)", p2, bytes.fromhex("a84614faf98fddb2d569aaf7a8bd3d6ff8b20fb5c22e72338af4558993176798"))
    check(G, C + "3219", "tag(W=1001) over pipe_secret(W+1)", tag_of(p1, 1001), bytes.fromhex("5bcdd791565a49a2764157d72a8450dc"))
    check(G, C + "3220", "tag(W=1002) over pipe_secret(W+2)", tag_of(p2, 1002), bytes.fromhex("7d15bb090586a75f651ab429b86e62f4"))
    ts = sha256(b"mt-vector-pipe-secret")
    check(G, C + "3227-3228", 'test_secret = SHA-256("mt-vector-pipe-secret")', ts,
          bytes.fromhex("a225e0b87df8a5568331badccfa4b217aaee3c7077b6021a52fab89d8856ef56"))
    check(G, C + "3229", "pipe_secret(test_secret, W+1)", ratchet(ts),
          bytes.fromhex("92aa039f4d82a59e12a6e65eacaa998fa06455b63e4aef6405556f9e99e4c666"))
    check(G, C + "3230", "tag(test_secret, W)", tag_of(ts, 1000), bytes.fromhex("42b13c7133439d4222c9bdf52b5f27b0"))
    check(G, C + "3231", "tag(pipe_secret(test_secret, W+1), W+1)", tag_of(ratchet(ts), 1001),
          bytes.fromhex("39e4abcd4db155761253830a3afa9959"))
    check(G, C + "3232 (pipe_key: " + C + "2142,2162)", "pipe_key(pipe_secret(test_secret, W+1), W+1)", seal_key(ratchet(ts), 1001),
          bytes.fromhex("eed13b68a4fecd9c91ff7ddca945f8289b62b02835f95260639616d2ba236ba8"))
    check(G, C + "3237 (negative)", "tag(unwalked secret, W=1001)", tag_of(shared, 1001),
          bytes.fromhex("c146445f67c990e25256dd88dfb39a49"))
    walked_1001 = shared
    for _ in range(1001):
        walked_1001 = ratchet(walked_1001)
    check(G, C + "3238 (negative)", "tag(walked 1001 steps, W=1001)", tag_of(walked_1001, 1001),
          bytes.fromhex("11062c7ece61d640abf601e486170994"))

    # The wrong implementations a comment names by prefix only (not asserted by the code): tried, reported, not counted.
    w_bk = (A + 2) * MPD
    negative("2311 MTPipe.swift:176 | 2308 MTPipe.swift:190", "sealKey without the domain separator", "1d1f4bf4",
             [("SHA-256('mt-pipe-key' || s || W_LE)", sha256(b"mt-pipe-key" + S32 + window_le(0x0102030405060708)))])
    negative("2311 MTPipe.swift:176 | 2308 MTPipe.swift:190", "sealKey with the window big-endian", "11178eee",
             [("SHA-256('mt-pipe-key' 00 s W_BE)", sha256(b"mt-pipe-key\x00" + S32 + (0x0102030405060708).to_bytes(8, "big")))])
    negative("2311 MTPipe.swift:177 | 2308 MTPipe.swift:191", "sealKey over the reversed secret", "c04889ba",
             [("SHA-256('mt-pipe-key' 00 rev(s) W_LE)", seal_key(S32[::-1], 0x0102030405060708))])
    negative("2311 MTPipe.swift:177 | 2308 MTPipe.swift:191", "sealKey under the label domain", "de05839d",
             [("SHA-256('mt-tag' 00 s W_LE)", domained("mt-tag", [S32, window_le(0x0102030405060708)])),
              ("SHA-256('mt-step' 00 s W_LE)", domained("mt-step", [S32, window_le(0x0102030405060708)]))])
    negative("2311 MTPipeKey.swift:308-309 | 2308 MTPipe.swift:200-201", "a walk that never steps", "a656c407",
             [("sealKey(s, (A+2)*1440)", seal_key(S32, w_bk)), ("convTag(s, A+2)", bytes.fromhex(conv_tag(S32, A + 2)))])
    negative("2311 MTPipeKey.swift:309 | 2308 MTPipe.swift:201", "a walk stepping once whatever the distance", "1110b376",
             [("sealKey(ratchet(s), (A+2)*1440)", seal_key(ratchet(S32), w_bk)),
              ("convTag(ratchet(s), A+2)", bytes.fromhex(conv_tag(ratchet(S32), A + 2)))])
    negative("2311 MTPipeKey.swift:310 | 2308 MTPipe.swift:203", "a walk step without the domain", "630dcd29",
             [("SHA-256(s)", sha256(S32))])
    negative("2311 MTPipeKey.swift:331 | 2308 MTPipe.swift:217", "a root without the zero separator", "3be77415",
             [("SHA-256('mt-conv-root' || s)", sha256(b"mt-conv-root" + S32))])
    negative("2311 MTPipeKey.swift:331 | 2308 MTPipe.swift:217", "a root under the walk's domain", "5dcfd876",
             [("SHA-256('mt-pipe-ratchet' 00 s)", ratchet(S32))])
    negative("2311 MTPipeKey.swift:357", "a tag that never walks, on Y", "cc699748",
             [("tag(s, Y*1440+5)", tag_of(S32, w_y))])


def properties():
    """What the 2311 text says, checked on this re-implementation (not vectors of the source)."""
    G = "property (this script, from the 2311 text)"
    k = PipeKey(S32)
    check(G, "2311 MTPipeKey.swift:48-51,65", "the walk is clamped at maxWalk steps",
          k.walked(A + MAX_WALK + 1000) == k.walked(A + MAX_WALK), True)
    check(G, "2311 MTPipeKey.swift:65,71", "no day before the anchor walks", k.walked(0) == S32 and k.walked(A - 1) == S32, True)
    check(G, "2311 MTPipeKey.swift:105-108", "the tag on days A+1..Y-1 still stands on the established secret",
          all(k.tag(d * MPD + 7) == tag_of(S32, d * MPD + 7) for d in range(A + 1, Y)), True)
    f60 = k.floored(E)
    f62 = k.floored(E + 2)
    check(G, "2311 MTPipeKey.swift:85-93", "a floor walked forward equals a floor made from the origin",
          f60.floored(E + 2) == f62, True)
    check(G, "2311 MTPipeKey.swift:88", "a floor never goes back", f62.floored(E) == f62, True)
    check(G, "2311 MTPipeKey.swift:276-281", "the archive head round trip of the floor",
          PipeKey.from_archive_head(f60.archive_head().decode(), E) == f60, True)
    check(G, "2311 MTPipeKey.swift:276-281", "a head holding the secret read from E is walked and erased",
          PipeKey.from_archive_head(k.archive_head().decode(), E) == f60, True)
    check(G, "2311 MTPipeKey.swift:267-271", "a book holding the secret lags from E and not before",
          lags({"c": k.sealed_form()}, E) and not lags({"c": k.sealed_form()}, E - 1)
          and not lags(stored({"c": k}, E), E), True)
    check(G, "2311 MTPipeKey.swift:166", "a floored pipe confirms nothing", f60.first_confirm(b"ct") == "", True)
    for d in (E, E + 1, E + 2):
        fd = k.floored(d)
        ok = all(fd.tag(w) == k.tag(w) and fd.body_key(w) == k.body_key(w)
                 for w in (d * MPD, d * MPD + 5, d * MPD + 1439, (d - 2) * MPD + 3, (d + 1) * MPD + 9))
        ok = ok and all(fd.conv_w(x) == k.conv_w(x) for x in range(d - 2, d + 2))   # boxDays: D-2 ... D+1 (MTNodeWire.swift:250)
        check(G, "2311 MTPipeKey.swift:83-108; MTNodeWire.swift:250", "floor of day %d derives every key of D-2..D+1" % d, ok, True)


# ── the table for the ports ──────────────────────────────────────────────────────────────────────────────────────────
B32 = bytes(range(0x20, 0x40))   # the newer pipe of an mt-same pair: this script's choice, no source vector
GROUP_MID = "g"                  # MTGroupWord.g; tagText = t, id, me, s, ti, tx joined by 0x00 (2311 MontanaGroup.swift:2120-2122)
GROUP_TEXT = "\x00".join(["post", "m1", "", "", "", "hello"])
NONCE = bytes(range(0x60, 0x70))
DAYS = [20754, 20755, 20756, 20757, 20758, 20759, 20760, 20761, 20762]
ASKED = {20754, 20757, 20758, 20759, 20760, 20762}
MINUTES = [0, 5, 720, 1439]
UNCONFIRMED = "derived, unconfirmed"
RULE = "rule (no hash; read off the source)"


def cell(s):
    """A Markdown table cell: the citations' own '|' would split it."""
    return str(s).replace("|", "/")


def status(key, value):
    c = CONFIRMED.get(key)
    if c is None:
        return UNCONFIRMED
    return ("confirmed: " + c[1]) if c[0] == hx(value) else ("CONTRADICTS " + c[1])


def utc(day, minute=0):
    t = datetime.datetime(1970, 1, 1, tzinfo=datetime.timezone.utc) + datetime.timedelta(days=day, minutes=minute)
    return t.strftime("%Y-%m-%d %H:%M UTC")


def phase(d):
    if d < A:
        return "before the anchor"
    if d == A:
        return "anchor (walk value = established secret)"
    if d < Y:
        return "walk running, old tag"
    if d < E:
        return "Y: new tag laid, old tag also accepted" if d == Y else "Y+1: new tag laid, old tag also accepted"
    return "E: floor D-2 only, old tag refused" if d == E else "after E: floor D-2 only, old tag refused"


def table():
    a, b = PipeKey(S32), PipeKey(B32)
    out = {}
    for d in DAYS:
        held_a = a.floored(d)          # what a 2311 device holds on day d (MTPipeBook.load -> MTPipeKey.book: floored on read)
        held_b = b.floored(d)
        rows = []

        def row(q, value, key=None, note="", rule=False):
            st = RULE if rule else (status(key, value) if key else UNCONFIRMED)
            rows.append({"quantity": q, "value": hx(value), "status": st, "note": note})

        row("steps from the anchor", PipeKey.steps(d), None, "2311 MTPipeKey.swift:65", rule=True)
        row("pipe_secret(day) = walked(D)", a.walked(d), ("walked", "A", d), "2311 MTPipeKey.swift:68-81")
        row("item key of the day (letter author, group author, coin blind)",
            "root" if a.item_key(d) == a.root_key() else "established secret", None, "2311 MTPipeKey.swift:126", rule=True)
        for m in MINUTES:
            w = d * MPD + m
            row("tag 2311 lays, minute %d (W=%d, %s)" % (m, w, utc(d, m)), a.tag(w), ("tag_new", "A", w), "2311 MTPipeKey.swift:105-108")
            row("old tag over the established secret, minute %d (what <=2309 lays)" % m, tag_of(S32, w), ("tag_old", "A", w),
                "2308 MTPipe.swift:47-49, 827-830")
            acc = held_a.tags(w)
            st = status(("tags", "A", w), acc) if len(acc) == 2 else status(("tag_new", "A", w), acc[0])
            if len(acc) == 1 and st.startswith("confirmed"):
                st += "; that it is the only one: rule 2311 MTPipeKey.swift:113 (the count is asserted on E*1440+5 by :365)"
            rows.append({"quantity": "tags a 2311 receiver accepts in W=%d (held as on day D)" % w, "value": hx(acc),
                         "status": st, "note": "2311 MTPipeKey.swift:109-115; index over W-1..W+1: MTPipe.swift:718-724"})
        w0 = d * MPD
        idx = []
        for kw in (w0 - 1, w0, w0 + 1):
            idx += [t.hex() for t in held_a.tags(kw)]
        row("tag index at the day's first minute (windows W-1, W, W+1)", idx, None, "2311 MTPipe.swift:718-724")
        row("conv_w(D)", a.conv_w(d), ("conv_w", "A", d), "2311 MTPipeKey.swift:100-102")
        row("conv_w over the box days D-2..D+1", [held_a.conv_w(x) for x in range(d - 2, d + 2)], None,
            "2311 MTNodeWire.swift:250; NotificationService.swift:904-905")
        for m in (0, 5, 1439):
            w = d * MPD + m
            row("body key, minute %d (W=%d)" % (m, w), a.body_key(w), ("body_key", "A", w), "2311 MTPipeKey.swift:95-98")
        lt = held_a.author_tag(PERSON, "m", "t", d)
        row('letter author tag, person 0x33x32, mid "m", text "t"', lt, None, "2311 MTPipeKey.swift:141-144; MTNodeWire.swift:139-144")
        row("withAuthor(empty body) bytes", held_a.with_author(b"", PERSON, "m", "t", d), None, "2311 MTNodeWire.swift:147-155")
        row("letter author tag a 2308 build lays on day D (itemKey: 2308 MTNodeWire.swift:84,179)",
            author_tag(PERSON, a.root_key() if d > A else S32, "m", "t"), None, "same as 2311 on every day")
        gt = held_a.author_tag(PERSON, GROUP_MID, GROUP_TEXT, d)
        row("group author tag (ownTag), mid %r, tagText %r" % (GROUP_MID, GROUP_TEXT), gt, None,
            "2311 MontanaGroup.swift:2124-2127 -> MTPipeKey.swift:159-161")
        g08 = author_tag(PERSON, S32, GROUP_MID, GROUP_TEXT)
        row("group author tag a 2308 build lays (over the established secret, every day)", g08, None,
            "2308 MontanaGroup.swift:2124-2127")
        row("2311 reader takes the 2308 group tag as own (authorHolds)", held_a.author_holds(g08, PERSON, GROUP_MID, GROUP_TEXT),
            None, "2311 MTPipeKey.swift:129,147-155")
        row("keys a 2311 reader tries for an author field", ["root", "established secret"] if held_a.holds_origin else ["root"],
            None, "2311 MTPipeKey.swift:129", rule=True)
        for dom in ("mt-same-ask", "mt-same-yes"):
            row("%s: older A beside newer B, what 2311 lays" % dom, held_a.same_tag(dom, held_b, d), None,
                "2311 MTPipeKey.swift:169-172; MontanaChatStore.swift:7584,7616")
            row("%s: what 2311 accepts" % dom, held_a.same_tags(dom, held_b, d), None,
                "2311 MTPipeKey.swift:174-178; MontanaChatStore.swift:7600,7626")
            row("%s: what 2308 lays (over the two secrets)" % dom, PipeKey.same_origin(dom, a, b), None,
                "2308 MontanaChatStore.swift:7532-7534,7586,7618")
        row("mt-same-rank A (kept for good, over the established secret)", held_a.same_rank(), ("rank", "A"),
            "2311 MTPipeKey.swift:185 | 2308 MontanaChatStore.swift:7541")
        row("mt-same-rank B", held_b.same_rank(), None, "same")
        row("rank A precedes rank B", held_a.same_rank() < held_b.same_rank(), None, "2311 MontanaChatStore.swift:7604-7605")
        row("coin SEND blind, nonce 60..6f (extra)", held_a.blind(NONCE, d), None, "2311 MTPipeKey.swift:133-135")
        if d >= E:
            f = held_a.floor
            row("floor day F = D-2", f.day, None, "2311 MTPipeKey.swift:87", rule=True)
            row("floor value walk(F)", f.value, ("walked", "A", f.day), "2311 MTPipeKey.swift:91")
            row("floor record, 249 bytes", held_a.sealed_form(), ("floor_form", "A", d), "2311 MTPipeKey.swift:233-244")
            row("SHA-256 of the floor record", sha256(held_a.sealed_form()), ("floor_form_sha256", "A", d), "2311 MTPipeKey.swift:370")
            row("archive head (s: + base64 of the record)", held_a.archive_head().decode(), None, "2311 MTPipeKey.swift:275")
        else:
            row("floor record", "none: 2311 keeps the 32 established bytes before E (floored returns self)", None,
                "2311 MTPipeKey.swift:86", rule=True)
        out[d] = {"date": utc(d), "phase": phase(d), "asked": d in ASKED, "rows": rows}
    return out


def kept_rows():
    k = PipeKey(S32).kept()
    return [
        {"quantity": "root", "value": k.root.hex(), "status": status(("kept.root", "A"), k.root)},
        {"quantity": "fingerprint material", "value": k.fingerprint.hex(),
         "status": status(("kept.fingerprint", "A"), k.fingerprint)},
        {"quantity": "face name (faceBid)", "value": k.face_bid, "status": status(("kept.face_bid", "A"), k.face_bid)},
        {"quantity": "face key", "value": k.face_key.hex(), "status": status(("kept.face_key", "A"), k.face_key)},
        {"quantity": "pair tag", "value": k.pair.hex(), "status": status(("kept.pair", "A"), k.pair)},
        {"quantity": "reference", "value": k.ref, "status": status(("kept.ref", "A"), k.ref)},
        {"quantity": "mt-same-rank", "value": PipeKey(S32).same_rank().hex(),
         "status": status(("rank", "A"), PipeKey(S32).same_rank())},
    ]


def main(argv):
    selfchecks()
    properties()
    tab = table()
    failed = [r for r in RESULTS if not r[3]]
    groups = {}
    for g, cite, what, ok, got in RESULTS:
        groups.setdefault(g, [0, 0])
        groups[g][0 if ok else 1] += 1
    print("# iOS 2311 pipe vectors (source c7e11da / 648c3225b9ae; 2308 = 59d30f3)")
    print()
    for g, (ok, bad) in groups.items():
        print("- %s: %d reproduced, %d failed" % (g, ok, bad))
    print("- negatives named by prefix only: %d of %d reproduced by the interpretation tried" %
          (sum(1 for n in NEGATIVES if n[3]), len(NEGATIVES)))
    for cite, what, prefix, hit in NEGATIVES:
        print("    %s %s... %s [%s]" % ("ok " if hit else "?? ", prefix, what, hit or "no interpretation tried reproduces it"))
    if failed:
        print()
        print("FAILED:")
        for g, cite, what, ok, got in failed:
            print("  [%s] %s -- %s: got %s" % (g, cite, what, got))
    print()
    print("Test secrets: A = bytes 00..1f (the source's bodySecret, 2311 MTPipeKey.swift:314); B = bytes 20..3f (newer pipe")
    print("of an mt-same pair, this script's choice); person key = 0x33 x 32 (2311 MTPipeKey.swift:342); coin nonce 60..6f.")
    print()
    print("## Kept for good (pipe A)")
    print("| quantity | value | status |")
    print("|---|---|---|")
    for r in kept_rows():
        print("| %s | `%s` | %s |" % (cell(r["quantity"]), r["value"], cell(r["status"])))
    for d, blk in tab.items():
        print()
        print("## Day %d (%s) -- %s%s" % (d, blk["date"], blk["phase"], "" if blk["asked"] else " [extra day: the source's own checks]"))
        print("| quantity | value | status | where |")
        print("|---|---|---|---|")
        for r in blk["rows"]:
            v = r["value"]
            v = ", ".join(v) if isinstance(v, list) else v
            print("| %s | `%s` | %s | %s |" % (cell(r["quantity"]), v, cell(r["status"]), cell(r["note"])))
    if "--json" in argv:
        path = argv[argv.index("--json") + 1]
        with open(path, "w", encoding="utf-8", newline="\n") as fh:
            json.dump({"source": {"2311": "c7e11da (iOS source 648c3225b9ae)", "2308": "59d30f3"},
                       "constants": {"walkAnchorDay": A, "tagWalkDay": Y, "eraseDay": E, "floorLag": FLOOR_LAG,
                                     "maxWalk": MAX_WALK, "minutesPerDay": MPD, "floorFormSize": FLOOR_FORM_SIZE},
                       "secrets": {"A": S32.hex(), "B": B32.hex(), "person": PERSON.hex(), "nonce": NONCE.hex(),
                                   "group_mid": GROUP_MID, "group_text_hex": GROUP_TEXT.encode().hex()},
                       "checks": [{"group": g, "cite": c, "what": w, "ok": ok, "got": got} for g, c, w, ok, got in RESULTS],
                       "negatives": [{"cite": c, "what": w, "prefix": p, "reproduced_by": h} for c, w, p, h in NEGATIVES],
                       "kept": kept_rows(), "days": {str(d): blk for d, blk in tab.items()}}, fh, indent=1)
    return 0 if not failed else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

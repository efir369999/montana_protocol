# Montana for Windows — Work in Progress

**Status: Work in Progress · Partial Implementation · Not a release.** State as of 28.10.2026 18:00 UTC.
Do not use it with an identity that holds anything of value. The installer built from this tree is a local build, unsigned,
and not an official release of Montana.

## What it is

A port of Montana for iPhone (`apps/messenger/ios`, the reference app) to Windows 10/11 as a [Tauri 2](https://tauri.app)
program: Rust underneath, the interface in React. The goal is a copy of the iOS app as exact as Windows allows, speaking the
same wire with iPhones, Android phones and other Windows computers.

The core is not reimplemented. `mt_bindings.dll` is built from `core/Code` (crate `mt-bindings`, the C ABI iOS links as
`MontanaBindings.xcframework`) and `montana_core.dll` from `core/Montana-Core` (crate `mt-boundary`, unchanged, the boundary
iOS links as `MontanaCore.xcframework`); both are called through their C ABI. The 24 words and the keys stay inside the core
and the Rust side and never reach the web view.

Porting and adaptation to Windows: **DinetPDA — Linkora**. Montana itself — the protocol, the core, the iOS app, its design and
its assets — is the work of its own authors; this port follows it and claims none of it. See [`CREDITS.md`](CREDITS.md).

## What is in this folder

| Path | What it is | Lines |
|---|---|---|
| `app/src-tauri/src/` | the Windows side in Rust, 67 files: identity, pipes and letters, the pipe's one owner and its floor (`pipekey`, iOS 2311 `MTPipeKey`), the rules beyond iOS 2311 and the clock guard (`rules`, `nodeclock`), the twin mirror, archive and copies, groups, channels, calls' signalling, coins and the wallet, music, the VPN wall's server book, the home node, Windows Hello, notifications | 38 780 |
| `app/src/` | the interface (React 19 + Vite 8), drawn after the iOS screens | 20 881 |
| `app/protocol-core/` | `montana_core.dll`: the boundary crate `core/Montana-Core/crates/mt-boundary`, unchanged, as one Windows library | 64 |
| `app/src-tauri/node/` | the home node the app installs on a machine one owns: iOS's `Code/Node` (deviation 12) | 1 250 |
| `app/tools/` | 106 UI scenes (PowerShell + UI Automation) that drive two or more real windows with test identities, a manual check of Windows Hello, their helpers, the runner `all.ps1` and the protocol core's build script; `chess-check.mjs`; `pagekeys.mjs`, which fails two pages of one kind that can take one place without keys of their own; `clocknode.py`, an isolated node on a clock of its own; `vectors_2311.py`, the iOS 2311 pipe formulas re-derived with Python's standard library alone, and `pipekey_days.py`, which turns its rows into `src-tauri/src/pipekey/days.rs`; the generators `glyphs.py` and `ui_atlas.py` | 12 223 |

This tree is the developer's local work at its commit `7a45c11` (28.10.2026 19:06 UTC), and its scenes at `ed2147b`, copied in as files; that history is not
published. Edits made for this repository, as for PR #13 and PR #45: the path of the core in `app/protocol-core/Cargo.toml`
(`../../../../../core/...`); LF line endings in `app/src/glyphs.js` and in the scenes `shot.ps1`, `offchat.ps1`,
`offident.ps1`, `offtwin.ps1`, `twinname.ps1` and `upgrade.ps1`; in those five new scenes, their header comments, which drop references to the
developer's unpublished working notes; and a wider `app/.gitignore`, which also keeps out `*.words` and `*.mtbak`, the files of
an identity's 24 words and of its copy (#40); and three files, `app/src-tauri/src/book.rs`, `rules.rs` and `history.rs`, leave
out local work that does not run in the program's defaults and is not part of this publication. Every other file of `app/` is
byte for byte the local one. Since the tree of PR #45 (local `aacbf76`) it has taken the port of the iOS 2311 pipe and its
floor with the rules of decision 188, the attachments tray, drag-and-drop, the Time Coin of presence (iOS 2314), a name off
the live road, and their scenes (the next sections and Checks).

## State per feature

*works*: a scene or unit test passed, with its date; *partial*: present, with the gaps named; *prototype*: a first form that
lacks an essential part. A scene named with no date is one of the 75 of `tools\all.ps1`: all of them passed in the final full
run of 26.10.2026 on the program code of the tree before this one (local `abdabb2`; 73 at once, `deeds` and `voice` on a
re-run), and **they have not been re-run on this tree's program code**. "28.10.2026" marks a run on this tree's program code
(build 4, Checks). "earlier code" marks a run of a scene outside `all.ps1` before the chats-tab work.

| Feature | State | Evidence | Main gaps |
|---|---|---|---|
| Identity: the 24 words, restoring by them, several identities on one computer, deleting one | works; interface partial | `seats`; `leave` (25.10.2026, earlier code); `upgrade` (28.10.2026: restored by the words into a fresh folder, the old build's copy laid back); unit tests in `history`, `twin`, `forgotten` | no Back while an identity is made; no restore page with progress; a new identity's sign-in is not a page over the tabs |
| Meeting a person, contacts, a person's page | partial | `ab` | no fingerprint verification (deviation 3); Contacts is a bare list; no @name sheet; a person's page lacks its call buttons; no MTSamePair, so one person can stand in two rows (deviation 4) |
| The pipe secret, its daily step and the floor (iOS 2310's transition, ported from iOS 2311's source) | ported; not yet on the real dates | unit tests in `pipekey` (iOS 2311's own checks and the rows of `tools/vectors_2311.py`), `wire::canon_walk`, `book`, `nodeclock`, `post`; `upgrade` (28.10.2026: the days moved +8 and +12 in a check's run, two Windows identities) | departs from Canon (deviation 1); rules beyond iOS 2311 by decision 188 (deviation 2); never run across the floor with an official iPhone or Android; not yet run on the real dates, 29.10, 01.11 and 03.11.2026 UTC |
| Letters one to one, Saved Messages, drafts, forwarding, selection | works; interface partial | `ab` (28.10.2026), `savedpick`, `select`, `drafts`, `forward`; `savedtwin` (24.10.2026, earlier code) | no search inside a conversation, no jump to the newest, no floating date; an unsent letter is not offered again; a poorer hold menu; no Send Later |
| One identity on several devices (iOS's twin mirror) | works between Windows and the Android port on emulators | `twin*.ps1` (25.10.2026, earlier code: that of PR #13; table below) | never checked against an iPhone; a two-minute wait of its own (deviation 14) |
| Groups | partial | `groups`, `gtyping`, `gface`, `glink`, `grights` | the group's page is flat |
| Channels | partial | `channel`, `ccomments` | the channel's page is flat |
| Calls: audio, video, rooms | partial | `calls`, `callbreak`, `callvideo`, `room` and the other call scenes | no ringing sounds; the video call is laid out differently; few ways into a call; the call's row in a conversation differs |
| Files and media | partial | the attachments tray and drag-and-drop: `tray`, `drag`, `dropin` (28.10.2026); `files`, `album`, `gfiles` (28.10.2026), `bigfile`, `heic`, `gif`, `voice`, `note`, `stickers` | iOS 2315's other two changes (a channel's sent file after reopening; the Files page's last folder) not ported; photos sent together are not laid as one album mosaic; the viewer's gestures and menus in part; video notes cut down; the emoji panel simplified; the Gallery has no viewer or menu |
| Notifications | partial | Windows toasts (`tauri-plugin-notification`); the tray keeps the program hearing letters with its window closed | no push service: letters are heard only while the program runs (deviation 9); a tap on a banner does nothing yet |
| The node: the doors, the box, the home node | partial | unit tests in `doors`, `homenode`; `homenode`, `core`, `p2p` | no Bluetooth mesh (deviation 11); the box read bounded by the floor (`post.rs` 152-158) is unit-tested only; box letters on moved days not checked |
| Wallet and coins | partial | `walletlook`, `coins`, `coinhold`, `coinreact`, `flow`, `seedbook`, `seatsend`; transfers checked only between test identities | sending from a group; the Transaction page; a call's receipt; the Explorer opens in the system browser (deviation 10) |
| Settings and the copy (`.mtbak`) | partial; the freeze fixed | `slotpage`: Settings from the drawer, each of its pages and an answer of Questions left by Back, Esc, Alt+Left, the mouse's back button and the edge stroke, five times in a row, and the slot's own pages left by their cross, the window answering after each; `datapage`, `keeping`; unit tests in `history` (a cut, forged or reordered copy is refused) | parts of Appearance; the copy's pages simplified; the waiting rows are a Windows adaptation (deviation 15) |
| Wall, music, chess, card, passwords | partial | `music`, `library`, `wall`, `chess`, `chesscpu`, `card`, `passwords`; `node tools/chess-check.mjs` 40 of 40 on the tree first published | the chess board's Moves sheet, Flip board, Live position and the timer of a game on one device; the wall's common feed; posts' media and links |
| VPN wall | prototype | `vpn`: the server book and the subscriptions | no tunnel: the engine is not in this port (deviation 11) |
| Languages | partial | Russian and English, keyed as iOS's catalog (`app/src/strings.js` 1-3) | iOS's catalog also has Simplified Chinese |
| The interface against iOS | partial | the atlas below | compared visually with the SwiftUI source on 22 rows; none on an iPhone; no animation compared |
| Build and installer | builds; unsigned | `npx tauri build` exit 0 on this tree's program code (`dc7689a`, build 4, 28.10.2026): the program and `Montana_0.1.0_x64-setup.exe`, hashes in Checks | the NSIS installer is not code-signed; a local build, not an official release |
| Dinet Device Sync | design only | — | no code (`CREDITS.md`) |

### The interface, row by row

The developer keeps an atlas of the iOS interface (iOS 2305): 1 501 rows — screens, pages, sheets, alerts, menus, actions,
transitions, states — each with its state on Windows, counted by `app/tools/ui_atlas.py`. As the atlas stands at this tree's
commit (the atlas itself is held back, see the last section; it has not been brought up to date with this tree's tray):

| Section | Rows | Code on Windows: yes / partial / missing | Functionally verified by a scene |
|---|---|---|---|
| Launch, sign-in, identities | 116 | 63 / 33 / 16 | 23 |
| Main screen and the chats tab | 150 | 112 / 26 / 8 | 5 |
| An open conversation | 190 | 72 / 62 / 46 | 5 |
| Media and attachments | 165 | 36 / 60 / 62 | 5 |
| Calls | 143 | 58 / 45 / 31 | 0 |
| People and communities | 215 | 61 / 88 / 59 | 2 |
| Wall, music, chess, card, passwords | 257 | 132 / 75 / 45 | 0 |
| TimeCoin, settings, network | 255 | 142 / 73 / 31 | 2 |
| Windows and Android add-ons (Dinet Device Sync) | 10 | 0 / 2 / 8 | 0 |
| **All** (besides: 53 rows not applicable to Windows, 2 unclear) | **1 501** | **676 / 464 / 306** | **42** |

Compared visually: 22 rows from the SwiftUI source, none against a Simulator shot or an iPhone. Animations compared: none.

### One identity on several devices (iOS's twin mirror)

Run with Windows and the Android port on **emulators**, test identities and the live nodes, on 25.10.2026 on the code of PR #13
(before 08:51 UTC) — **not on a real phone, not against an iPhone, not re-run on this tree**. The Android side is the Windows
checks' counterpart, published apart (`CREDITS.md`).

| Scenario | Scene | Result |
|---|---|---|
| own letters both ways; a correspondent's letter taken by one device | `twin3.ps1` | 4 of 4 |
| a device away for an hour | `twinhour.ps1` | 40 of 40 |
| both on line take the same letters | `twinrace.ps1` | 10 of 10 on each; a mirror word heard twice laid once |
| both write at once | `twinboth.ps1` | 11 of 11 on each, 10 of 10 at the correspondent |
| written offline, app killed, network back | `twinoffline.ps1` | 3 of 3 everywhere |
| the mirror's road cut the instant it is fetched | `twincut.ps1` | the row stayed in the box and was taken 15 s after the network came back, once |
| both apps restarted; a wiped phone opened by the same words | `twinrestart.ps1`, `twinrestore.ps1` | the same list word for word; 6 conversations and 47 rows from the copy, no letter twice |
| three devices of one identity | `twinthree.ps1 -Strict` | every letter on both Androids since the port of iOS 2306 (08:2x UTC) |
| groups; rows asked back (`TA:`/`TW:`); cards (`TC:`); a row the mirror never carried | `twingroup.ps1`, `twinask.ps1`, `twincard.ps1`, `twingap.ps1` | passed |
| two Windows computers of one identity: own letters, the reading, deletion for all | `twinwin.ps1` | reading on one took the other's unread off in about 13 s; each letter once on both |
| the pipe secret's step at 29.10.2026 00:00 UTC | `walk.ps1`, `twin3.ps1` | **not yet run** on the real date |

### Following the iOS builds (the protocol reference)

The interface is compared with iOS 2305 screen by screen; the protocol follows the iOS builds as their source is published.

| iOS build | Change | Windows |
|---|---|---|
| 2306 | each device takes a letter on its own: `/fetch` names the device per label (`devs`), `/box-del {"taken": [[mid, device]]}` | **ported**; the live nodes answer `taken`; the three-device scenario passes |
| 2307 | a call packet goes to every door at once | **ported** |
| 2308 | the ring of a call that ended is answered «delivered» | **ported** |
| 2309 | one owner of the conversation secret (wire unchanged) | **ported** with 2311's `MTPipeKey` (`app/src-tauri/src/pipekey.rs`) |
| 2310 | the tag on the walked secret from 01.11.2026, both tags accepted on 01-02.11, only the new one from 03.11.2026; the floor F = D - 2 and erasure of the established secret | **ported from 2311's source**, with the rules of decision 188 on top (deviation 2); unit-tested and run on moved days, not yet on the real dates |
| 2311 | its source is in `main` (c7e11da, source 648c3225b9ae, 25.10.2026 12:48 UTC): the base the 2310 transition is ported from; its visible change, the order of a long message's pieces | the pipe: **ported**; the pieces' order not compared |
| 2312 | several files chosen at once wait above the field numbered 1, 2, 3, a cross takes one out, they leave one after another in that order | **ported**: the attachments tray (`tray`, `files`, `album`, `gfiles`); a set of files counted as one message under a group's slow mode not checked |
| 2313 | the first floor waits up to four days for the day the nodes confirm (a clock running ahead) | this port's own guard of #38 (decision 188, `MONTANA_CLOCK_GUARD`, on by default); not compared with 2313's source line by line. 2313's `archive_head` trace line is written |
| 2314 | the Time Coins page's balance rising with the seconds on their way | **ported** (`wallet.rs` `minting`, `growing`; unit tests); not run on a real wallet, not compared with an iPhone |
| 2315 | a plate of that row held and carried to another place, the numbers following; a channel's sent file after reopening; the Files page's last folder | the carry: **ported** (`drag`; files dropped from Explorer, `dropin`); the other two not yet |

The transition is checked in a check's own folders before anything is claimed, never on a real wallet or a real identity:
`app/tools/clocknode.py` runs the original's `apps/messenger/ios/Code/Node/montana-home-node.py` on a clock of its own on the
loopback, with the live box's 2306 behaviour added; a check's run of Windows is pointed at it by `MONTANA_TEST_DOOR` and counts
its day ahead by `MONTANA_CLOCK_SHIFT_DAYS` (both read only when `MONTANA_DATA_DIR` is set); `app/tools/walkiso.ps1` drives
Windows and an emulator through it from today to 03.11.2026. `app/tools/upgrade.ps1` moves the days of two Windows test
identities on the live nodes the same way (Checks).

## New in this tree: the floor, the attachments tray, drag-and-drop, the Time Coin of presence, names off the live road

**The pipe's one owner and its floor.** `app/src-tauri/src/pipekey.rs` ports iOS 2311's `MTPipeKey` (`main` c7e11da): one
owner of a pipe's secret that hands no byte of it out; the walk by the UTC day from 28.10.2026; the tag of a window on the
walked value from 01.11.2026 UTC, the earlier tag still heard on 01-02.11; from 03.11.2026 UTC the floor F = D - 2 in its
249-byte form and the established secret erased; a box read bounded below by the floor (`post.rs` 152-158; before the floor
this port reads the days D-8 .. D+1 where iOS `boxDays` reads D-2 .. D+1, `post.rs` 117). Its tests hold iOS 2311's own
checks and the rows of `app/tools/vectors_2311.py`, which re-derives the 2311 formulas with Python's standard library apart
from this code (`pipekey/tests.rs`, `pipekey/days.rs`, made by `tools/pipekey_days.py`).

**The rules beyond iOS 2311** (`app/src-tauri/src/rules.rs` 1-16), each behind a switch of its own read from the environment
(`1` on, `0` off):

| Switch | Default | What it does when on (off: iOS 2311 exactly) |
|---|---|---|
| `MONTANA_D1_KEEP_CONFIRM` | on (#36) | a first letter's confirmation kept beside its ciphertext from the pipe's birth and carried after the floor with that ciphertext alone |
| `MONTANA_D3_SAME_PIPE` | on (#37) | a pipe met again in another form is the same pipe when both, walked to the later floor day, give the same whole floor |
| `MONTANA_CLOCK_GUARD` | on (#38) | the first floor waits while the device's day stands more than a day past the day a node's Date confirmed, at most four days by either count (`nodeclock.rs`; the shared cases `pipekey/clock-guard-cases.json`); the book walks to the day on its first read of each new day |

All three follow decision 188 of Montana's author (25.10.2026) with the chairman's rulings on #36, #37 and #38. This is a
port that follows the published iOS source and that decision: it is **not a specification** and changes no document of one. By the code's own account (`rules.rs` 15-16) no
switch changes a byte of the wire but the book's walk at 03.11 00:00 UTC (the window of 02.11 23:59 then hears only the walked
tag). Where the port departs from Canon, see deviation 1.

**The attachments tray** (`app/src/Conversation.jsx`, `app/src/chats.css`; iOS 2312 and 2315 `pastedPreview`,
`MTStagedOrder`). What the picker, a paste or a drop hands waits above the field as numbered plates; the cross takes one out
and the numbers close up; `+` adds one more; a plate held by the mouse takes the place of the one it enters and the numbers
follow; the arrow sends the tray once, in the order it stands, the caption with the first; a file that did not leave comes
back to the tray's head in its order. **Drag-and-drop:** the window's own drop handler is off (`app/src-tauri/tauri.conf.json`,
`"dragDropEnabled": false`), so the page receives HTML5 drag: plates are carried inside the tray, and files dropped from
Windows Explorer stand in the tray and leave only by the arrow.

**The Time Coin of presence** (`app/src-tauri/src/wallet.rs`, the coin walk and `wallet_state` in `lib.rs`, `app/src/Wallet.jsx`;
iOS 2314 `MTCoinTx.hereFrom` and `beHere`, `MTCoinSplit.minting` and `growing`). The node's second at which the person came
bounds a present MINT: it never counts from before the coming. While the person is here the chain is asked at once and then
every beat (40 s); the stay ends at the first beat that finds them away. The seconds since the last MINT, counted at the node's
second, rise on the Time Coins page as the Time Balance (the available and the seconds on their way); Send is weighed by the
available alone. Unit tests `wallet::tests::a_stay_asks_at_once_then_every_beat_and_ends_away` and
`a_present_mint_counts_from_the_coming_and_the_page_shows_the_same_seconds`; not run on a real wallet, not compared with an
iPhone.

**A word of who they are off the live road** (`app/src-tauri/src/post.rs` `channel_letter`). A pipe's key is held by both
people and by every device of each, and iOS (`MontanaPhoneNode.sendLetter`) and Android (`Pipes.kt` 1412-1420) lay the live
road without the author field. A name, a face or an about (`NM:`, `AV:`, `AB:`) that comes by a channel unsigned may be the
person's own other device telling the correspondent the person's own name: Windows took it as the correspondent's and renamed
the chat (reported 28.10.2026). Such a word is now left to the box, which brings the correspondent's copy and never a
device its own person's letter (one sub id). Unit test `post::tests::an_unsigned_name_off_the_live_road_never_renames_the_chat`,
which fails without the change. iOS and Android take such a word the same way; signing the live road on every client is a
question for the protocol, not answered here. Live run, `tools	winname.ps1` (28.10.2026 19:39-19:52 UTC, the official Android
584 on an Android 12 test emulator; Android 11 stops 584 once a conversation's signal starts, `CallLine.routeWatch` being an
Android 12 class): the person renamed on Android; build 4 named the correspondent's chat on the person's Windows device by the
person's own new name twice in 90 s, build 5 kept the correspondent's name (trace `channel_self_word unsigned -- left to the
box`). The person's own letters from Android, by a channel, stand there as the correspondent's (`theirs`) on both builds. Not
checked: a read mark.

## Checks

**On this tree**, as published, Windows 11 Pro 10.0.26200, with
`CARGO_TARGET_DIR=<repo>/target` and `RUSTUP_TOOLCHAIN=1.92.0-x86_64-pc-windows-msvc`, 28.10.2026:

| Command | In | Exit code | Result |
|---|---|---|---|
| `cargo test --lib` | `app/src-tauri/` | 0 | 190 passed, 0 failed, 19 ignored (debug, 44 s) |
| `cargo clippy --all-targets -- -D warnings` | `app/src-tauri/` | 0 | no warning |
| `python -I tools/vectors_2311.py` | `app/` | 0 | every vector the iOS 2311 tree states reproduces and every property check holds |
| `npx tauri build` | `app/` | not run | not built as published; build 5 below is of local `1e54b2d`: `7a45c11` after it rewords two comments of `post.rs`, and the three files named above differ |

Build 5, local `1e54b2d` (`npx tauri build` exit 0, 28.10.2026 18:57 UTC, 20.00 MiB installer), SHA-256 of its files:

| File | SHA-256 |
|---|---|
| `target\release\montana-desktop.exe` | `8ab435aa5832bfd54da4a17e893d13135a29d040e7b981d17088b66ef1bbaa2e` |
| `target\release\bundle\nsis\Montana_0.1.0_x64-setup.exe` | `0ee5d39b4d77260e6930843350e895b8863e85e07705246a58707cafce1307f6` |

**The UI scene on build 5**, 28.10.2026, on a fresh root of throw-away test identities on the live nodes: `ab` exit 0 (61 s).

Build 4, local `dc7689a` (the program code before the Time Coin of presence and the names), SHA-256 of its files:

| File | SHA-256 |
|---|---|
| `target\release\montana-desktop.exe` | `e252c1ee12047fe7b2f98d7d12ecaf4fcb30cba8368e2a3790cacfe766117b9f` |
| `target\release\bundle\nsis\Montana_0.1.0_x64-setup.exe` | `0b972e2fa849b5d0489bef708484752014527d3cf8dbed2e26c52229f9955950` |
| `montana-desktop.exe` as packed in that installer | `a30ad490acc1cab7e90c53f6d1028fdf432e165f228ea8252246608c79e43d71` |

The packed program differs from `target\release\montana-desktop.exe` only by Tauri's bundle marker (3 bytes, `UNK` -> `NSS`).
These hashes name the files the scenes below ran on; whether another machine builds the same bytes has not been tried. The
installer is not code-signed and is a local build, not an official release of Montana.

**The UI scenes on build 4**, 28.10.2026, one after another on one root of throw-away test identities on the live nodes, each
as `powershell -NoProfile -STA -File tools\SCENE.ps1 -Root DIR -Shots DIR` in `app/`, all exit 0: `ab` (38 s), `tray` (36 s),
`files` (32 s), `album` (34 s), `gfiles` (35 s), `drag` (35 s), `dropin` (22 s). `drag` and `dropin` move the real mouse for
a few seconds and put it back.

**The upgrade, `tools\upgrade.ps1 -Root DIR -OldDir OLD -NewDir NEW`**, 28.10.2026: 46 of 46 checks in 520 s. OLD is the
Windows line installed on 26.10.2026 (its `montana-desktop.exe` SHA-256
`fee375e3b9865e9afcda29acab1f7a452bbe502ac667f3e5daef1ce174881e3a`), NEW is build 4. Two throw-away identities, each in a
check's own folder, talk on OLD directly and through the box; then:

- NEW opens OLD's folders: the conversation, its letters, the identity's permanent link and the copy kept with the peer stand
  as they were, once; after a restart, letters go directly and through the box both ways;
- a restore by the 24 words into a fresh folder: 6 of 6 letters back, the copy OLD kept (generation 1) laid back, and the
  restored device shows a link of its own (32 random bytes per device, as iOS `MontanaLocalVault`);
- the days moved by `MONTANA_CLOCK_SHIFT_DAYS` (a check's run only): at +8 (03.11) the clock guard holds the first floor, at
  +12 (07.11) the floor is laid (`pipes_floored` in the trace);
- OLD opens a copy of the migrated folder and writes a letter in it, and NEW still finds the conversation whole.

**Not covered on this tree:**

- Windows with an official iPhone or Android across the floor: both identities of `upgrade` are Windows. The scenes with the
  official Android APK on a test emulator (`offident`, `offtwin`, `offchat`) are in the tree; their runs are not reported here
  (before `ed2147b` they ran `targetelease` whatever `-Exe` said: `$Exe` and `ui.ps1`'s `$script:exe` are one variable).
- Letters through the box on moved days: the live box keeps the nodes' real minute, so they can be checked only on the real
  dates (from 03.11.2026 UTC).
- The 75 scenes of `tools\all.ps1` have not been re-run on this tree's program code.
- No release build of the unit tests (`cargo test --release --lib`) on this tree.

**On the tree before this one** (local `aacbf76`, program code `abdabb2`), 25.10.2026, the same machine and variables:

| Command | Exit code | Result |
|---|---|---|
| `cargo test --release --lib` | 0 | 150 passed, 0 failed, 19 ignored |
| `cargo clippy --release --all-targets -- -D warnings` | 0 | no warning |
| `npx tauri build` (in `app/`) | 0 | the program and its NSIS installer `Montana_0.1.0_x64-setup.exe`, 19.94 MiB (23:16 UTC) |

The runs, then and now, were made in the developer's repository, whose `core/` is tree 17ea6d0e: the `core/` this folder was
first published against (4ce3d670) with the portability commit 291869b applied.

**The UI scenes on that tree**, the final full run on a fresh root, 26.10.2026: `tools\all.ps1`, 75 scenes, **73 passed**.

- `deeds` and `voice` failed once, and passed on a re-run and on a fresh root. `deeds` then got a scene fix (`aacbf76`): it
  waited for the Sent status, which stands for under a second before Delivered.
- The run before it found one fault of the app: `datapage`, the copy's count after its last page closed. `abdabb2` fixes it,
  and `datapage` passed on the fresh root.
- `slotpage`, new with the freeze fix, drives every way back from Settings, each of its pages and the slot's own pages, five
  times in a row, and checks that the window answers after each. It passed.

**Earlier, on the tree first published** (local `da7c109`, program code of `6aad43e`), 25.10.2026: `node tools/chess-check.mjs`
0 (40 of 40), `npm ci` 0, `npm run build` 0, `cargo clippy --all-targets -- -D warnings` 0, `cargo test --lib` 0 and
`cargo test --release --lib` 0 (150 passed, 0 failed, 19 ignored each). The full run of 74 scenes on that program code passed
68; the six failures were faults of the scenes or of the environment, fixed in the scenes since.

The 19 ignored unit tests need the live network or a second window; the scenes run those paths instead. The core, built with the
portability change of `dinetpda/core-windows-portability` (see Build), passed `cargo test --release --workspace` in `core/Code` on
19.10.2026: 1 421 passed, 0 failed, 1 ignored upstream (DEV-012). No constant-time measurement (dudect or other) has been made for
this port and none is claimed.

## Build

Visual Studio Build Tools with the MSVC toolset and a Windows SDK, Rust 1.92.0 (`x86_64-pc-windows-msvc`), Node and npm, and
Perl for OpenSSL's source build (Strawberry Perl); versions under Toolchain. Built and checked on Windows 11 only. In PowerShell,
from the root of a clone of this repository:

```powershell
$repo = (Get-Location).Path
# one target folder for the core and the app: the program and its installer take the DLLs from target\release
$env:RUSTUP_TOOLCHAIN = "1.92.0-x86_64-pc-windows-msvc"
$env:CARGO_TARGET_DIR = "$repo\apps\messenger\windows\target"

# 1. the core's C ABI, mt_bindings.dll. On MSVC the core needs the portability change of three files offered on branch
#    dinetpda/core-windows-portability (commit 291869b); apply it in your own clone only -- this folder never changes core/.
git fetch origin dinetpda/core-windows-portability
git cherry-pick --no-commit 291869b
$env:OPENSSL_SRC_PERL = "C:\Strawberry\perl\bin\perl.exe"   # wherever perl.exe is
Push-Location core\Code; cargo build --release -p mt-bindings; Pop-Location

# 2. the protocol core's boundary, montana_core.dll, built in a target folder of its own and laid in target\release
powershell -NoProfile -ExecutionPolicy Bypass -File apps\messenger\windows\app\tools\protocol-core.ps1

# 3. the program and its installer
Push-Location apps\messenger\windows\app
npm ci
npx tauri build
Pop-Location
```

The result is `apps\messenger\windows\target\release\montana-desktop.exe` beside `mt_bindings.dll` and `montana_core.dll`, and
an NSIS installer in `target\release\bundle\nsis\` that installs for the current user without administrator rights. It is not
code-signed, so Windows SmartScreen warns before it runs.

## Run the tests

With the two variables of the build set, after its steps 1 and 2 (the program's build script wants both DLLs in
`target\release`) and one `npm ci; npm run build` in `app` (the program embeds `app\dist`):

```powershell
Push-Location apps\messenger\windows\app\src-tauri
cargo test --release --lib                     # or cargo test --lib; mt_bindings.dll is found in target\release
cargo clippy --all-targets -- -D warnings
Pop-Location

Push-Location apps\messenger\windows\app
node tools\chess-check.mjs                     # the chess core of the interface, no build needed
python -I tools\vectors_2311.py                # the iOS 2311 pipe formulas, standard library only; --json OUT.json writes the rows

# The UI scenes: two or more real windows of the built program, driven by UI Automation, with throwaway test identities on the
# live Montana nodes. DIR is an empty scratch folder -- never the folder of a real identity. Each scene runs in a process of
# its own with a deadline; the summary says which passed.
powershell -NoProfile -STA -ExecutionPolicy Bypass -File tools\all.ps1 -Root DIR
Pop-Location
```

`tools\all.ps1` runs 75 scenes in turn; the others (the twin mirror with the Android port on emulators, the walk in the isolated
room, Windows Hello by hand, the tray's `tray`, `drag` and `dropin` after `ab` on the same root, the upgrade between two build
folders, and `offident`, `offtwin`, `offchat` with the official Android APK on a test emulator) run one by one. `drag` and
`dropin` move the real mouse for a few seconds; run them when nobody is using the computer. The scenes drive the Russian
interface. Between scenes `tools\all.ps1` closes the windows left behind only under the developer's own checkout path (its
line 17); in another clone, close them by hand.

## Toolchain

Read from the machine these checks ran on, 25.10.2026 (Rust, Node and Python read again on 28.10.2026, unchanged):

| | Version |
|---|---|
| Windows | 11 Pro 10.0.26200 |
| Visual Studio Build Tools 2022 | 17.14.28, MSVC 14.44.35207, Windows SDK 10.0.26100.0 |
| Rust | 1.92.0 (ded5c06cf 2025-12-08), `x86_64-pc-windows-msvc`, cargo 1.92.0 — the version `core/Code/rust-toolchain.toml` pins |
| Node, npm | 24.13.0, 11.6.2 |
| Tauri (crate and CLI), React, Vite, @vitejs/plugin-react | 2.12.1, 19.3.0, 8.3.2, 6.1.2 (pinned in `app/package.json` and `app/src-tauri/Cargo.toml`) |
| WebView2 Runtime | 155.0.4283.62 |
| Strawberry Perl (OpenSSL's source build) | 5.42.3 |
| Python (`tools/clocknode.py`, `vectors_2311.py`, `pipekey_days.py`, `glyphs.py`, `ui_atlas.py`) | 3.14.2 |

## Deviations from the specification and from iOS

`app/...` paths are in this folder; iOS paths are under `apps/messenger/ios/Code/`; line numbers are of `main` c7e11da unless
another commit is named.

1. **The pipe secret walks by the UTC day, not by the window.** Canon walks it one step per window of the chain from the window
   of the pipe's first letter: «The secret a tag stands on walks with the window» (`Audit/Protocol-13/source/docs/Montana
   Canon.md` 1338-1344, the invariant at 1359; `core/docs/Montana Canon.md` 1345-1351 and 1366 on the PR #11 line, commit
   f0ba296). Montana App on that line names the day walk of today's iOS client a temporary exception (`core/docs/Montana App.md`
   97, and item 8 at 195, commit f0ba296; not yet in `main`). The exception names the iOS client only, so for this port the day
   walk is a departure. The port walks as iOS 2305-2311 does: one step per UTC day from day 20754 (28.10.2026 UTC) for every
   pipe, at most 36 600 steps (iOS `Montana/MTPipeKey.swift` 42-50, 65; `app/src-tauri/src/pipekey.rs` 94-96, 136-164); from
   01.11.2026 UTC the tag of a window stands on the day's walked value, one value for the day's 1440 windows. The test
   `wire::canon_walk::canon_divergence_the_walk_is_by_day_from_one_anchor_and_the_tag_walks_by_the_day` (`wire.rs` 420-472)
   holds the divergence as it stands rather than bending the port to the vectors.
2. **Rules beyond iOS 2311, on by default.** The three switches of decision 188 (`MONTANA_D1_KEEP_CONFIRM`,
   `MONTANA_D3_SAME_PIPE`, `MONTANA_CLOCK_GUARD`; the section "New in this tree") make this port keep a first letter's
   confirmation after the floor, know a pipe met again in another form, and hold the first floor while the device's clock runs
   ahead of the nodes' Date — where iOS 2311 sends an empty confirmation after the floor (`Montana/MTPipeKey.swift` 166), knows
   only the very pipe it holds (`Montana/MTPipe.swift` 273) and floors by the device's day alone (line numbers of 2311's source,
   `main` c7e11da). iOS 2313 brought a guard of its own; the two have not been compared. Each switch set to `0` gives iOS
   2311's behaviour; whether these rules meet an iPhone or an official Android across the floor has not been checked.
3. **No fingerprint verification.** iOS compares a correspondent's key out of band and shows «Not verified» on the person's page
   until then (`Montana/MontanaPeerInfoScreen.swift` 349-350, `Montana/MontanaScreens.swift` 165, 202); this port has neither the
   comparison nor the mark, which `core/docs/Montana App.md` 91 requires ("A handshake not yet bound to a key by an out-of-band
   comparison is shown as unverified").
4. **No MTSamePair.** iOS folds the two pipes of one pair into one conversation (`Montana/MontanaChatStore.swift` 7519,
   `Montana/ContentView.swift` 4003); this port has no counterpart, so one person can stand in two rows.
5. **The drawer is at most 480 CSS px wide.** iOS: 78 % of the window (`Montana/ContentView.swift` 1635, `MontanaSideDrawer`);
   here 78 %, capped at 480 px on a wide window (`app/src/turn.js` 31-35, `app/src/pages.css` 2) — a Windows adaptation by the
   owner's decision of 25.10.2026, not an iOS number.
6. **The local stores are sealed by DPAPI** (`app/src-tauri/src/vault.rs` 86-115) where iOS uses the Keychain
   (`Montana/MontanaKeychain.swift` 136).
7. **Windows Hello before the 24 words show** (`app/src-tauri/src/hello.rs` 1-4) where iOS asks the passcode
   (`Montana/MontanaScreens.swift` 440-442); with no Windows Hello set the words stay hidden, as iOS keeps them on a phone
   without a passcode.
8. **Deleting an identity from this computer erases more than iOS does.** Its passwords, its VPN book with keys and subscription
   links, the seed of its files' keys and the window's browser data (storage, cookies, cache) are erased; its coin book,
   TimeChain and archive go aside first and come back to the same words (`app/src-tauri/src/forgotten.rs` 22-38, 66-123). iOS
   keeps those keychain items for whoever comes next (`Montana/MontanaChatStore.swift` 6895-6900, `SeedScope.keychainStays`).
   The owner's decision of 25.10.2026. Switching identity keeps every person's folder whole and closed to the others; the
   network identity, the coins on the chain and the person's backups are never touched; the files of letters are served with
   `no-store`, never laid in the web view's disk cache.
9. **No push service.** iOS is woken by its notification extension (`MontanaNSE/NotificationService.swift` 13); this program
   hears letters only while it runs, kept in the tray with its window closed (`app/src-tauri/src/lib.rs` 4585).
10. **The Explorer opens in the system browser** (`app/src/Wallet.jsx` 507-510, `app/src-tauri/src/lib.rs` 304-309) where iOS
    opens it in a sheet inside the app (`Montana/MTWalletScreen.swift` 1203-1209).
11. **No VPN tunnel and no Bluetooth mesh.** iOS carries a packet tunnel (`PacketTunnel/PacketTunnelProvider.swift` 188) and a
    Bluetooth mesh (`Montana/MontanaBLEMesh.swift` 95-97); this port has the VPN wall's server book and subscriptions alone
    (`app/src-tauri/src/vpn.rs` 1-5) and no mesh.
12. **The home node file is one comment behind.** `app/src-tauri/node/install.sh` and `README.md` are byte for byte iOS's
    `Node/`; `montana-home-node.py` is iOS 2244's, which differs from the current one in one comment line only (a time written
    in UTC).
13. **Approximations of the look, not compared on an iPhone:** the SF Symbols are replaced by SVG glyphs, most of them taken from
    the Android app's drawables (`app/src/glyphs.js`, made by `tools/glyphs.py`), a few drawn by hand — close in form, not the
    same; Liquid Glass and the system materials are CSS filters (`app/src/pages.css` 554-557: `.ultraThinMaterial` from a public
    dump of UIBlurEffect, the thin and regular materials estimated); the panel's icon sizes are by eye; the order of the date
    plates in Money Flow is not compared.
14. **A row asked back from another device of one's own lands after two minutes without an answer** (`app/src-tauri/src/twin.rs`
    525-530): an addition for an Android client that keeps its devices in step by archive blocks alone and answers no ask. iOS has
    no such wait, so a Saved Message written on an iPhone may reach a running Windows about two minutes late; not checked with an
    iPhone.
15. **A page whose read has not answered shows a waiting row, and a red row that asks again.** The window asks the program for
    what a page shows, where iOS reads the values at once (`@AppStorage`, `MTKeepingSection`). So the pages of Settings
    (notifications, data, privacy, language), the copy's section, a person's page, the wall's rules and the wallet's frame stand
    at once with a spinner row; when the read did not answer, the row turns red, «Could not be read.», with «Try again»
    (`app/src/Settings.jsx` 211-220 `Waiting`, 298, 322, 345, 379, 603; `Profile.jsx` 247; `Wall.jsx` 129, 772;
    `Wallet.jsx` 928). A Windows adaptation; not compared on an iPhone.
16. **The crop of a new photo has a loading frame.** iOS has the picture before its crop's cover rises (`MTAvatarCropView` in a
    `fullScreenCover`, `Montana/MontanaProfile.swift` 810-811); here a window reads it first (a HEIC is converted), so the crop
    stands at once with its ground, its cross and a spinner while it is read, never nothing over the page under it
    (`app/src/Profile.jsx` 93-104).
17. **The window guards itself where iOS needs nothing of the kind.** A second press of a double click that lands on another
    layer than its first is no press (`app/src/press.js`), and a page takes no pointer until its way in ends, as UIKit takes no
    touch during a push; a cover with no live page over it for a second is traced («cover freed») and taken away
    (`app/src/App.jsx`); a window that fell while drawing is drawn again, at most once in 30 s, and a second fall shows a black
    ground with «Try again» (`app/src/main.jsx`). A linked post that is not on this computer opens a Comments page with iOS's
    words, where iOS shows its verdict and opens nothing (`Montana/MontanaFirstContact.swift` 1938-1942). Not compared on an
    iPhone.

## Known defects

- One person can stand in two rows (deviation 4); no fingerprint verification (deviation 3).
- The floor of iOS 2310-2311 has not met an iPhone or an official Android, and has not run on the real dates (01.11 and
  03.11.2026 UTC); its rules beyond iOS 2311 are on by default (deviation 2).

## Cryptography

Montana's own protection is the core's: ML-KEM (FIPS 203), ML-DSA (FIPS 204), SHA-256, called through the core's ABI,
unchanged. The Windows side adds no public-key cryptography of its own. It uses, at the same places as iOS:

- ChaCha20-Poly1305 and HKDF-SHA-256 for the `.mtbak` v2 copy (the format iOS and Android read);
- AES-256-GCM (Windows CNG) for a group room's pair words, as iOS `MTRoomSeal` (`app/src-tauri/src/aesgcm.rs`);
- the Windows data-protection API for this computer's seal of the local stores (deviation 6).

Classical public-key cryptography appears only where an external standard requires it and is not counted as Montana's
protection: TLS (WinHTTP, the nodes' doors), WebRTC (DTLS-SRTP of calls), SSH (setting up one's own node, `russh`), and the VPN
wall's own protocols — as on iOS.

**Dinet Device Sync** — a planned, separate add-on by DinetPDA — Linkora for one identity's own devices where iOS has no
mechanism. Design stage only; no code here. It will be hybrid (classical + ML-KEM) with mandatory post-quantum protection and no
silent downgrade, and will not change the iOS wire.

## Internal notes held back

The developer's working notes stay unpublished by the owner's rule: they are written in Russian for the owner and his own
working sessions, and hold decisions, plans, logs of checks and references to the owner's private machines. What a reader needs
from them — the state, the checks, the deviations — is in this README. They are:

- the UI atlas of iOS against Windows, its checklist, its progress page and the list of reference shots wanted from an iPhone
  (`app/tools/ui_atlas.py` counts the atlas and needs it to run);
- the working plan, the reports of each part of the work, the logs, traces and shots of the checks reported above, and the
  Windows build and design notes;
- the iOS-to-Windows matrix and the notes on iOS builds 2304 and 2310;
- the research on one identity across devices (the twin mirror, Saved Messages, copies, deleting an identity);
- the notes on checks with the Android port and with an iPhone;
- the design notes of Dinet Device Sync.

## License

The core is dual-licensed MIT OR Apache-2.0 (`core/Code/LICENSE-MIT`, `core/Code/LICENSE-APACHE`). This port is offered to the
Montana project on the same terms as the repository it is part of. Third-party crates and npm packages keep their own licenses
(see `app/src-tauri/Cargo.lock`, `app/protocol-core/Cargo.lock`, `app/package-lock.json`). Every picture in `app/src/assets` and
the app icon are Montana's own iOS or Android assets from this repository, used for the same app; no Apple system artwork is
shipped.

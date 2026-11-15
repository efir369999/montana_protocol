1.0 (2405) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2405), from source commit `a9c33f2776a50d59c091f762c0522c3de56ebd72` of 2026-11-15. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Your identity is born on the phone alone. The Mac signs in by QR: it shows a code, you scan it in Settings — Devices — Connect a device on the phone, check that both screens show the same six digits and confirm with Face ID or your passcode. No words to type on the Mac.
- Settings — Devices on the phone lists your other devices and when each was last heard; «End session» erases your identity from that device as soon as it hears the phone. «End inactive sessions automatically» — 1 week, 1, 3 or 6 months: a device that has not heard the phone for that long erases it itself, offline too. On the Mac, Privacy shows «End session» in place of «Delete account»: it erases only the Mac; your account is deleted from the phone alone.
- Please check: on a Mac with no identity choose «Sign in by QR», connect it from the phone and see your chats appear; then end its session from the phone.

Security and anonymity
----------------------

- This build adds one way for the identity to leave the phone, and guards it: the words are sealed to a one-time ML-KEM-768 key whose fingerprint the Mac's QR carries, so a node cannot swap the key, and a photo of the QR opens nothing; they leave only after the phone owner's Face ID or passcode, and both screens show one number. The link box lives ten minutes. A joined Mac shows no words. Honest limit: a device kept offline keeps the identity until its term ends.

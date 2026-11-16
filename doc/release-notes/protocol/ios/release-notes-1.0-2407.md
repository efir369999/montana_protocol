1.0 (2407) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2407), from source commit `6cd34af09f52ca1fff9d18cdb0a890d124bc9f21` of 2026-11-16. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- While a voice or a video message is being recorded, the screen no longer turns: the recording buttons stay in place under your finger.
- Nothing else you see changes in Montana: this build separates the Messenger from the protocol core, and Montana keeps its node, its coins and the copies to your own node as before.
- Please check: start a voice message and a video message, turn the phone during the recording -- the screen stays put and the buttons stay.
- Please check: a backup and a restore from the 24 words, and the copy kept by the people you write to, work as before.

Security and anonymity
----------------------

- This build changes nothing of security or anonymity in Montana: the screen's turn is held while a message is recorded, and the digest a kept copy and a node's file are checked by moved to one owner, the same SHA-256 byte for byte.

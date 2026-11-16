1.0 (2409) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2409), from source commit `6530bd18934dc7e540fc92a1725112b5f617ede7` of 2026-11-16. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- A video message no longer freezes while it records: when the camera stops under the tape, it starts again by itself and the stopped stretch stays out of the note.
- The app's diagnostic diary reaches the developer even when its one diaries machine is out of the phone's reach: it goes through any other node of the network.
- Please check: record several video messages in a row, long and short -- the picture moves the whole length of each.

Security and anonymity
----------------------

- The diagnostic diary, sent only with the person's consent, may now pass through any node of the network on its way to the diaries machine; the node forwards it in one hop and keeps nothing. The camera's session words are written into that diary. Nothing else of security or anonymity changes.

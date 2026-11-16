1.0 (2410) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2410), from source commit `98d9c5873b862520f6cc066d6595f220425e6315` of 2026-11-16. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- A video message no longer freezes while it records: when the camera stops under the tape, it starts again by itself and the stopped stretch stays out of the note.
- With both cameras the big circle never waits for the small one after a flip or when both cameras turn on, and a small circle that stops starts again by itself.
- The app's diagnostic diary can reach the developer through a node that says it carries diaries on, when the diaries machine itself is out of the phone's reach.
- Please check: record several video messages in a row, long and short, with one camera and with both, flipping while recording -- the picture moves the whole length of each.

Security and anonymity
----------------------

- The diagnostic diary, sent only with the person's consent, may pass to the diaries machine through a node that says it forwards diaries; that node forwards it in one hop and keeps nothing. The camera's session words are written into that diary. Nothing else of security or anonymity changes.

1.0 (2360) Release Notes
========================

Montana for iPhone, iPad and Mac, build 1.0 (2360), from source commit `5666090ffadbd7907a7bc191d8a205ecaa19a919` of 2026-11-04. The folder `apps/messenger/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Names with @ are gone: no public @name, no search or link by @name, and the people you add are no longer written into your phone's Contacts. A contact is shared through the system's own picker, so Montana asks for no access to Contacts.
- Adding a person speaks only of your card: people reach you by its QR code or link, and the first screen of each app says what it is.
- Video in a voice call: the asker sees «Video request sent»; once the other person accepts, video comes on at both ends and stays until each turns off their own camera; turning your camera off never stops the other person's video, and a yes that arrives after a long wait still turns the video on.

Security and anonymity
----------------------

- No new server, network road or key leaving the device. The keeper of names is asked nothing and no @name is published; the app holds no Contacts access (NSContactsUsageDescription is gone from Info.plist, and tools/mt-apple-rules-check.py refuses it back), and the book of others' @names and the cards it wrote are erased on the phone (MTNameBook.forgetClaimedNames).
- The video consent stays a word between the two phones (call-video-ask, call-video-ok, call-camera-off): no new server, road or key; a yes is taken only within 120 s of one's own ask, and turning one's camera off rests only one's own capture.

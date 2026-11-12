1.0 (2384) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2384), from source commit `0be1c72052617bb484df4ea8e8d27faa29adc9ec` of 2026-11-12. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Restoring an account from the 24 words: the name you typed (for example «Demo Akk») is no longer replaced by an older one another device of the same words was given -- each setting carries the moment it was set, and the later one stands.
- New groups show their chat history to new members by default; the owner can still hide it.
- The Montana updates channel looks like a channel: a tap on it shows the current description and the link to the site.
- Photos are viewed one at a time, with no strip of thumbnails below, and with Share and Save buttons.
- The screen no longer goes dark during a group call, just as in a call of two.
- Please check: restore the account from the 24 words on the iPad -- the full name from the iPhone stands on both devices, your photo and your contacts' photos are in place.
- Please check: a new group with Chat History, a tap on the Montana channel's name, a photo in a chat, a group video call of a few minutes.

Security and anonymity
----------------------

- Copy card: each plain value (string, number) now carries the moment it was set (tag t:moments; a per-person book mt.card.moments in SeedScope.seatKeys, never in a copy). A copy laid as one of many (MTKeeping light and full copies) keeps a value set here later; a copy restored by hand from a file or iCloud lays its values as before. Older builds skip the tag. No change to the wire format of letters.
- The screen's idle timer has one owner (MontanaScreenAwake): a call of two and a group's room hold it by name, and the end of one no longer releases another's hold. Save writes into Photos through the existing road (MontanaSavedToPhotos). No change to the wire format.

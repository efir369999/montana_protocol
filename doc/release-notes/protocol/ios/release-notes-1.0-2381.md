1.0 (2381) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2381), from source commit `f4d52813684d4665426ab9a291067c837ed0b35a` of 2026-11-12. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Restoring an account from the 24 words: no separate group call chat appears any more -- a group call is started only from its group's chat, and an old invitation the network still held no longer creates a chat.
- After a restore your other devices are no longer woken by notifications about old group messages and group calls: what the restored device catches up on arrives silently.
- The faces of your contacts come back right after a restore, without opening each chat; your own name and photo reach the restore copy within a minute of being changed.
- A voice chat that has already ended no longer hangs in a group after a restore, and the message field and call button respond.
- The 24 words are accepted when pasted with tabs, non-breaking spaces or Windows line breaks.
- Please check: after the update, change the name and photo on the main device, wait a minute, restore the account from the 24 words on the iPad -- the name, your photo and your contacts' photos are in place, there is no separate group call chat, no voice chat hangs in the group, and the other devices show no notifications.

Security and anonymity
----------------------

- Restore: MontanaSeed.entered, the moment a person came in on this phone (written by MontanaSeed.enter alone, kept with the seat, never in a copy), is the one sign of a restore's catch-up -- a group's letter or room word born before it is carried on with hs=1 (no ring), an invitation born before it births no group (decision 865 = a). The light copy carries one's own name and face within a minute and the people within ten (decision 866 = a). No change to the wire format.

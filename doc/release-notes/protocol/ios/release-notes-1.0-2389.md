1.0 (2389) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2389), from source commit `11c34ed741ae85b72e9e53cf34434b7950eeb9d9` of 2026-11-13. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- One build number for Messenger and the Protocol: both are 2389, built from the same code.
- A message deleted for everyone no longer comes back as a notification: a banner rings only for a message that stands in the chat.
- The official Montana channel with the updates stands among the channels, not among the chats.
- A photo opened in a chat, a group or a channel turns to the next or the previous picture of the same chat with a swipe right or left; Save and Share stand under each one.
- A phone restored from a copy asks for a group message's photo, video or gif again and receives it.
- A new photo, video or gif refreshes your full copy at once, without waiting for the charger.
- Please check: ask someone to send you a message and delete it for everyone -- no notification about it comes again, even a day later.
- Please check: the Montana channel is on the Channels page and not on the Chats page.
- Please check: open a photo in a chat with several photos and swipe left and right.

Security and anonymity
----------------------

- A message deleted for everyone leaves the node box: when a deleted letter comes again, this phone asks every store to delete it (box-del), so it reaches none of the person's devices; before, the node's taken mark lived a day and the box yielded the deleted letter again.
- Deleted words never reach the notification list: the notification extension learns this phone's graves from the shared claim folder of the app group and shows a deleted letter's repeat as an empty plate it takes off at once.
- A banner rings only for a row standing in the chat (MontanaNotify.present asks ChatStore.stands). No change to the wire format; nothing new leaves the device.
- A group message's file asked for again after a restore rides the group's own road, end-to-end sealed as every group word; the node sees no new field.

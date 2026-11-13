1.0 (2390) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2390), from source commit `35cbcae9897e3f4e8c0b5f301f666ede91b9e79d` of 2026-11-14. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- One build number for Messenger and the Protocol: both are 2390, built from the same code.
- A group's picture opens sharp: the members receive the full picture, not the small one stretched.
- Every unread badge stands over its own icon on the time panel: chats over Chats, groups over Groups, channels and the official Montana channel over Channels.
- The official Montana channel's update messages are never empty: each build carries its own notes, and earlier messages that stood empty fill in.
- A group whose owner has not updated yet shows how many members it has, with a row saying how many are not listed yet.
- Please check: open a group with a picture -- the picture is sharp, not blurred.
- Please check: get a message in a chat, a group and a channel -- each badge stands over its own icon.
- Please check: open the Montana channel -- the update messages show what changed.

Security and anonymity
----------------------

- A group's full picture goes up to the node once, sealed under a fresh key and named by its content; the key rides only inside the end-to-end sealed invitation, so the node holds an opaque blob for a week and reads nothing.
- A build's update notes ride inside the app from the same file the TestFlight notes are written from; reading them asks nothing of the network.
- The unread badges and the members' count are computed on the device; nothing new leaves it.

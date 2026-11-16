1.0 (2412) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2412), from source commit `6517242ac6a0c7d5937325db28be51aacf3b78a9` of 2026-11-16. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Your own avatar no longer wears the coin balance: in Saved Messages and in the side panel it shows the level alone.
- A Montana link to a group, a channel or a person tapped inside a chat opens right in the app, with no trip to the site and no question which app to open it in; a link from outside still goes through the site.
- Sharing a link or a picture no longer waits for the slowest node: the message stands in the chat at once, and the nodes are chosen while it is prepared.
- A Mac joined by QR takes the history from the phone alone: the phone lays a fresh copy for this joining, the Mac reads until that copy arrives, never lays an empty copy of its own, and asks none of the people you write to for letters.
- What you write to Saved Messages or to anyone reaches your other devices within seconds, even while a backup is being made.
- Please check: send a group link from one phone to another and tap it in the chat; share a link through Share; join the Mac by QR and watch the chats arrive; send a photo to Saved Messages and watch it on the Mac.

Security and anonymity
----------------------

- A Montana link tapped inside the app no longer opens the site, so the site sees no request for it. A device joined by QR takes back only the copy its phone laid for that joining, never its own. The share sheet asks the same nodes as before, only without waiting for the slowest. A joined device no longer asks the people one writes to for letters and rings none of one's keepers, so they learn nothing of the joining. Nothing else of security or anonymity changes.

1.0 (2386) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2386), from source commit `f83b58cba25c5a2af52976eb962ae0e0eee5fb68` of 2026-11-13. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- In a group, the number of members in the header and the member list are the same: both come from the one list the group's owner gives.
- From a group you can open the page of any member and write to them, even if they are not in your contacts.
- A notification of a muted chat no longer hangs on the screen, and a tap on a notification opens its chat.
- A chat read on one of your devices takes its notifications off your other devices at once.
- A group letter sent again with «retry» no longer turns red a second later.
- The first post of a new channel or group appears as soon as you are added, without a five-minute wait.
- Channels and groups in the chat list show the same picture as on their page.
- Choosing Groups or Channels in the side panel puts their icon in the time panel's dynamic place; Chats keep theirs.
- Please check: open a group as a member -- the number in the header equals the list; tap a member, open their page and write to them; tap a fresh notification.
- Please check: read a chat on one device: its notification leaves the others; make a channel with a post and add two people: they see the post on entering; choose Groups in the side panel: its icon stands in the time panel.

Security and anonymity
----------------------

- A group's invitation (inv) now names every member's seat (MTGroupWord.ps), never a pipe; each member lays its permanent card («pc», COMPAT-GATED, only where the owner names seats), the owner hands the cards on, and a member writes to another by the existing meeting road (MontanaMeeting.meet). Older builds skip both keys.
- The notification extension keeps its quiet plates in one book under one queue, and a wake takes off only the plate it laid. No change to the wire format of letters.
- A device that reads a chat tells the person's other devices in the person's own pipe (MTTwinRead, sealed by MTKeeping.knockText); the extension reads that pipe with the secret the app mirrors under MTTwinRead.secretKey. Letters to other people are unchanged. A group word that comes before its invitation is kept in memory only (at most 64) and answered once the invitation lands.

1.0 (2369) Release Notes
========================

Montana for iPhone, iPad and Mac, build 1.0 (2369), from source commit `7569c4712df578c0850c0a4cfc2345643dcc2ab2` of 2026-11-11. The folder `apps/messenger/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- A new icon: Montana Protocol wears its new picture on the home screen, in the icon chooser and on the first screen; the round glass version stands in Settings and in the room.
- The nodes say honestly when their disk is short: a node whose storage has no room left refuses a new attachment at once and names it beforehand, and the app sends the piece to the next node without waiting. Nothing the app does has changed with it.
- Please check: the home screen icon, the icon chooser in Settings, the first screen; send a photo and a voice message; chats, groups, calls and Settings as usual.

Security and anonymity
----------------------

- No algorithm or key change. The app icon's pictures were replaced (tools/delivery-policy.json pins the new blobs and retires the old ones). The node store answers /blob-put with 507 when its shelf has no room and lays nothing, and /health names it beforehand (blob_room false while blob stays); the phone already treats any refusal as the next door. Door machines no longer carry a copy of the code.

1.0 (2394) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2394), from source commit `58e949bdd94a2cd26b4b5205dd320460f4db927d` of 2026-11-14. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Coming back to the app no longer freezes the screen: the system's upload session is prepared in the background (it could hold the screen for up to 10 seconds).
- A chat opens lighter: the page under it and the side drawer are not redrawn at every change of the chats, and while the chat rises the feed lays out only the part already shown.
- Incoming letters no longer hold the screen: read letters' files are removed in the background, the link finder is made once, and the extensions' diaries are read off the screen's thread.
- The chat list scrolls without a stall: a row's small picture and the chain's last word are prepared in the background.
- The app starts lighter: large saved values that stayed inside the settings move to files of their own, so the settings are read faster at launch.
- Please check: leave the app and come back, then open several chats in a row -- nothing stalls.

Security and anonymity
----------------------

- Nothing new leaves the device without consent: the diagnostics diary keeps the screen's measures in a file of their own (function addresses and counts, no message content) and sends them only by the consent already given.

1.0 (2396) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2396), from source commit `e41300b40747ff3002340121b305d6e49ef9fc62` of 2026-11-14. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- The Time Balance rises again by 1 coin every second you are here, as before: the seconds in the network are credited on the balance itself, not on a separate line.
- A chat with very long letters opens and closes without a stall: a long letter is now drawn away from the screen's thread, piece by piece, only where the screen shows it -- it looks exactly as before.
- The app opens faster: the search field no longer holds the first screen -- it stands as it was the moment the app opens, and comes alive right after.
- Please check: open the Wallet and watch the Time Balance -- it rises every second.
- Please check: open the chat with the longest letters and leave it several times in a row -- nothing stalls.

Security and anonymity
----------------------

- Nothing new leaves the device: the diagnostics diary notes only a long letter's size in bytes and the time its layout took, never its words.

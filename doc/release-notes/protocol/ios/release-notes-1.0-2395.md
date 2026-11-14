1.0 (2395) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2395), from source commit `5c183c83a2137b2122465f4cbd7d1e31ffa289f2` of 2026-11-14. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- A chat with very long letters opens and closes without a stall: a long letter is now drawn away from the screen's thread, piece by piece, only where the screen shows it -- it looks exactly as before.
- Please check: open the chat with the longest letters and leave it several times in a row -- nothing stalls.

Security and anonymity
----------------------

- Nothing new leaves the device: the diagnostics diary notes only a long letter's size in bytes and the time its layout took, never its words.

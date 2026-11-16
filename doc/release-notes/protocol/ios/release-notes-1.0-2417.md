1.0 (2417) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2417), from source commit `d7672fe4e018f973a5aac73af5bfa70897c20bb3` of 2026-11-16. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Your devices now keep count of what they told each other: every word one device passes to your other devices carries its number, a device that missed one asks for it by that number, and the device that wrote it sends it again -- a word no longer goes missing because a device was away or the delivery box was full.
- A word you save on one device now wakes your other devices at once, whichever machine they listen on.
- The sign of muted sound now stands to the right of the coin, larger and easier to tap.
- Please check: write and send a photo to Saved Messages on one phone and watch it appear on the other device; then turn the other device off for a while, write again, and turn it back on.

Security and anonymity
----------------------

- A word passed between your own devices still rides sealed in the pipe only your devices derive; the new number and run are inside that sealed letter, and the device keeps the roads of its own letters on its disk for six days at most. No new name, address or key leaves the device. Nothing of anonymity changes.

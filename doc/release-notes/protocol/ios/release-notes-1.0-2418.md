1.0 (2418) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2418), from source commit `c22a497d5616ea95a52cc0855f9880fbaf101df5` of 2026-11-16. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Your devices now keep count of what they told each other: every word one device passes to your other devices carries its number, a device that missed one asks for it by that number, and the device that wrote it sends it again -- a word no longer goes missing because a device was away or the delivery box was full.
- A word you save on one device now wakes your other devices at once, whichever machine they listen on.
- The sign of muted sound now stands to the right of the coin, larger and easier to tap.
- A chat's diary now names each row's kind by its file and writes one print of the whole chat, so two of your devices can be compared row for row.
- Settings -- Devices now shows a device you connect as soon as it is heard, without a separate «connected» line, and the list stays current while it is open.
- Your backup copy no longer piles up on the network's machines: each new copy takes the one before it off, and the relay machines keep nothing longer than a week.
- Please check: write and send a photo to Saved Messages on one phone and watch it appear on the other device; then turn the other device off for a while, write again, and turn it back on.

Security and anonymity
----------------------

- A word passed between your own devices still rides sealed in the pipe only your devices derive; the new number and run are inside that sealed letter, and the device keeps the roads of its own letters on its disk for six days at most. No new name, address or key leaves the device. Nothing of anonymity changes.
- The phone now keeps on its own encrypted vault the names and keys of the lists of its own backup pieces, to take them off the machines later; no new name, address or key leaves the device. Backups rest on one machine of the network instead of three; their content stays sealed as before.

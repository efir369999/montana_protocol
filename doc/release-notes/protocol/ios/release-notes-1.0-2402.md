1.0 (2402) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2402), from source commit `4bff00b380425198e5e0f1a00186d022ba39d40b` of 2026-11-15. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- A letter of yours that reached your other device without its signature, and stood there on the correspondent's side, moves to your side as soon as your own copy of it arrives.
- Your other device now gives back every letter it is asked for, a pair's named letters too, and a file it could not hand over is tried again four times instead of being dropped; Saved Messages appears on a device that never wrote to itself.
- Please check: write from the phone to a person and to Saved Messages, then open both on the Mac -- your letters stand on the right, theirs on the left, and the photo is in Saved Messages.

Security and anonymity
----------------------

- This build changes neither security nor anonymity: the side of a row follows the seed's own sealed word, and the seed's devices give each other rows they already exchange; nothing new leaves the phone and no one else is told anything.

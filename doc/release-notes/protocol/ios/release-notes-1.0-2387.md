1.0 (2387) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2387), from source commit `3081b32b00f182046d6a2de01b3ba77cfcd45592` of 2026-11-13. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- A group chat you deleted, such as the chat of a group call, no longer comes back by itself: only a new letter in it, or being added again, brings it back.
- In a group, a tap on a member whose page cannot open yet says why: the page opens once the group's owner updates Montana.
- On a device restored from a copy, a tap on a photo or video in a personal chat that came without its file asks its sender, and the file arrives again while the sender still has it.
- Please check: delete the chat of a group call; after the update it does not come back.
- Please check: in a group, tap a member who is not in your contacts: their page opens, or a line says it opens after the owner's update.
- Please check: on a restored iPad, tap a photo without its file in a personal chat: it arrives.

Security and anonymity
----------------------

- A restored device asks a personal chat's sender for a missing attachment by the existing cargo-gone word (cargoLostMark); the sender lays the file again under the same letter name, once per letter in six hours on each side (keptFileAsks in the local vault). No new word on the wire.
- A group's own renewal words no longer lift a row the person deleted; only a letter, the group's birth or being added again do. No change to the wire format.
- A group member with neither pipe nor card shows a local alert only; nothing is sent.

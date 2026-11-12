1.0 (2383) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2383), from source commit `bb0ba72125ec2a33cc1768d5f5c1e4c0a4cee7c2` of 2026-11-12. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Restoring an account from the 24 words: a letter your contact gave back before the restore copy arrived no longer stands twice in the chat -- the copy's own letter takes its place.
- Please check: restore the account from the 24 words on the iPad while your contacts are online -- every letter of the conversation stands once; the name, your photo and your contacts' photos are in place.

Security and anonymity
----------------------

- Restore: a row a correspondent gave back (mid given:) and the full copy's row of the same letter are known by one name, ChatStore.rowSig; the given row yields to the copy's row and its journal file is dropped (critic K3). No change to the wire format.

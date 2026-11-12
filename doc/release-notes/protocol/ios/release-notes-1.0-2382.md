1.0 (2382) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2382), from source commit `a48939be104a42f108d15eab4e1f6909b36a866a` of 2026-11-12. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Switching between accounts on one phone: if the phone's keychain refuses the account being opened, everything of it goes back to its place and the account you left opens as it was -- the two accounts never mix.
- Please check: on a phone with two accounts, switch between them several times -- each opens with its own chats, name and photo, and nothing of one shows in the other.

Security and anonymity
----------------------

- Seats: MTSeats.liftStore folds a lift back when the seed is refused -- the lifted folders return to the shelf and the store is cleared before the person who left is lifted again (critic K4). No change to the wire format.

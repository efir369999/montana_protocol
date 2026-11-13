1.0 (2391) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2391), from source commit `bdce3e2ea7dbf7a4ee8f33f19120822a8231d8d9` of 2026-11-14. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- One build number for Messenger and the Protocol: both are 2391, built from the same code.
- One person is one on every device of the seed: the name, face, bio, link and presence switch changed on one device stand on all of them, the later change winning.
- The shown balance is the chain's, one on every device of the person; the presence seconds the core has not yet confirmed stand on their own line.
- The person's choices ride to the person's other devices entry by entry: pinned, muted, blocked and archived chats, pinned and archived contacts, the contacts and the names given by hand. A block set on the phone holds on the Mac too.
- An entry met for the first time only joins: another device's older word never lifts a block, a contact or a pin that stands here.
- Please check: rename yourself on one device -- the other devices show the new name.
- Please check: block a chat on the phone -- on the Mac it is blocked too.
- Please check: the balance on the phone and on the Mac is one number.

Security and anonymity
----------------------

- The person's own keys and choices ride only to the seed's own devices, through the twin mirror sealed under the seed's secret; no other person and no node reads them.
- Faces set by hand are files of each device and stay on it.

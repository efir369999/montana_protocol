1.0 (2408) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2408), from source commit `8a4ed08f050232dc71e9a068606ece766e54ee35` of 2026-11-16. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- A Mac signing in by the phone's QR stays signed in: it no longer stops at «Restoring the identity chain» at 0 % and is no longer thrown back to the sign-in, and its chats arrive from the phone.
- Settings -- Devices: every session can be ended, not only a Mac's, and a new button «End all sessions» ends every other device at once.
- Links to invitations, groups, channels and contact cards that Montana writes now open on montana.xxx.
- Please check: sign in on the Mac by the phone's QR -- the restore goes past 0 %, the chats come, and the Mac is not signed out.
- Please check: on the phone, Settings -- Devices -- «End all sessions» clears the list; each device has «End session».
- Please check: share a group or channel invitation -- the link starts with montana.xxx and opens Montana.

Security and anonymity
----------------------

- The QR box of a Mac's sign-in now carries only its key, no device tag before the words. The word «end» carries the moment it was said: a Mac signed in anew after it is not erased by an older end, and the phone lifts that older end. A device ended from the list gets no more history from the phone; a device that signed in with the seed phrase keeps its words. Universal links are claimed on montana.xxx instead of montana.quest. Nothing else of anonymity changes.

1.0 (2416) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2416), from source commit `4b559de03d11db022425d7ef82072ef01d975cb6` of 2026-11-16. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Your groups and channels now come to the Mac joined by QR even where an earlier session on that Mac was ended: the old session's sealed leftovers no longer hold the groups' list shut.
- Ending a session on a device now takes every value of that person off it, not only the chats -- nothing stays behind under a key that is gone.
- A backup's piece now leaves the phone once: the machine that takes it lays the copies on the other machines itself, retrying a machine that does not answer -- a third of the phone's traffic for a backup.
- Please check: end the session on the Mac, sign in again by the phone's QR, and compare the list of groups and channels with the phone.

Security and anonymity
----------------------

- Ending a session now removes the person's sealed values together with the device key that sealed them, so nothing unreadable of that person stays on the device.
- A backup's piece is the same sealed ciphertext as before; a machine copies to its siblings only what it already holds, and no new name, address or key leaves the phone. Nothing of anonymity changes.

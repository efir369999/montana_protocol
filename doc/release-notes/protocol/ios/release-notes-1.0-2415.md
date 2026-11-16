1.0 (2415) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2415), from source commit `0be38b730237f448f80299d790d3568cf60049ee` of 2026-11-16. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Saved Messages: a photo, video or file you put there now reaches your other devices -- the Mac joined by QR and back. Its file goes out as your own send, no longer behind the backup's upload, and it is kept on the device until it has left, so closing or updating the app on the way loses nothing.
- A group's invitation link tapped in a chat now opens the group itself: at once if you are already in it, otherwise the moment the owner's phone adds you -- no longer leaving you in the owner's chat.
- Please check: put a photo into Saved Messages on the phone and open Saved Messages on the Mac; then the other way round.
- Then on one phone tap a group's invitation link sent from another and watch the group open.

Security and anonymity
----------------------

- The file of a Saved Messages row is sealed under a fresh key as before, and its key rides only in the pipe your own devices open; only its place in the phone's queue changes.
- Nothing of security or anonymity changes.

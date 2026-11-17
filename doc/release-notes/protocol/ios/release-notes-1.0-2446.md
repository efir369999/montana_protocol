1.0 (2446) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2446), from source commit `17b1f7c4143f5b608410497ff67c2afb450b61f1` of 2026-11-18. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Restoring your account on a new device now takes your other device's current history and never an older copy.
- A message you deleted stays deleted when you restore your account on another device, whichever way the history comes back.
- A chat or group you delete leaves nothing behind on your device and no longer comes back on your other devices.
- A new device of your account no longer keeps stray pieces of other devices' past, and each device's release-notes channel shows only its own build.

Security and anonymity
----------------------

- Anonymity is unchanged. Security: a new device of your words takes its history only from the picture your own device names for it now, sealed by your seed as before; an older copy is never laid, and a deleted message is refused by every road that brings history back.

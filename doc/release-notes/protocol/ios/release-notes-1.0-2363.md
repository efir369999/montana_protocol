1.0 (2363) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2363), from source commit `9a67c720f5c4c81913dd4f9a10b9ffcbd9f92492` of 2026-11-06. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- The music's diary on this device tells a track that ran to its end from one the phone could not read to its end: a track that falls silent early is named by the moment it stopped and its length, so its cause can be found and mended.

Security and anonymity
----------------------

- No new server, network road or key leaving the device. The diary on the device (MontanaTrace) gains three marks -- music_end (the player's clock, the bar's tick, the length), sound_end broken and sound_decode (the platform's error code) -- naming no track, file or folder.

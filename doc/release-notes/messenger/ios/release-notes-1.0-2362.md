1.0 (2362) Release Notes
========================

Montana for iPhone, iPad and Mac, build 1.0 (2362), from source commit `46960956b97be1aa5b6d710a14a1643a3b31b9a4` of 2026-11-06. The folder `apps/messenger/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- The music page shows only tracks that play: the tracks of a lent folder that did not open stay out of the list, the queue and the shuffle until the folder opens again or is lent again with the plus, and the folder lent again takes its tracks back in their places.

Security and anonymity
----------------------

- No new server, network road or key leaving the device. A lent folder that does not open is told in the diary by its cause (MTMusicFolders.open: the bookmark's refusal or the platform's), naming no folder; its tracks stay in this device's own list (Application Support, excluded from backup) and the music list is written only when it changes.

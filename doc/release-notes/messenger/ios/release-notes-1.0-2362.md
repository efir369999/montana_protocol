1.0 (2362) Release Notes
========================

Montana for iPhone, iPad and Mac, build 1.0 (2362), from source commit `6e2b29ca7c9c462f821bebbf6480ddf2dbf5a8a3` of 2026-11-06. The folder `apps/messenger/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- The music page shows only tracks that play: the tracks of a lent folder that did not open stay out of the list, the queue and the shuffle until the folder opens again or is lent again with the plus, and the folder lent again takes its tracks back in their places.
- Passwords leave the app: the Passwords tile and page are gone, a secret letter sent by an older build is no longer shown, and the passwords vault is erased from this device's keychain at launch.

Security and anonymity
----------------------

- No new server, network road or key leaving the device. A lent folder that does not open is told in the diary by its cause (MTMusicFolders.open: the bookmark's refusal or the platform's), naming no folder; its tracks stay in this device's own list (Application Support, excluded from backup) and the music list is written only when it changes.
- No new server or network road. The passwords vault leaves both editions (decision 671 (a)): its keychain item is known by print and deleted at launch (MontanaKeychain.forgetRetired, decision 673 (a)) and never restored from a seat; a secret letter of an older build is a retired kind, buried unread and ringing nothing (MTRetired.letterLines, decision 668 (a)).

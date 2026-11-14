1.0 (2400) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2400), from source commit `37a89b823a2045dbfa9e1f2e7a6872a8cbeb9c6e` of 2026-11-14. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Your own files sent before a restore come back: the phone asks the correspondent for them by the second, kind and place of the letter, and the correspondent's phone lays its copy into the same row -- no second row is born.
- In groups, your own files after a restore come back the same way: the group asks, and only the holder of your letter lays its copy back to you.
- Files your correspondent sent are given back with their rows when a chat is restored, and each file loads by the seal of its letter.
- A photo, video, GIF or document in Saved Messages after a restore is asked of your other device of the same seed phrase and lands in the same row.
- A photo or a word shared into Saved Messages from another app now reaches your other device of the same seed phrase.
- Please check: restore the account from the seed phrase and open a photo you sent before the restore, in a chat and in a group -- it loads.
- Please check: restore the account, open Saved Messages and tap a photo without its file -- it loads from your other device.

Security and anonymity
----------------------

- The asks name no word of a letter: a restored own file is asked by its second, kind and place, a saved one of the twin by the hash of its row; the files travel end-to-end encrypted, as before.

1.0 (2392) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2392), from source commit `4e5bd3b37632817dbdfc0e3e166f963dfe829091` of 2026-11-14. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- One build number for Messenger and the Protocol: both are 2392, built from the same code.
- A chat opens at once: a bubble no longer asks the disk while it is drawn -- where each photo, voice and video lies is known in memory and learned off the screen's thread.
- The rest of a long chat is laid after the chat has risen, in small parts, so the opening does not stall.
- The chats list stands still under an open chat and catches up when the chat is closed.
- The calls log reads again only the chats that changed.
- The coin's correspondents are read from the disk once and held, so a chat no longer waits on that file.
- Please check: open a chat with photos and voices -- it opens as fast as a chat of words.
- Please check: close the chat -- the list shows its last letter and unread count at once.

Security and anonymity
----------------------

- Nothing new leaves the device: where a file lies is a word in memory of this device only.

1.0 (2368) Release Notes
========================

Montana for iPhone, iPad and Mac, build 1.0 (2368), from source commit `d1377b726272a7f98bb8c882773f8d0e5f1c6ae8` of 2026-11-11. The folder `apps/messenger/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Voice messages under the lock: the controls of a locked recording no longer vanish. They now follow the recording itself, not the piece of the bar they were drawn from, and a moment after the lock the app checks that they stand on the screen and can be pressed. The cross, the play and the pause are one size, on one line inside the recording's bubble, with even steps between them, and each wears the blue ring.
- Photos and voice messages leave at once: what you send no longer waits in line behind the app's own background uploads -- the pages of the wall carried to its visitors among them, and a piece no longer stands for up to a minute at a node that takes no byte -- it goes to the next one. An attachment is laid on every node, including one that was resting a moment ago; and when the receiver reports an attachment lost, it is sent again once and a second check no longer takes the repair away.
- A call that reaches the phone late -- for example after it had no data during a cellular call -- now shows who called and leaves one missed call in the phone's list instead of a nameless failed call.
- Please check: record a voice message, lock it, turn the screen and back, press pause, play, the cross and send; send a photo and a voice message after the app has been in the background; chats, groups, calls and Settings as usual.

Security and anonymity
----------------------

- No algorithm or key change. A call envelope is still opened only by the pipe's key of a minute; a wake of another minute is also tried at the minute the node stamped it -- the stamp chooses the minute, the seal still proves the sender, and the call's own age still buries a stale call (one missed line for one call). MTBulkPuts gives each cargo places of its own and the person's put the way before the app's quiet puts; MTPutPulse reads a node's silence from the bytes sent; a resting node is asked last instead of being left out, so a copy reaches every node; the sender's cargo check asks one question for a letter at a time and its verdict names the letter by its birth. A wall page's long body is quiet cargo; two diagnostic words were renamed; the labels keys are derived from are unchanged, so every existing key, chat and copy stays readable.

1.0 (2398) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2398), from source commit `bf8635d8fc386ed33a962349be73847781e82782` of 2026-11-14. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- A GIF in a chat no longer stands as a dark plate: the app's first look at its files now tells every picture that asked about its file while the look ran.
- The input field no longer hangs when you open stickers or GIFs, and the app no longer closes on that page: the screen is laid out after the keyboard has changed, not in the middle of the change.
- An app installed anew with no account signed in asks the notification servers to forget the address its last install left, and clears its old notifications.
- A voice reply recorded at the ear keeps its buttons: the recording stands with its send, bin and pause from its first second, so nothing vanishes from the bar.
- Saved Messages reach your other devices: a note written on the phone appears on the Mac with the same account, and a note the other device missed is given back when it asks.
- Notifications turned off no longer take the place of a button on the top panel: a small crossed-speaker badge stands over the logo and opens the notification settings.
- The side panel closes with a sideways swipe from anywhere on it.
- Replying at the ear starts with a short, soft two-note sound, like the start of a recording in Voice Memos.
- The Protocol on the Mac no longer weighs down the computer: the queue of outgoing letters is written to disk at once only when a new letter is born, and the rest at most every ten seconds, instead of the whole file at every receipt.
- The Protocol on the Mac no longer freezes: a write of the settings away from the screen's thread no longer waits for that thread, which was waiting for it in turn.
- Please check: in a chat open the stickers, then GIFs, then the letters again, several times in a row -- the field answers at once.
- Please check: listen to a voice message with the phone at your ear, reply after the tone, then take the phone away -- the recording waits with all its buttons.

Security and anonymity
----------------------

- Nothing new leaves the device: a phone with no account only asks the notification servers to forget its own old address.

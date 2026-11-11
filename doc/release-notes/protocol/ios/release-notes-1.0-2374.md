1.0 (2374) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2374), from source commit `f2d7f1c3567ac8a1dad4c48cd40e213632a84bcc` of 2026-11-11. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Switching to the emoji keyboard while typing no longer covers the input field: the field and the messages rise to the top of the emoji keys at once, and come back down when you switch to letters.
- The build's number now lives in one place, so every part of the app carries the same number by construction.
- Groups and Channels are in the side panel; the strip of folders under the time panel on the chats page is gone (as in 2373).
- Please check: Settings, About shows build 2374; tap Groups and Channels in the side panel.
- Please check: open a chat, tap the field, switch to the emoji keyboard and back several times -- the field always stands right above the keys; then type a word with emoji and send it.

Security and anonymity
----------------------

- No algorithm, key or network change: the build number and version moved from 22 target settings of the project into Version.xcconfig, read by every target.
- No algorithm, key or network change: the chat page also listens to the system's change of the input mode (UITextInputMode.currentInputModeDidChangeNotification) and places the input bar on the keys' top read from the keyboard's own accessory; tools/mt-keyboard-drag-check.py, now in the build ring, refuses a rider without it.

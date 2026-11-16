1.0 (2421) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2421), from source commit `b72b5ed286276c3dd5ba989824a484097754ad68` of 2026-11-16. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- A message you write in Saved Messages now goes to your other devices the moment it is written, in a letter of its own, and stands in the open chat at once; its photo or file follows right behind it.
- Your other devices now hear a new message the same second even when they listen at another machine of the network than the one the writing device chose.
- A photo in Saved Messages that arrived without its file now asks the device that wrote it again at once and appears in the chat, not only in the list.
- A Mac joined by the phone's QR now goes on exactly from where the phone's copy stopped: messages written while the copy was on its way are no longer lost.
- A device whose session you ended now learns of it from any letter of your other devices, even after days away, and it leaves the list of devices everywhere.
- Your devices now compare every chat with each other every few minutes; when one of them lacks rows, they fill them in by themselves.
- Please check: sign in on the Mac by the phone's QR, write a message and send a photo to Saved Messages on the phone and watch both appear in the open chat on the Mac within a second; then end the Mac's session from the phone.

Security and anonymity
----------------------

- Every letter between your own devices still rides sealed in the pipe only your devices derive; the new Saved Messages letter, the list of ended sessions, the copy's cursors and each chat's print (a count of rows and a hash of their keys) all lie inside that sealed letter. No new name, address or key leaves the device. Nothing of anonymity changes.

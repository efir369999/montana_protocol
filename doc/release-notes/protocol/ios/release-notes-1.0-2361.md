1.0 (2361) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2361), from source commit `a2913264003bb97484b7e173f09f77e847b72642` of 2026-11-05. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Each app opens by its own link scheme: Montana by montana://, MT Protocol by mt-protocol://. A Montana link in a letter opens inside the app where you tap it, and an invitation still opens in either app.
- The app has one name everywhere: the permission questions no longer name it (the system already shows its name above them), and its iCloud folder is called MT Protocol, as under the icon.
- The call screen no longer freezes in the first call after the app starts: the mute button answers, the video request is shown to the caller, and the video comes on for both sides once the other side agrees. The screen now proves it was laid out again after every change and is born again at once when it was not.

Security and anonymity
----------------------

- No new server, network road or key leaving the device. The two schemes are stated once (Code/Names.xcconfig: MT_SCHEME_PROTOCOL, MT_SCHEME_MESSENGER) and checked by tools/mt-names-check.py; a montana:// link tapped in a text goes to the app's one entrance of a link (MontanaMeeting.handleLink) and never leaves the app by the system's round.
- No new server, network road or key leaving the device. Each product's name is stated once (Code/Names.xcconfig: MT_NAME_PROTOCOL, MT_NAME_MESSENGER); the iCloud folder reads it, no purpose string names an app, and tools/mt-names-check.py (rules 8 and 9) holds the store page and the site to the same owner.
- No new server, network road or key leaving the device. The call window's host (MTCallHost, Code/Montana/MontanaCall.swift) counts its layouts; CallWindowPresenter raises a generation at every change of the call model, the screen's tree echoes it from inside its geometry, and a generation not echoed within 0.3 s is written to the diary (call_layout stall) and the window is born again, twice a call at most (call_window reborn).

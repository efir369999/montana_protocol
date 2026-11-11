1.0 (2365) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2365), from source commit `06cca42c988b9d2596193c6f34e118a3015f9e71` of 2026-11-09. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- The voice button is rebuilt so the press reaches it: on iOS 27.0.1 a press on the microphone did not reach the button at all, so holding it recorded nothing. The press is now taken by the button's own platform recogniser, and its whole area is declared touchable.

Security and anonymity
----------------------

- No change to any algorithm, key or network road. MontanaHoldControl tracks the press through a UILongPressGestureRecognizer (minimum duration 0, cancels and delays nothing) instead of UIControl's beginTracking, with the same thresholds and verdicts; the slot carries contentShape(Rectangle()). The diary on the device gains hold_hit (the point and the enabled flag where the platform hands a finger to the button) and hold_down by=press -- coordinates only, no content.

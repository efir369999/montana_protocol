1.0 (2359) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2359), from source commit `ada594f9dc02fa63103a1c32a5d0c9078994415c` of 2026-11-04. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Call recording is gone: no switch in Settings, no record mark on the call screen, and what an earlier build left of it on this phone is removed. A recording already sent to a chat stays in that chat.
- Your node is gone from Settings. Files a node already holds still come back when you open them.
- Turning video on during a voice call no longer stalls: requests that cross are settled at once, a phone already in video answers by itself, and the Video button shows the other camera joining.
- The Montana room with updates stands for everyone: the build in your hands is its first row, even before the network answers.
- The Update button appears only when a newer build is out.
- The copy kept by the people you write to is laid only on the charger, outside Low Power Mode, with the phone cool and not on a metered network: no more heat on the battery.
- Asking the nodes which files they hold goes in slices of 900 names, so a large library is no longer refused.

Security and anonymity
----------------------

- No new server, network road or key leaving the device. Call recording is removed from the code: no recorder, no recording switch in a call's capabilities; the leftover unfinished files and the two settings keys are erased on the phone (ChatStore.sweepCallRecordings). The own-node row is drawn in no edition (MTEdition.ownNode false) and the node sends nothing.
- No new server, network road or key leaving the device. A video request is answered from the call's own state (CallUIModel.videoJoinPending, a 40 s join window); no new message kind.
- No new server or key leaving the device. The update document is read by the app's own bundle (updates/BUNDLE-CHANNEL.json) from montana.xxx and the nodes, as before; the app accepts only a document naming its own bundle. The keeping copy now waits for charger, cool phone and an unmetered path (MontanaPower.bulkHeld).

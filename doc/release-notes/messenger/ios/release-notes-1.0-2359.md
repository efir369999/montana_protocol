1.0 (2359) Release Notes
========================

Montana for iPhone, iPad and Mac, build 1.0 (2359), from source commit `ada594f9dc02fa63103a1c32a5d0c9078994415c` of 2026-11-04. The folder `apps/messenger/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- The Montana room with updates stands for everyone: the build in your hands is its first row, even before the network answers.
- The Update button appears only when a newer build is out.
- The copy kept by the people you write to is laid only on the charger, outside Low Power Mode, with the phone cool and not on a metered network: no more heat on the battery.
- Asking the nodes which files they hold goes in slices of 900 names, so a large library is no longer refused.

Security and anonymity
----------------------

- No new server or key leaving the device. The update document is read by the app's own bundle (updates/BUNDLE-CHANNEL.json) from montana.xxx and the nodes, as before; the app accepts only a document naming its own bundle. The keeping copy now waits for charger, cool phone and an unmetered path (MontanaPower.bulkHeld).

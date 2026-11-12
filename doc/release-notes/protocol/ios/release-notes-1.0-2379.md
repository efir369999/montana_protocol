1.0 (2379) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2379), from source commit `310cfb4b4b54e90b5c75691cd132178898416ecb` of 2026-11-12. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- On an iPad, which has no earpiece, a call no longer shows a Speaker button that could not be turned off: the sound always plays through the iPad's own speaker, and with headphones or Bluetooth the Audio button and its menu appear as on the iPhone.
- Please check: on the iPad make a call and a group call -- there is no Speaker button, the row of buttons closes up; connect headphones -- the Audio button appears.

Security and anonymity
----------------------

- No network or protocol change: MontanaAudioRoute.hasEarpiece is the one place that knows whether the device has an earpiece; MontanaAudioRoute.Face.shown hides the loudspeaker switch where there is nothing to switch (decision 861 = a, the author's word 20038).

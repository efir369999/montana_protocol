1.0 (2364) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2364), from source commit `0a3ed5688699903d530f9bcd698af1ca009e9d50` of 2026-11-06. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Music keeps its word with the car and the headphones: when a call, Siri, a navigator or a car's Bluetooth takes the sound, the track pauses where it stands and the bar says so; when the sound is given back with «resume», the track plays on from its place. A headset or a car that goes away pauses the track instead of playing it aloud into the room.

Security and anonymity
----------------------

- No new server, network road or key leaving the device. The diary on the device (MontanaTrace) gains music_remote (the lock screen's, headset's or car's button), music_interrupt (began/ended, resume asked, played on) and music_route (an output device came or went, its port type) -- naming no track, file or folder; play_refused resume from the previous build stays.

1.0 (2377) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2377), from source commit `45aae68628f073d769486e9719b35e25c253cf62` of 2026-11-12. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- A group call that everyone else has left ends on the last phone too: it no longer stays open with nobody in it.
- A group call that has ended no longer comes back in the group as an empty call: a late message of a participant who already left cannot open it again, and leaving such a call no longer leaves it hanging.
- Please check: in a group call of T1, T2 and T3 leave the call on T1, then on T2 -- the call ends on T3 at once.
- A phone that leaves a group call and comes back into the same call within half a second connects again: the closing of its earlier connection no longer takes the lane of the new one.
- Please check: in a group of three phones (T1, T2, T3) leave the call on T2 and come back into it at once -- T2 is heard again on T1 and T3.

Security and anonymity
----------------------

- No network or protocol change: MTGroupRoom.close closes a pair's lane only when no other living pair of the room in hand stands on it; tools/mt-room-lane-check.py refuses a tree without that rule (critic 19744).
- No network or protocol change: MTGroupRoom.endIfLeftAlone ends a call (not a stream) for the last seat once every other seat said its bye or its own out -- the last one's end closes the room; a seat that fell silent keeps the room; tools/mt-room-lane-check.py refuses a tree without it (the author's word 19777).
- No network or protocol change: MTGroupRoom.over remembers the rooms that ended here and bury is the one door that forgets an ended room; a late word of a room that is over is refused (room_refused), and a view that lost this phone does not bury the room it is in; tools/mt-room-lane-check.py refuses a tree without it (the author's words 19800, 19801).

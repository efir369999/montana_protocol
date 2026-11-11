1.0 (2376) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2376), from source commit `79d1024251dcd48af4a522cecb5c42f26c408fa0` of 2026-11-11. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- In a group call, a phone that comes in is connected within about a second from the group's second call on: its connection is born the moment it says it is in, on the call's own lane, without waiting for the group's message.
- A phone that leaves a group call disappears at once on every other phone: its voice stops first, its goodbye reaches the others, and only then does its connection close.
- When the owner ends a call for everyone, every phone hears it at once, on the call's own connection and on its lane.
- Phones need this build or a later one to call each other in groups: a phone on an earlier build does not connect to one on this build.
- Please check: in a group of three phones (T1, T2, T3) start a call, end it, and start a second one: the phone that joins is heard within a second. Leave the call on T2 -- its tile disappears at once on T1 and T3. The owner ends the call for everyone -- it ends on every phone.

Security and anonymity
----------------------

- Network change: a group call's open and in words on the group's road carry nq, the ML-KEM-768 public key the seat will use in its next room of that group; the group's road proves whose it is. Its secret stays sealed on the phone (MontanaLocalVault) and is spent by that one room; MTGroupRoom.roomKey and nextKey are its only owners.
- Network change: each room has its own lane, room:ID, keyed by SHA-256 over mt-room-lane and the room key. It carries presence alone (in, here), unsealed; a pair is born from it only for a seat whose key the group's road has already named, and a seat's tile waits until its pair stands.
- Protocol change: an offer carries 32 random bytes (n) sealed to its receiver; the offerer accepts an answer only when it carries them back, and a standing pair believes a bye or end on the lane only when it is sealed with them. Answers from earlier builds carry none and are refused. tools/mt-room-lane-check.py, in the build ring, refuses a tree that drops either rule.

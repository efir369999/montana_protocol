1.0 (2414) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2414), from source commit `e72752ea956e5a6bf52c95137c57fcd38ab62a2d` of 2026-11-16. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- On a slow connection -- through a tunnel or on weak cellular -- a photo or a file sent in a chat is no longer given up when the nodes take a moment longer to answer: a quiet node only calls the next one in, and a node is dropped only after its full wait.
- While the backup of your history is uploading, its extra copies for the other nodes now go one at a time and only when nothing of yours is sending -- photos, links and calls go first; a copy that fails is written to the diary with its cause.
- Please check: on cellular, send a photo right in a chat and watch it leave.

Security and anonymity
----------------------

- The backup's copies reach the same nodes as before, sealed as before; only their moment and their road on the phone change.
- The nodes a piece of a photo is offered to are the same, and in the same order; a node that has not answered is only waited for longer before it is passed over.
- Nothing of security or anonymity changes.

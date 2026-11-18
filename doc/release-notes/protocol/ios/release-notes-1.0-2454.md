1.0 (2454) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2454), from source commit `9dc057b706009ebfe881eb9e72fe7a46d6eefe34` of 2026-11-19. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- A link or text you share from another app now shows the time you sent it, not the time the app picked it up, and stands in its place in the chat.
- Every message wears its own time everywhere -- in the chat, in the list and before the app opens -- so a late arrival no longer changes its time or its place.
- In a video call your picture's quality now follows what the other person actually receives, not the losses on their own side of the line.
- A video call whose picture had to stop on a weak network now tries again by itself and brings the picture back when the network allows.
- After a hold, a reconnection or a screen share the other person's picture is shown again at once, instead of a black screen while the video arrives.

Security and anonymity
----------------------

- Anonymity is unchanged. Security is unchanged: a message's time is read from its own name, as its order already was, and a correspondent's claimed time stays bounded by this phone's clock; a call's quality reads only that call's own reports.

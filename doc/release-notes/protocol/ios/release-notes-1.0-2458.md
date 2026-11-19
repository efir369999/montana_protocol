1.0 (2458) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2458), from source commit `d5af87c83636e1f49fac8d71c71e7201ebf29d85` of 2026-11-19. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- In a video call the picture no longer goes dark by itself; on a congested cellular network the video lowers its quality instead of freezing.
- A call rings sooner: it no longer waits on a server that has just stopped answering, and during a call the app no longer keeps dialling servers that are down.
- After switching between Wi-Fi and cellular, the app keeps a separate memory of reachable servers for each network and does not carry a dead server over from the other one.
- A call through a relay checks the encrypted relay route once per network and does not lose time on it where that route is blocked.
- A muted chat, group or channel no longer counts toward the number on the app icon, the tabs or a notification; its unread count stays on its own row.

Security and anonymity
----------------------

- Anonymity improves: the call relays no longer keep a journal of callers' addresses. Security is unchanged: the relay's pass is issued by one place on each server under the same keys and term, and the encrypted relay route is unchanged.

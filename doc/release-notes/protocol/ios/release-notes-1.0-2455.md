1.0 (2455) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2455), from source commit `dfa5956186d63db1412f328925467bae1ebd0fc7` of 2026-11-19. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- A message that arrives while the app is closed is no longer pushed out of the waiting queue when many messages come at once -- it lands in its chat.
- A message you have already read in its chat no longer comes back as a new notification; an unread one keeps its notification until you open it.
- A repeated notification of a long message keeps the words and the chat already received, instead of showing it as a personal message.
- Photos and videos no longer wait on a server that cannot be reached from your network: backup copies pass it until it answers again.
- A video call that goes through a relay no longer starts at full quality on one side -- the picture does not freeze and the call does not break in its first seconds.
- After a call is put on hold and resumed, the other person's picture no longer stays black while their video keeps arriving.

Security and anonymity
----------------------

- Anonymity is unchanged. Security is unchanged: the notification extension keeps only the names of messages that already left its box, as empty files under the same protection, and the app removes them by the term a message can still return; a message's own copies still go to every server, and only backup copies pass a server at rest.

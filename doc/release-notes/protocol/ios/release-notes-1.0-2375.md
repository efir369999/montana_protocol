1.0 (2375) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2375), from source commit `4c20068e778b270f95b6a60e9226c66b020174ca` of 2026-11-11. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- To the right of the name of the Montana chat, which writes about updates, stands a blue seal with a white check, as in Messenger.
- In groups, every sender's photo now stands beside their messages on every phone, not only on the group owner's: the photo travels with the sender's own message.
- A group's photo is chosen and cropped by the same screen as your own profile photo.
- Setting a group's photo no longer sends the members an empty message or a system error message.
- In the Montana chat, the Update button stands under the message about a new version as its own button, always leading to the source of updates.
- No second chat «Recovered history» with pieces of a group's words appears beside the group after restoring the archive.
- Group system actions -- adding a member, naming an administrator, changing the group's photo or title -- no longer send notifications.
- A muted or archived chat -- a group, a channel or a person's chat -- no longer leaves notifications: a mention still reaches you.
- Please check: in a group of three phones, write from each phone -- on every phone each message shows its sender's photo. Change the group's photo -- members get no empty message. Open the Montana chat: the Update button stands under the new-version message. Mute a group on one phone and write to it from another -- no notification stays. Name a member an administrator -- nobody gets a notification.
- Please check: Settings, About shows build 2375; the Montana chat shows the blue seal beside its name in the chats list and in the chat.

Security and anonymity
----------------------

- Network change: a group letter may carry a small copy (96 by 96 pixels) of its sender's own profile photo in the field sf, encrypted as the letter itself is; it rides only when the photo changed, the group grew or a day passed, and the owner passes it on unchanged. A receiving phone keeps one photo per seat (MTGroup.speakerFace is the one answer the feed reads). tools/mt-speaker-face-check.py, in the build ring, refuses a feed that reads the photo elsewhere.
- No algorithm or key change: the Update button under the Montana chat's release message reads its one source (MTReleaseState.source), not the letter's own address; tools/mt-release-door-check.py, in the build ring, refuses a scroll without it.
- No algorithm or key change: the notification service judges a group letter by its opened text, and a group's photo change is carried silently; the unused organization chats code is removed.
- No algorithm or key change: whether a word rings has one owner, MTRing (MontanaKeychain.swift), read by the group's wire, the notification service and the app; a group's system word is sent silently, and a muted chat's notice is taken off by the notification service within its wake. tools/mt-ring-owner-check.py, in the build ring, refuses a second owner.
- No algorithm, key or network change: the seal beside the Montana chat's name (MTMontanaSealMark) is drawn by the code both editions share; Protocol carries it from this build.

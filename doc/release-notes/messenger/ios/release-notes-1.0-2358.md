1.0 (2358) Release Notes
========================

Montana for iPhone, iPad and Mac, build 1.0 (2358), from source commit `19a0b1741491def4f88a9a873e7c25f6773241b9` of 2026-11-04. The folder `apps/messenger/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Delete for everyone always works: a channel post taken away leaves every page, whatever the level.
- No download button under a post: the row under it ends with the views.
- A call is recorded whatever the other side's switch or level.
- Your own node no longer waits for level 8, and the «Access by level» texts are gone.

Security and anonymity
----------------------

- A post deleted for everyone is no longer kept on a subscriber's phone by level (MTLevelAccess.keepDeleted removed).

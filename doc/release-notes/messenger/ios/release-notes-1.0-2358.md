1.0 (2358) Release Notes
========================

Montana for iPhone, iPad and Mac, build 1.0 (2358), from source commit `7eae3e8b2f9faeffa8b5a6730ab3cc9f9f0e889a` of 2026-11-04. The folder `apps/messenger/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- Delete for everyone always works: a channel post taken away leaves every page, whatever the level.
- No download button under a post: the row under it ends with the views.
- A call is recorded whatever the other side's switch or level.
- Your own node no longer waits for level 8, and the «Access by level» texts are gone.

Security and anonymity
----------------------

- A post deleted for everyone is no longer kept on a subscriber's phone by level (MTLevelAccess.keepDeleted removed).

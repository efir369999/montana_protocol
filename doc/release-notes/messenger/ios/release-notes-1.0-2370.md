1.0 (2370) Release Notes
========================

Montana for iPhone, iPad and Mac, build 1.0 (2370), from source commit `025ddbfdcaae1f942b308462664d9a3e91825034` of 2026-11-11. The folder `apps/messenger/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- The Montana Protocol icon fills the whole icon on the home screen: the white frame and the square inside the square are gone, the coin stands at full size.
- The pages of signing in and creating an identity stand on black, and every word on them is white and larger, so everything reads easily: no grey text from the first screen to the chats. Black is the default background.
- Please check: the home screen icon and the icon in Settings; create a new identity and read every page up to the chats; chats, groups, calls and Settings as usual.

Security and anonymity
----------------------

- No algorithm or key change. The app icon's picture was replaced (tools/delivery-policy.json pins the new blob and retires the old one); a new guard, tools/mt-icon-fill-check.py, refuses an app icon that does not fill its canvas.
- The pages of signing in and creating an identity change only their colours and type sizes (the author's word 18938); the default ground is black; tools/mt-birth-ground-check.py refuses a grey word on that path.

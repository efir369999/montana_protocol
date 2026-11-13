1.0 (2388) Release Notes
========================

MT Protocol for iPhone, iPad and Mac, build 1.0 (2388), from source commit `b9b212d4fa30db36027017e91d3bb326d8445c4e` of 2026-11-13. The folder `apps/protocol/ios` of this repository holds that tree as it is built.

Notable changes
---------------

- One build number for Messenger and the Protocol: both are 2388, built from the same code.
- Settings, Privacy: choose who may message you, call you, see your page and add you to groups and channels -- Everyone, My contacts or Nobody. Your phone obeys the choice.
- Settings, Appearance: the time panel at the top or at the bottom of the screen. Messenger starts with it at the bottom, the Protocol at the top.
- Each page shows only its own kind: Chats only personal chats, Groups only groups, Channels only channels.
- The time panel is the same in the Protocol and in Messenger: Chats, Groups and Channels right of the logo; the globe is gone, and the logo's green ring says the phone is connected to the network. Music, the gallery and the network open from the side panel.
- Please check: in Privacy set Who can message me to My contacts, and ask someone not in your contacts to write -- the letter does not arrive.
- Please check: in Appearance switch the time panel to the other edge -- it moves, and the chats run under it.

Security and anonymity
----------------------

- Who may reach me (MTPrivacy): the person's four rules -- writing, calling, the page, adding to a group or a channel; Everyone, My contacts or Nobody -- are kept in this device's shared keychain and obeyed on the receiving phone alone: the letter's door, the store, the live call and the voip ring, the page's words, a group's new invitation and the notification extension's banner. The rules never leave the device; no new word on the wire.
- A group's invitation to a call rings only within its moment, in the app and in the notification extension alike (one owner, MTRing). No change to the wire format.
- A chat is born only by a letter: meeting a person by their card no longer writes a list record. No change to the wire format.
- A group's member is reached by one road from the feed's face and the page's row (MTGroupPage.way): by the pipe held, or by the member's permanent card the group already carries. The time panel's changes touch no data and no wire.

# Changelog

Every build of MT Protocol for iPhone, iPad and Mac, newest first. The version is the one Settings shows: 1.0 (build). Each entry is written here by the build road before the build is made, from the release notes the build carries; a build whose entry is not on this page is not built. Each build's full notes are in [`doc/release-notes/protocol/ios`](doc/release-notes/protocol/ios).

## 1.0 (2379) - 2026-11-12

Source commit `310cfb4b4b54`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2379.md).

- On an iPad, which has no earpiece, a call no longer shows a Speaker button that could not be turned off: the sound always plays through the iPad's own speaker, and with headphones or Bluetooth the Audio button and its menu appear as on the iPhone.
- Please check: on the iPad make a call and a group call -- there is no Speaker button, the row of buttons closes up; connect headphones -- the Audio button appears.

**Security and anonymity**

- No network or protocol change: MontanaAudioRoute.hasEarpiece is the one place that knows whether the device has an earpiece; MontanaAudioRoute.Face.shown hides the loudspeaker switch where there is nothing to switch (decision 861 = a, the author's word 20038).

## 1.0 (2378) - 2026-11-12

Source commit `f1778d7fa2c1`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2378.md).

- A group call that everyone else has left ends on the last phone too: it no longer stays open with nobody in it.
- A group call that has ended no longer comes back in the group as an empty call: a late message of a participant who already left cannot open it again, and leaving such a call no longer leaves it hanging.
- Please check: in a group call of T1, T2 and T3 leave the call on T1, then on T2 -- the call ends on T3 at once.
- A phone that leaves a group call and comes back into the same call within half a second connects again: the closing of its earlier connection no longer takes the lane of the new one.
- Please check: in a group of three phones (T1, T2, T3) leave the call on T2 and come back into it at once -- T2 is heard again on T1 and T3.

**Security and anonymity**

- No network or protocol change: MTGroupRoom.close closes a pair's lane only when no other living pair of the room in hand stands on it; tools/mt-room-lane-check.py refuses a tree without that rule (critic 19744).
- No network or protocol change: MTGroupRoom.endIfLeftAlone ends a call (not a stream) for the last seat once every other seat said its bye or its own out -- the last one's end closes the room; a seat that fell silent keeps the room; tools/mt-room-lane-check.py refuses a tree without it (the author's word 19777).
- No network or protocol change: MTGroupRoom.over remembers the rooms that ended here and bury is the one door that forgets an ended room; a late word of a room that is over is refused (room_refused), and a view that lost this phone does not bury the room it is in; tools/mt-room-lane-check.py refuses a tree without it (the author's words 19800, 19801).

## 1.0 (2376) - 2026-11-11

Source commit `79d1024251dc`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2376.md).

- In a group call, a phone that comes in is connected within about a second from the group's second call on: its connection is born the moment it says it is in, on the call's own lane, without waiting for the group's message.
- A phone that leaves a group call disappears at once on every other phone: its voice stops first, its goodbye reaches the others, and only then does its connection close.
- When the owner ends a call for everyone, every phone hears it at once, on the call's own connection and on its lane.
- Phones need this build or a later one to call each other in groups: a phone on an earlier build does not connect to one on this build.
- Please check: in a group of three phones (T1, T2, T3) start a call, end it, and start a second one: the phone that joins is heard within a second. Leave the call on T2 -- its tile disappears at once on T1 and T3. The owner ends the call for everyone -- it ends on every phone.

**Security and anonymity**

- Network change: a group call's open and in words on the group's road carry nq, the ML-KEM-768 public key the seat will use in its next room of that group; the group's road proves whose it is. Its secret stays sealed on the phone (MontanaLocalVault) and is spent by that one room; MTGroupRoom.roomKey and nextKey are its only owners.
- Network change: each room has its own lane, room:ID, keyed by SHA-256 over mt-room-lane and the room key. It carries presence alone (in, here), unsealed; a pair is born from it only for a seat whose key the group's road has already named, and a seat's tile waits until its pair stands.
- Protocol change: an offer carries 32 random bytes (n) sealed to its receiver; the offerer accepts an answer only when it carries them back, and a standing pair believes a bye or end on the lane only when it is sealed with them. Answers from earlier builds carry none and are refused. tools/mt-room-lane-check.py, in the build ring, refuses a tree that drops either rule.

## 1.0 (2375) - 2026-11-11

Source commit `4c20068e778b`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2375.md).

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

**Security and anonymity**

- Network change: a group letter may carry a small copy (96 by 96 pixels) of its sender's own profile photo in the field sf, encrypted as the letter itself is; it rides only when the photo changed, the group grew or a day passed, and the owner passes it on unchanged. A receiving phone keeps one photo per seat (MTGroup.speakerFace is the one answer the feed reads). tools/mt-speaker-face-check.py, in the build ring, refuses a feed that reads the photo elsewhere.
- No algorithm or key change: the Update button under the Montana chat's release message reads its one source (MTReleaseState.source), not the letter's own address; tools/mt-release-door-check.py, in the build ring, refuses a scroll without it.
- No algorithm or key change: the notification service judges a group letter by its opened text, and a group's photo change is carried silently; the unused organization chats code is removed.
- No algorithm or key change: whether a word rings has one owner, MTRing (MontanaKeychain.swift), read by the group's wire, the notification service and the app; a group's system word is sent silently, and a muted chat's notice is taken off by the notification service within its wake. tools/mt-ring-owner-check.py, in the build ring, refuses a second owner.
- No algorithm, key or network change: the seal beside the Montana chat's name (MTMontanaSealMark) is drawn by the code both editions share; Protocol carries it from this build.

## 1.0 (2374) - 2026-11-11

Source commit `38ab92e36910`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2374.md).

- Switching to the emoji keyboard while typing no longer covers the input field: the field and the messages rise to the top of the emoji keys at once, and come back down when you switch to letters.
- The build's number now lives in one place, so every part of the app carries the same number by construction.
- Groups and Channels are in the side panel; the strip of folders under the time panel on the chats page is gone (as in 2373).
- Please check: Settings, About shows build 2374; tap Groups and Channels in the side panel.
- Please check: open a chat, tap the field, switch to the emoji keyboard and back several times -- the field always stands right above the keys; then type a word with emoji and send it.

**Security and anonymity**

- No algorithm, key or network change: the build number and version moved from 22 target settings of the project into Version.xcconfig, read by every target.
- No algorithm, key or network change: the chat page also listens to the system's change of the input mode (UITextInputMode.currentInputModeDidChangeNotification) and places the input bar on the keys' top read from the keyboard's own accessory; tools/mt-keyboard-drag-check.py, now in the build ring, refuses a rider without it.

## 1.0 (2373) - 2026-11-11

Source commit `f7cb3e8092fb`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2373.md).

- Groups and Channels open from the side panel, each on its own page. The strip of folders under the time panel on the chats page is gone.
- A swipe on the chats page turns the time panel's pages, as on every page.
- Settings no longer has Chat Folders: the folders' page is gone with the strip.
- Please check: open Groups and Channels from the side panel; swipe left and right on the chats page.

**Security and anonymity**

- No algorithm, key or network change: the chats page's folder tabs, their swipe and the folders' page in Settings are removed; the kept folders are only read, by the Groups and Channels pages.

## 1.0 (2371) - 2026-11-11

Source commit `d600afe7e451`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2371.md).

- Nothing changes on the phone. This build's source is published in its own folder of the public repository, apps/protocol/ios, under the Protocol's name; the Messenger keeps its own repository.
- Please check: chats, groups, calls and Settings as usual.

**Security and anonymity**

- No algorithm, key or app code change. The public source moves from apps/messenger/ios to apps/protocol/ios in the same commit, so nothing is published twice; the publisher's scan reads the old folder's published pages as already shown and still refuses anything new.

## 1.0 (2370) - 2026-11-11

Source commit `9e0d4b150f3c`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2370.md).

- The Montana Protocol icon fills the whole icon on the home screen: the white frame and the square inside the square are gone, the coin stands at full size.
- The pages of signing in and creating an identity stand on black, and every word on them is white and larger, so everything reads easily: no grey text from the first screen to the chats. Black is the default background.
- Please check: the home screen icon and the icon in Settings; create a new identity and read every page up to the chats; chats, groups, calls and Settings as usual.

**Security and anonymity**

- No algorithm or key change. The app icon's picture was replaced (tools/delivery-policy.json pins the new blob and retires the old one); a new guard, tools/mt-icon-fill-check.py, refuses an app icon that does not fill its canvas.
- The pages of signing in and creating an identity change only their colours and type sizes (the author's word 18938); the default ground is black; tools/mt-birth-ground-check.py refuses a grey word on that path.

## 1.0 (2369) - 2026-11-11

Source commit `76d57be365ea`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2369.md).

- A new icon: Montana Protocol wears its new picture on the home screen, in the icon chooser and on the first screen; the round glass version stands in Settings and in the room.
- The nodes say honestly when their disk is short: a node whose storage has no room left refuses a new attachment at once and names it beforehand, and the app sends the piece to the next node without waiting. Nothing the app does has changed with it.
- Please check: the home screen icon, the icon chooser in Settings, the first screen; send a photo and a voice message; chats, groups, calls and Settings as usual.

**Security and anonymity**

- No algorithm or key change. The app icon's pictures were replaced (tools/delivery-policy.json pins the new blobs and retires the old ones). The node store answers /blob-put with 507 when its shelf has no room and lays nothing, and /health names it beforehand (blob_room false while blob stays); the phone already treats any refusal as the next door. Door machines no longer carry a copy of the code.

## 1.0 (2368) - 2026-11-11

Source commit `d1377b726272`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2368.md).

- Voice messages under the lock: the controls of a locked recording no longer vanish. They now follow the recording itself, not the piece of the bar they were drawn from, and a moment after the lock the app checks that they stand on the screen and can be pressed. The cross, the play and the pause are one size, on one line inside the recording's bubble, with even steps between them, and each wears the blue ring.
- Photos and voice messages leave at once: what you send no longer waits in line behind the app's own background uploads -- the pages of the wall carried to its visitors among them, and a piece no longer stands for up to a minute at a node that takes no byte -- it goes to the next one. An attachment is laid on every node, including one that was resting a moment ago; and when the receiver reports an attachment lost, it is sent again once and a second check no longer takes the repair away.
- A call that reaches the phone late -- for example after it had no data during a cellular call -- now shows who called and leaves one missed call in the phone's list instead of a nameless failed call.
- Please check: record a voice message, lock it, turn the screen and back, press pause, play, the cross and send; send a photo and a voice message after the app has been in the background; chats, groups, calls and Settings as usual.

**Security and anonymity**

- No algorithm or key change. A call envelope is still opened only by the pipe's key of a minute; a wake of another minute is also tried at the minute the node stamped it -- the stamp chooses the minute, the seal still proves the sender, and the call's own age still buries a stale call (one missed line for one call). MTBulkPuts gives each cargo places of its own and the person's put the way before the app's quiet puts; MTPutPulse reads a node's silence from the bytes sent; a resting node is asked last instead of being left out, so a copy reaches every node; the sender's cargo check asks one question for a letter at a time and its verdict names the letter by its birth. A wall page's long body is quiet cargo; two diagnostic words were renamed; the labels keys are derived from are unchanged, so every existing key, chat and copy stays readable.

## 1.0 (2367) - 2026-11-10

Source commit `c4ad41da9fd3`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2367.md).

- The voice button answers the finger at once: the hold is counted from the moment the finger lands, not from the system's later recognition, so a short hold no longer turns into a tap that swaps the microphone for the camera. The reply at the ear after listening records again: the phone counts the ear as near if the sensor saw it within the last second and a half, and every reason a reply did not start is written down. The diary of the device is never thrown away: the node keeps every line or answers that it could not, and the phone keeps its lines until they have landed. The notice that someone keeps a sealed part of their copy on this phone is gone; Settings explains the copy with contacts in plain words.
- The VPN tunnel rises by the hand alone: no profile of ours carries On Demand any more, so a tunnel that ended -- by the hand or because the extension died -- stays down until the switch is touched again, and every end is shown; the background supervisor and the Mac's launch agent that used to raise it are gone.
- A call reaches the other phone through the nodes' line as well: every call word now also rides the standing connection to the nodes, the other side answers the receipt by the same line, and when the nodes' web doors are dead the call follows the line instead of beating on them -- the call to the iPhone 17 that never rang on 09.11 at 16:47 UTC went this way.

**Security and anonymity**

- No change to any algorithm, key or network road. MontanaHoldControl arms on hitTest and runs the 190 ms mode timeout from the touch; the recogniser's .began only confirms the finger and writes its lateness (hold_press late=). MTProximity keeps the sensor's last «near» (nearWithin); the voice_end and voice_ear_reply diary lines carry armed/near/recent flags and a reason word, no content. MontanaLog keeps 24 finished diary files instead of 6; MontanaDiagShip reads the node's answer (ok, lines, dropped) and keeps the lines on the phone unless every line landed. The nodes (montana-node-store.py, montana-home-node.py) drop the per-device day cap and the ring; a refused disk answers 507, never ok. MTKeeping no longer posts the «Keeping a copy» local notification and no longer imports UserNotifications; the two Settings footers are reworded; four catalogue keys leave and two arrive in en, ru and zh-Hans. MontanaVPNTunnel.noReconnect is the one owner of «no On Demand»: every profile save passes through it, liftReconnect lifts an older build's rule at first load, standing(m) replaces isOnDemandEnabled as the one meaning of «the VPN is on»; the only start of a session is start(); the extension writes one mark into the profile (ExtDeadMark); MTVPNSupervisor, MTVPNRecoveryHooks, MTMacVPNService and the Mac launch agent leave the tree (copies in CouncilWall/archive/vpn-reconnect-2026-11-09/); tools/mt-vpn-recovery-check.py holds the rule. MTCallRoute gains the door «line» beside «near» (isPipe): callRoute lists it last while a node channel stands, a copy of every packet rides it (echoDoors), a receipt by it turns the route onto it, and nodeOnly words (rejoin, a dead seed's end) avoid both pipe doors (avoidPipe). MontanaDirect.sendOnStandingLine lays the sealed envelope under the peer's pipe label on the standing channels to the nodes alone -- no dial, no peer channel, no new key or frame type; labelTarget is the one shape of the label target. MontanaPhoneNode.sendLetter(standingLineOnly:) carries it; ChatStore names the door of a pipe word (near while a local connection stands, else line) so the receipt returns the same way. The diary gains line_tx and sent_route route=line, counts only. tools/mt-call-route-check.py exercises the line vectors. The sentence a permission asks with has one owner: the English stands in Code/Montana/Info.plist alone (no en.lproj repeats it), and the Messenger's Info.plist and InfoPlist.strings are written from the Protocol's by tools/mt-messenger-strings.py; tools/mt-ssot-check.py runs in the build ring. The text a person reads does not change.

## 1.0 (2365) - 2026-11-09

Source commit `06cca42c988b`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2365.md).

- The voice button is rebuilt so the press reaches it: on iOS 27.0.1 a press on the microphone did not reach the button at all, so holding it recorded nothing. The press is now taken by the button's own platform recogniser, and its whole area is declared touchable.

**Security and anonymity**

- No change to any algorithm, key or network road. MontanaHoldControl tracks the press through a UILongPressGestureRecognizer (minimum duration 0, cancels and delays nothing) instead of UIControl's beginTracking, with the same thresholds and verdicts; the slot carries contentShape(Rectangle()). The diary on the device gains hold_hit (the point and the enabled flag where the platform hands a finger to the button) and hold_down by=press -- coordinates only, no content.

## 1.0 (2364) - 2026-11-06

Source commit `0a3ed5688699`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2364.md).

- Music keeps its word with the car and the headphones: when a call, Siri, a navigator or a car's Bluetooth takes the sound, the track pauses where it stands and the bar says so; when the sound is given back with «resume», the track plays on from its place. A headset or a car that goes away pauses the track instead of playing it aloud into the room.

**Security and anonymity**

- No new server, network road or key leaving the device. The diary on the device (MontanaTrace) gains music_remote (the lock screen's, headset's or car's button), music_interrupt (began/ended, resume asked, played on) and music_route (an output device came or went, its port type) -- naming no track, file or folder; play_refused resume from the previous build stays.

## 1.0 (2363) - 2026-11-06

Source commit `9a67c720f5c4`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2363.md).

- The music's diary on this device tells a track that ran to its end from one the phone could not read to its end: a track that falls silent early is named by the moment it stopped and its length, so its cause can be found and mended.

**Security and anonymity**

- No new server, network road or key leaving the device. The diary on the device (MontanaTrace) gains three marks -- music_end (the player's clock, the bar's tick, the length), sound_end broken and sound_decode (the platform's error code) -- naming no track, file or folder.

## 1.0 (2362) - 2026-11-06

Source commit `6e2b29ca7c9c`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2362.md).

- The music page shows only tracks that play: the tracks of a lent folder that did not open stay out of the list, the queue and the shuffle until the folder opens again or is lent again with the plus, and the folder lent again takes its tracks back in their places.
- Passwords leave the app: the Passwords tile and page are gone, a secret letter sent by an older build is no longer shown, and the passwords vault is erased from this device's keychain at launch.

**Security and anonymity**

- No new server, network road or key leaving the device. A lent folder that does not open is told in the diary by its cause (MTMusicFolders.open: the bookmark's refusal or the platform's), naming no folder; its tracks stay in this device's own list (Application Support, excluded from backup) and the music list is written only when it changes.
- No new server or network road. The passwords vault leaves both editions (decision 671 (a)): its keychain item is known by print and deleted at launch (MontanaKeychain.forgetRetired, decision 673 (a)) and never restored from a seat; a secret letter of an older build is a retired kind, buried unread and ringing nothing (MTRetired.letterLines, decision 668 (a)).

## 1.0 (2361) - 2026-11-05

Source commit `a2913264003b`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2361.md).

- Each app opens by its own link scheme: Montana by montana://, MT Protocol by mt-protocol://. A Montana link in a letter opens inside the app where you tap it, and an invitation still opens in either app.
- The app has one name everywhere: the permission questions no longer name it (the system already shows its name above them), and its iCloud folder is called MT Protocol, as under the icon.
- The call screen no longer freezes in the first call after the app starts: the mute button answers, the video request is shown to the caller, and the video comes on for both sides once the other side agrees. The screen now proves it was laid out again after every change and is born again at once when it was not.

**Security and anonymity**

- No new server, network road or key leaving the device. The two schemes are stated once (Code/Names.xcconfig: MT_SCHEME_PROTOCOL, MT_SCHEME_MESSENGER) and checked by tools/mt-names-check.py; a montana:// link tapped in a text goes to the app's one entrance of a link (MontanaMeeting.handleLink) and never leaves the app by the system's round.
- No new server, network road or key leaving the device. Each product's name is stated once (Code/Names.xcconfig: MT_NAME_PROTOCOL, MT_NAME_MESSENGER); the iCloud folder reads it, no purpose string names an app, and tools/mt-names-check.py (rules 8 and 9) holds the store page and the site to the same owner.
- No new server, network road or key leaving the device. The call window's host (MTCallHost, Code/Montana/MontanaCall.swift) counts its layouts; CallWindowPresenter raises a generation at every change of the call model, the screen's tree echoes it from inside its geometry, and a generation not echoed within 0.3 s is written to the diary (call_layout stall) and the window is born again, twice a call at most (call_window reborn).

## 1.0 (2360) - 2026-11-04

Source commit `5666090ffadb`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2360.md).

- Names with @ are gone: no public @name, no search or link by @name, and the people you add are no longer written into your phone's Contacts. A contact is shared through the system's own picker, so Montana asks for no access to Contacts.
- Adding a person speaks only of your card: people reach you by its QR code or link, and the first screen of each app says what it is.
- Video in a voice call: the asker sees «Video request sent»; once the other person accepts, video comes on at both ends and stays until each turns off their own camera; turning your camera off never stops the other person's video, and a yes that arrives after a long wait still turns the video on.

**Security and anonymity**

- No new server, network road or key leaving the device. The keeper of names is asked nothing and no @name is published; the app holds no Contacts access (NSContactsUsageDescription is gone from Info.plist, and tools/mt-apple-rules-check.py refuses it back), and the book of others' @names and the cards it wrote are erased on the phone (MTNameBook.forgetClaimedNames).
- The video consent stays a word between the two phones (call-video-ask, call-video-ok, call-camera-off): no new server, road or key; a yes is taken only within 120 s of one's own ask, and turning one's camera off rests only one's own capture.

## 1.0 (2359) - 2026-11-04

Source commit `ada594f9dc02`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2359.md).

- Call recording is gone: no switch in Settings, no record mark on the call screen, and what an earlier build left of it on this phone is removed. A recording already sent to a chat stays in that chat.
- Your node is gone from Settings. Files a node already holds still come back when you open them.
- Turning video on during a voice call no longer stalls: requests that cross are settled at once, a phone already in video answers by itself, and the Video button shows the other camera joining.
- The Montana room with updates stands for everyone: the build in your hands is its first row, even before the network answers.
- The Update button appears only when a newer build is out.
- The copy kept by the people you write to is laid only on the charger, outside Low Power Mode, with the phone cool and not on a metered network: no more heat on the battery.
- Asking the nodes which files they hold goes in slices of 900 names, so a large library is no longer refused.

**Security and anonymity**

- No new server, network road or key leaving the device. Call recording is removed from the code: no recorder, no recording switch in a call's capabilities; the leftover unfinished files and the two settings keys are erased on the phone (ChatStore.sweepCallRecordings). The own-node row is drawn in no edition (MTEdition.ownNode false) and the node sends nothing.
- No new server, network road or key leaving the device. A video request is answered from the call's own state (CallUIModel.videoJoinPending, a 40 s join window); no new message kind.
- No new server or key leaving the device. The update document is read by the app's own bundle (updates/BUNDLE-CHANNEL.json) from montana.xxx and the nodes, as before; the app accepts only a document naming its own bundle. The keeping copy now waits for charger, cool phone and an unmetered path (MontanaPower.bulkHeld).

## 1.0 (2358) - 2026-11-04

Source commit `48ccc9de8f07`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2358.md).

- Delete for everyone always works: a channel post taken away leaves every page, whatever the level.
- No download button under a post: the row under it ends with the views.
- A call is recorded whatever the other side's switch or level.
- Your own node no longer waits for level 8, and the «Access by level» texts are gone.

**Security and anonymity**

- A post deleted for everyone is no longer kept on a subscriber's phone by level (MTLevelAccess.keepDeleted removed).

## 1.0 (2355) - 2026-11-03

Source commit `fcdeb9ba7a4d`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2355.md).

- The name under the icon on the home screen is MT Protocol; the app stays Montana Protocol on the App Store.
- One icon: the Montana coin fitted to the icon shape. The alternate icons are gone; a phone that had one returns to the coin.

**Security and anonymity**

- Only the name under the icon and the icon change; the app identifier, its data and its keys stay as they were.
- No new server, network road or key leaving the device.

## 1.0 (2352) - 2026-11-03

Source commit `b12a68ed6373`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2352.md).

- The app is now called Montana Protocol on the home screen, in the share sheet, in iCloud storage and in its terms -- the same name as on the App Store.
- A new icon: the Montana coin.

**Security and anonymity**

- Only the name a person sees and the icon change; the app's identifier, its data and its keys stay as they were.
- No new server, network road or key leaving the device.

## 1.0 (2351) - 2026-11-02

Source commit `63de00ad1b6b`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2351.md).

- A long message that arrives while the app is open appears with its words at once; it no longer hangs as «New message» with a spinning wheel until you open the chat again.

**Security and anonymity**

- A long letter's reference that comes over the live channel is opened by the same road as a pushed one (the sealed cargo fetched by its own key inside the end-to-end envelope) before any row is written; a reference whose cargo is not yet reachable waits in the app's own letter box, never as a row.
- No new server, network road or key leaving the device.

## 1.0 (2350) - 2026-11-02

Source commit `cfa3a6742919`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2350.md).

- The feed's motion is one for everyone: the springiness and animation duration an older build let you set no longer apply, and they leave the phone at launch.
- Signing in and creating an identity stand on one indigo background from the first page to the last; it no longer changes along the way.
- Montana's messages about new builds reach every phone again: a phone whose App Store check stayed silent now reads its channel from the receipt Apple wrote.
- The balance shows coins by default; the choice of its main sum starts anew.
- No wallet or VPN icons under the words Pantheon on Fire, and no 0-60 s line under it.

**Security and anonymity**

- Three stored display values leave the phone at launch by their prints (chatSpringBounce, chatSpringDuration and the balance's main sum, 5da1d072bfa76c8b) and no copy carries them any more.
- When StoreKit's AppTransaction does not answer, the release channel is read only from the name of the app's own receipt file (sandboxReceipt = beta, receipt = store, anything else = no channel); nothing is sent, and a store build never points at a TestFlight link.
- No new server, network road or key leaving the device.

## 1.0 (2349) - 2026-11-02

Source commit `6e9f03a4389e`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2349.md).

- Leaving the registration midway no longer leaves an identity behind: until the profile's checkmark the making is unfinished, and the next launch forgets it.
- The registration has no step of choosing the page's ground any more: a new identity is born on the indigo, and Appearance changes it later.
- GIFs play without stopping; a tap stops one and a tap plays it on. Appearance gains Reduce GIF animation, which brings back three plays and then only on a tap.
- In Appearance the app icons scroll and take the tap, the feed's motion section is gone, and Voice message style stands higher.
- Your node is no longer in the Settings list: it opens from Access by level, level 8.

**Security and anonymity**

- Two new stored values, both on this phone: gifReduceMotion (a display preference, carried in the person's copy as other looks), and mt.birth.unfinished (the SHA-256 print of the words being made, never the words, kept only from Create to the profile's checkmark and never in a copy). No new server, network road or key leaving the device.

## 1.0 (2348) - 2026-11-02

Source commit `842496231721`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2348.md).

- Time Coins now says one thing about coins: one coin is one second. The Minting section with its other sources (rights from the wall, coins from chats, the Pantheon, the VPN wall, reactions, burned, received and sent) is gone, and the badge over the coin shows the level alone.
- The TimeChain page names the chains of earlier builds as one line, Earlier builds.

**Security and anonymity**

- Only screens and their texts change: no new stored value, server, key leaving the device or network road. The coins' chain and its fold are untouched.

## 1.0 (2347) - 2026-11-02

Source commit `386c007160f1`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2347.md).

- When you create an identity, a tap on a background now applies it to the page at once: no preview and no extra confirmation. The same goes for a photo from your library. The checkmark then goes on to create the identity. Choosing the background later in your profile still shows the preview.

**Security and anonymity**

- The background chosen while creating an identity is stored on the device exactly as a background chosen in the profile. No new stored value, server, key leaving the device or network road.

## 1.0 (2345) - 2026-11-02

Source commit `403d039f8ec2`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2345.md).

- The app no longer freezes and closes when it goes to the background. During a call, photos, videos, GIFs and Wall of Thoughts posts now send while the call is connected; they wait only while the call is ringing or connecting. Coins that did not leave because the key's node was busy for the minute are now sent again automatically for up to a minute, and if they still cannot go you are told why and nothing leaves your balance. On the Wall of Thoughts you can like, keep and repost a post even without a chat with its writer, a long press on the like shows who liked it, a long press on the eye of your own posts shows who saw them, and a tap on the writer's name opens their page. A repost now stands under your name, with the original's wall noted below. The side panel's background now reaches the bottom of the screen.

**Security and anonymity**

- A post on the public Wall of Thoughts table now carries the @name its writer holds, which is already public and searchable; a lapsed name is not carried. A page sent to a person who holds a chat with the wall's owner carries the names of up to fifty people who liked each post, as those people named themselves; who viewed a post never leaves the owner's phone. Reacting to a table post without a chat opens a chat by that @name through the existing meeting road. No new stored value, server, key leaving the device or network road.

## 1.0 (2342) - 2026-11-01

Source commit `0e1d9b73dbee`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2342.md).

- The phone stays cooler and the app opens lighter: at launch it no longer re-sends wall pages to visitors who never came for the last one (they get the page when they reappear), no longer lays out the music list until you open the music page, remembers which parts of the coin book it has already opened so a launch fetches only the new ones, and no longer keeps redialling addresses that only its own tunnel answered.

**Security and anonymity**

- No new stored value, address, server, key leaving the device or network road. One new local file (coin-vault-pieces) keeps only the names of coin-book pieces already fetched, under the same protection as the book, and leaves with the account; wall pages, music and dials are the same roads, started later or less often.

## 1.0 (2341) - 2026-11-01

Source commit `82054e1d5dc8`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2341.md).

- Chats are fresh sooner after launch: the app reads its mailbox the moment it starts instead of waiting a fixed two and a half seconds, and a read that comes before the network's doors have answered -- or before the account's reference is ready -- is kept and goes the moment they are, never dropped.

**Security and anonymity**

- No new stored value, address, server, key leaving the device or network road. The same mailbox read starts earlier and repeats only once the doors already in use have answered; nothing else changed.

## 1.0 (2340) - 2026-11-01

Source commit `7ed684ede6bd`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2340.md).

- Cooler phone: a quick pass through the feed on the way to another tab no longer wakes the phones of everyone in it, and a message arriving while the app is open no longer makes it read the whole mailbox again.
- Faster start and less heat: the chat history is no longer rewritten in full (several megabytes) at every launch, an incoming name no longer rewrites the names list on the main thread, and the clean-up of old stored files runs once a minute instead of once per write.
- Steadier connection, less heat: the app finds the network's doors once and holds them; it no longer starts a new search for every request while one is under way (seventeen searches in five seconds were measured), a change of network or VPN starts one fresh search on the new network, and the first mailbox read at launch waits for the first door that answers instead of giving up.
- Less heat while the feed comes in: your wall is saved in the background, not on the main thread, and in the order it changed.
- Cooler phone between wakes: the app reads the whole mailbox (four days) at most once a minute and, between, only today's letters -- one page instead of four. It still reads everything at launch, when you pull the list down, when the network says a letter waits, and when a new chat opens.

**Security and anonymity**

- No new stored value, address, server, key leaving the device or network road. The feed now sends fewer wall asks and a push into an open app reads the mailbox through its existing gate; stored files no key names any more are still removed no sooner than a minute after their key moved on.
- No new address, server or stored secret. The door map stores the same machine list as before, now in one fixed key order; a network change only re-asks the doors already known.
- No new stored value, address, server or key leaving the device. The wall is encoded, sealed and stored on one serial background queue (montana.board.writes) in the same keys and under the same seal as before; clearing, parking or forgetting a person waits for the writes on their way first.
- No new stored value, address, server or key. A pickup between the whole reads asks the same labels for fewer days (MTNodeWire.freshDays, always inside the box term); a letter sealed under an older day by a sender whose clock is off by more than an hour waits for the next whole read, at most a minute while the app runs. The notification extension still reads the whole term.

## 1.0 (2336) - 2026-11-01

Source commit `d039af024dca`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2336.md).

- Faster and cooler start: when nothing in your chat archive has changed since the last launch, the app no longer reads and decrypts the whole archive again at every start -- it checks the size and time of each chat's log and moves on.

**Security and anonymity**

- One new value on this device: mt.archive.face, the SHA-256 of each chat folder's log size and last-write time with the feed's chat names, the pipe count, the asked-row count and the recovery day, kept in the app's own defaults, scoped to the seated seed (SeedScope) and erased with it. It holds no message, key or address in clear and never leaves the device. It is computed only when the archive's keys are present, so an archive that could not be opened is never remembered as read. No new address, server, key leaving the device or network road is added.

## 1.0 (2335) - 2026-11-01

Source commit `e46ce11cc870`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2335.md).

- The chats scroll at once after the app opens: the key is no longer derived from your words at every launch -- it is derived once and kept on this device.
- Opening the chats no longer builds the music and the feed pages behind them; a page is built when you look at it, and the music library is gathered off the main thread.
- Less heat: your wall's posts for the shared table are prepared in the background, not while the chats open.

**Security and anonymity**

- The master seed stretched from the recovery words (2^20 iterations) is now kept in the keychain beside the words, under the same class (AfterFirstUnlockThisDeviceOnly: this device only, never synchronized, never in a backup), bound to the SHA-256 of the seated words, and erased with them (clear, a new identity, forgetMasterSeed). It is the same secret as the words in another shape. Every reader asks one owner (MontanaSeed.root); a guard (tools/mt-root-owner-check.py) refuses a second derivation site. No new address, server, key leaving the device or network road is added.

## 1.0 (2330) - 2026-11-01

Source commit `d0886d400527`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2330.md).

- The chats open faster: a conversation shows its newest twelve messages at once and lays the rest a moment later, and the video-note camera starts only when you record, not when you open a chat.
- Less heat: the music library is walked only when you open it, not at every launch; the history is written chat by chat, only the chat that changed, and only what was written is read back.
- Messages that arrive while the app is closed wait as one sealed file each, so the app reads only the new ones when it opens.

**Security and anonymity**

- Letters kept by the notification extension move from one keychain item to one file per letter in the app group's folder, each sealed with AES-GCM under a key born once in this device's keychain (this device only) and bound to the file's name; the old item is moved once and emptied. The history is stored as one sealed part per chat. Main-thread and disk budgets are written only to the device's own logs. No new address, server, key leaving the device or network road is added.
- Issue #136: the wallet now calls the core's join door (mtc_machine_join) from its background thread when the phone holds no head, walking from two anchor proposals the build would carry; this build carries no anchor, so no phone walks yet. No new address, server, key kind or network road is added.

## 1.0 (2325) - 2026-10-30

Source commit `d13e6baf45f2`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2325.md).

- The Time Balance rises again every second while you are in the network: the app is open or the Montana VPN is live. The available balance moves only when the core confirms the seconds at the close of a window.
- The core inside the app now carries the door through which a phone that holds nothing joins the running chain from two consecutive final proposals and checks every certificate itself. The app does not open this door yet.

**Security and anonymity**

- Issue #20 (Protocol-13 item 9): the core framework of the client is built from montana_messenger main 2f5a1a2 and carries mtc_machine_join, the authenticated join that judges every certificate from an anchor and refuses a snapshot certified only by its own announced root. No new address, server, key kind or network road is added.

## 1.0 (2324) - 2026-10-30

Source commit `d20b2d66826d`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2324.md).

- When your contact's phone renews the secret of your private conversation, yours follows: it takes the renewal letters, checks them and opens the messages sealed under the new key. This phone does not start a renewal of its own yet.
- The offer to update is now a small notice over the screen instead of a full page: it hides by itself after nine seconds, or close it with the cross in its top right corner. A touch on it opens the update.

**Security and anonymity**

- The heal of a pipe (issue #75, Montana Network) is received and opened as the Canon defines it: the phone checks its computation of the renewed keys against the Canon's frozen vectors at start and opens nothing under a computation that disagrees. It sends no heal of its own until the devices of one seed carry one heal between them (the Author's decision 416 a). No new address, server, key kind or network road is added.

## 1.0 (2323) - 2026-10-30

Source commit `a007fbba7fbf`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2323.md).

- When your contact's phone renews the secret of your private conversation, yours follows: it takes the renewal letters, checks them and opens the messages sealed under the new key. This phone does not start a renewal of its own yet.

**Security and anonymity**

- The heal of a pipe (issue #75, Montana Network) is received and opened as the Canon defines it: the phone checks its computation of the renewed keys against the Canon's frozen vectors at start and opens nothing under a computation that disagrees. It sends no heal of its own until the devices of one seed carry one heal between them (the Author's decision 416 a). No new address, server, key kind or network road is added.

## 1.0 (2322) - 2026-10-30

Source commit `09f6bff2c957`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2322.md).

- A comment on a wall costs no coin and brings none: nothing leaves your wallet when you comment, the wall's owner burns nothing and pays no share to the post's writer. Coins are born only by the network's core, one coin a second.
- The wallet's «Chats and calls» chains are history: the coins chats, calls and comments minted and burned before stay readable there, and nothing moves them now.

**Security and anonymity**

- Security and anonymity are unchanged. A comment's word no longer carries a coin and its name; the coin book no longer has a burn door, and the lines written before are read as history. No new address, key or network road is added.

## 1.0 (2321) - 2026-10-30

Source commit `bf11ab0b1f58`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2321.md).

- The app no longer counts coins of its own: letters in chats, taps on the wallet's coin, the VPN wall's seconds, the wallet's pull and the time away mint nothing on the device. Coins are born only by the network's core, one coin a second.
- The +1 that rose under a chat's field and at the finger on the wallet's coin is gone, since nothing is credited there.
- From 30.10.2026 12:00 UTC the app no longer asks the network to mint the seconds you are here, and the wallet no longer shows seconds on their way to such a minting: from that moment the network takes no such record.

**Security and anonymity**

- Security and anonymity are unchanged. The device's coin book no longer writes earned lines; the lines written before are read as history. The VPN wall and the wallet's pull still tell the chain that the person is here, as before. From 30.10.2026 12:00 UTC the app lays no minting record on the coin chain. No new address, key or network road is added.

## 1.0 (2319) - 2026-10-29

Source commit `5068b59c0575`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2319.md).

- Calls no longer credit or take coins: a call's seconds mint nothing and burn nothing, and no coin receipt stands under a call. Coins are born only by the network's core, one coin a second.
- Chess no longer credits or takes coins: no coins for moves, no stakes, no pot and no winnings. Coins are born only by the network's core, one coin a second.
- The captured pieces beside each player are all drawn at the same size, in the board's own artwork.
- A first message that arrives as a long letter is shown with its words, and a service message is never shown as a chat bubble.

**Security and anonymity**

- Security and anonymity are unchanged. Chess letters no longer carry any coin field; a letter of an earlier build is read without its coin fields. A call writes no coin and lays no burn on the coin chain; the call's row keeps only its name and seconds. No new address, key or network road is added.

## 1.0 (2317) - 2026-10-29

Source commit `3febd62ee04f`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2317.md).

- Chess no longer credits or takes coins: no coins for moves, no stakes, no pot and no winnings. Coins are born only by the network's core, one coin a second.
- The captured pieces beside each player are all drawn at the same size, in the board's own artwork.
- A first message that arrives as a long letter is shown with its words, and a service message is never shown as a chat bubble.

**Security and anonymity**

- Security and anonymity are unchanged. Chess letters no longer carry any coin field; a letter of an earlier build is read without its coin fields. No new address, key or network road is added.

## 1.0 (2316) - 2026-10-29

Source commit `f6484629d89a`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2316.md).

- When the network does not answer at the moment you open the wallet, the balance shows the last amount the chain confirmed for your key, not a lower local estimate. Your coins on the chain did not change.

**Security and anonymity**

- Security and anonymity are unchanged: the last amount the chain confirmed is kept sealed in your own folder on this phone, next to the key it belongs to, and never travels. When the road to the chain fails, the diary now records the reply code and which of the two chain nodes was asked, never the key.

## 1.0 (2315) - 2026-10-27

Source commit `1604d008df48`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2315.md).

- A file you sent to a channel or a group no longer shows as failed with «retry» after you reopen the app: it stands as sent, the way the channel shows it.
- In the row of files and pictures above the message field, hold one and carry it to another place: the numbers follow, and they are sent in the order you set.
- The Files page opens in the folder you last took a file from.

**Security and anonymity**

- Security and anonymity are unchanged: the folder of the last chosen file is kept sealed on this phone alone and never travels in a copy, and the sent state of a channel's file is read from the copies this phone already holds. Nothing new leaves the phone.

## 1.0 (2314) - 2026-10-26

Source commit `df26a92dd9fe`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2314.md).

- The Time Coins page shows the minting as it happens: while you are in the app, your Time Balance rises every second with the seconds on their way to the network, before the window that confirms them is finalized; when the network confirms them the number moves on without a jump. The line Available beneath it, and what you can send, stay exactly what the network has confirmed.

**Security and anonymity**

- Security and anonymity are unchanged: the rising seconds are shown and never spent, a transfer is weighed by the confirmed balance alone, and a minting record now counts only from the moment the person came into the app, so no second before their presence is claimed. Nothing new leaves the phone.

## 1.0 (2313) - 2026-10-25

Source commit `7c3624af1f00`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2313.md).

- Conversations keep their keys in step with the day more reliably: when the phone's clock runs ahead of the network, the first daily step of a conversation's keys waits up to four days for the network to confirm the day, and a conversation met again after a pause stays the same conversation. Messages, calls and coins work as before.

**Security and anonymity**

- This build strengthens security: the daily step that erases a conversation's earlier key is taken by the day the nodes confirm, not by the phone's clock alone, and waits at most four days for that confirmation; a restore from the 24 words takes the later of the two days; erased keys are written to the device's storage again if the first write was refused. Anonymity is unchanged: nothing new leaves the phone.

## 1.0 (2312) - 2026-10-25

Source commit `4c768afb86eb`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2312.md).

- Several files can now be chosen at once: they wait above the message field numbered 1, 2, 3 in the order you chose them, a cross takes one out and the numbers close up, and they are sent one after another in that order to a chat, a channel or a group, so a small file never overtakes a larger one. Messages, calls and coins work as before.

**Security and anonymity**

- This build changes neither security nor anonymity: the chosen files are sent by the same encrypted road as before, only one after another in the chosen order; in a group with slow mode, a set of files sent together now counts as one message.

## 1.0 (2311) - 2026-10-25

Source commit `a8986f934307`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2311.md).

- A long message split into several bubbles now reads top to bottom in the order it was written: the first part on top, and scrolling down shows the continuation. Messages, calls and coins work as before.

**Security and anonymity**

- This build changes neither security nor anonymity: it changes only the order in which the pieces of one long message are shown on the screen; nothing is sent, stored or encrypted differently.

## 1.0 (2310) - 2026-10-25

Source commit `f83cf7350b4b`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2310.md).

- Each conversation's secret now moves forward one day at a time. From 3 November the phone keeps only the step from two days back and erases the original secret, so a secret taken from the phone later cannot open earlier days. From 1 November messages carry the day's tag: update both ends to this build before then. Messages, calls and coins work as before.

**Security and anonymity**

- Security: the pipe secret walks by UTC day; from 03.11 every stored form keeps a 249-byte floor (the walk's value at D-2 and the quantities kept for good) with no byte of the original secret. Limits: an archive head already sent to twins and keepers stays until a newer head is written over it; a device clock set forward moves the floor for good. Builds 2309 and earlier do not hear the new tag from 01.11.

## 1.0 (2309) - 2026-10-25

Source commit `8439bb0f4306`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2309.md).

- The secret of each conversation now lives in one place inside the app and is never handed out: every part of the app, the notification extension and the share sheet ask that one place for what they need. Messages, calls and coins work exactly as before.

**Security and anonymity**

- Security: one owner holds the conversation secret and exposes only derived values; a build check refuses any read of the raw secret elsewhere. The wire is unchanged byte for byte. Erasing the original secret by day is the next step and is not in this build.

## 1.0 (2308) - 2026-10-24

Source commit `2d8a183d860e`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2308.md).

- A call that has already ended no longer keeps waking your device: the stale invitation is now confirmed to its sender, so it stops being sent again.

**Security and anonymity**

- Security: the confirmation is the same end-to-end sealed receipt as for any letter and carries nothing about the call. Anonymity is unchanged.

## 1.0 (2307) - 2026-10-24

Source commit `d2cd1db9a283`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2307.md).

- Calls connect when one of the nodes is down: every call signal also goes through the other node at once.
- On the Mac, closing the last window quits the app.
- A conversation you erased no longer comes back to the device's storage at every launch.

**Security and anonymity**

- Security: a call signal sent through a second node is the same end-to-end sealed packet, and the receiver drops repeats by its id; an erased conversation leaves no folder on the device. Anonymity is unchanged.

## 1.0 (2306) - 2026-10-24

Source commit `221bec73d642`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2306.md).

- One person, many devices: a letter, a photo or a video is taken by each of your devices on its own, so the Mac fetching first no longer leaves the phone without it.

**Security and anonymity**

- Security: the node marks a letter as taken per device under a per-label device name, so it links no two labels of one device; the sealed letter stays end-to-end encrypted and leaves the node at the box term. Anonymity is unchanged.

## 1.0 (2305) - 2026-10-24

Source commit `1d217f36fc91`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2305.md).

- Coins: the wallet's number rises every second again, as it did before, and each second's coin is shown rising. The count is still kept by the network's core.

**Security and anonymity**

- Security: the screen adds only the seconds the chain itself credits, never past the online cut; nothing is minted on the phone. Anonymity is unchanged.

## 1.0 (2304) - 2026-10-23

Source commit `33eb64764036`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2304.md).

- Coins: every second is credited again — one second, one coin — for everyone, including those who minted by the earlier doubling rule. The count is kept by the network's core; the app shows it.

**Security and anonymity**

- Security: the chain's nodes fold every coin record by the core alone; no phone mints for itself. Anonymity is unchanged.

## 1.0 (2303) - 2026-10-23

Source commit `ece5b8669210`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2303.md).

- Chats, groups and channels: pick as many photos, videos and files as you like at once; they are sent in the order you chose them.

**Security and anonymity**

- Security: the pick stays on the device until you send it; each item travels as before, a large pick in groups of ten. Anonymity is unchanged.

## 1.0 (2302) - 2026-10-23

Source commit `8df01bb54133`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2302.md).

- Montana now finds its update on every iPhone: the update chat and the button on the time panel no longer depend on the iOS version.

**Security and anonymity**

- Security: the update channel is the environment Apple signed for the installed app, so an App Store build still never points at a TestFlight beta. Anonymity is unchanged.

## 1.0 (2301) - 2026-10-23

Source commit `91a0555a2caf`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2301.md).

- Coins on someone's wall post now leave by a record of your TimeChain: the post's owner receives them from the chain, not from the phone.

**Security and anonymity**

- Security: a coin given on a wall post is a SEND of the chain -- a phone can no longer credit the owner coins the chain does not carry. Anonymity is unchanged.

## 1.0 (2300) - 2026-10-23

Source commit `4b68ce190d6e`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2300.md).

- The transfer screen speaks of your time chain: a transfer not yet in it reads "Not in your TimeChain yet" -- the word book is gone.

**Security and anonymity**

- This build changes neither security nor anonymity: one visible word on the transfer screen and its translations.

## 1.0 (2299) - 2026-10-23

Source commit `4d8b7da44672`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2299.md).

- From 26 October a call that is not recorded is paid on the time chain itself: its seconds are burned as one record at the head of your chain, once a window and at the end of the call.

**Security and anonymity**

- The burn of a call waits on disk until the node accepts it, and the debt falls only by an accepted BURN; before the online cut of 25 October 21:00 UTC nothing is laid, since the node refuses a BURN until then.

## 1.0 (2298) - 2026-10-23

Source commit `e66c53c8fefb`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2298.md).

- The chats page no longer stutters: the coin view of each conversation is reckoned off the main thread.
- Montana on a Mac opens only when you open it: a call or a letter no longer launches the app after you quit it.

**Security and anonymity**

- A Mac keeps no push token at any notify machine: Apple's letter token is dropped, every row of both tokens is withdrawn from every door, and the keys go only when every door has said yes. A closed Mac hears no calls or letters; it hears them while the person keeps it open.

## 1.0 (2297) - 2026-10-23

Source commit `9c4e9919a828`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2297.md).

- Your coin balance now shows what the time chain says; after the time cut calls and taps no longer mint coins on the phone itself, and an unrecorded call burns exactly its price.

**Security and anonymity**

- Past the time cut the phone mints nothing of its own: every number of the person's coins is read from the chain, less the sends still on their way. Until the chain first answers, the phone's own record still shows -- that view is replaced as soon as the chain speaks.

## 1.0 (2296) - 2026-10-23

Source commit `03de274b7160`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2296.md).

- Coins given on a letter now leave as a SEND record in the time chain under that letter; the reaction only names the record, and the reader's coins arrive from the chain itself.

**Security and anonymity**

- A paid reaction sent by this build moves its coins only by a SEND record laid on the sender's chain and taken by the receiver's key; its chat word carries the record's name and the letter's name and moves no coins. The older reaction word of earlier builds is still credited by the reader for now -- that road is not closed yet.

## 1.0 (2295) - 2026-10-23

Source commit `7806640bf463`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2295.md).

- From 26 October, time coins are minted while you are here: every forty seconds the phone records the seconds you spent with Montana open or behind the Montana VPN as a MINT in the time chain. Taps, calls and time away mint nothing separately.

**Security and anonymity**

- The phone now records minted seconds itself as MINT records in the time chain, signed by its own key; the node accepts only seconds inside its freshness window, so a phone cannot mint time it was not present for. Nothing new about the person is sent: a MINT carries the key, the interval and the amount, as before.

## 1.0 (2294) - 2026-10-23

Source commit `55a032bb8d74`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2294.md).

- Montana no longer reaches a server that left the network: the old Frankfurt machine now belongs to someone else, and the phone stopped connecting to it.

**Security and anonymity**

- The phone's core node dialed four genesis machines; one address now belongs to a third party and would have learned the phone's network address. It is removed; the phone reaches Lauterbourg, Moscow and Amsterdam only. Nothing new is sent.

## 1.0 (2293) - 2026-10-23

Source commit `057f2d1ec42d`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2293.md).

- Coins follow the new rule of the time chain: from 25 October 21:00 UTC a mint counts only the seconds you are present in Montana, and a burned coin is a record of the chain like any other spend.
- The gold page ground is gone; a page that used it now shows Montana's own dark ground with its pattern.

**Security and anonymity**

- The phone folds its coin chains by the same rules as the nodes (fold-7), so the balance it shows is the one the nodes compute; a burn is a signed record the nodes read. Nothing new is sent.

## 1.0 (2292) - 2026-10-23

Source commit `679efed3dd05`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2292.md).

- A background wake no longer keeps Montana busy: the copy of the coin chains is checked at most once in ten minutes even when a notification starts the app, and a record's signature is verified once, not at every wake.
- Large stored values (the wall above all) now live in their own files instead of the app's settings, which had grown past the system's limit and were rewritten whole at every change.
- Saving a value that has not changed no longer rewrites the keychain.

**Security and anonymity**

- Large sealed values stay sealed under the device key in their own files, named by the SHA-256 of their bytes; a copy and a change of person carry them exactly as before. A coin record is still verified before it is first believed. Nothing new is sent.

## 1.0 (2291) - 2026-10-22

Source commit `d5970e7ad630`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2291.md).

- Switching the record button between voice, video and coin no longer cuts off music or a voice message that is playing: a tap only switches the mode, and the microphone is taken only when you hold to record.

**Security and anonymity**

- The recording session is taken only by the hold that records; a tap of the switch opens no microphone. Nothing new is sent or stored.

## 1.0 (2290) - 2026-10-22

Source commit `33cd784c0819`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2290.md).

- The gold background now wears the system's dark appearance: black panels and white text on it, as the system draws them on a dark background.
- Chats stand in the list by the time of their newest message, the same time the row shows: letters restored from the archive or a late photo no longer lift an old chat to the top.
- Photos and files you open load first: the background copy of your history waits while the media you are looking at comes in.

**Security and anonymity**

- The text colour on the gold background is decided on the device from the background stored there; nothing new is sent or stored.
- The chat order and the yielding of the background copy are decided on the device from letters already there; nothing new is sent or stored.

## 1.0 (2288) - 2026-10-22

Source commit `47080687a05c`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2288.md).

- Words follow the background under them: a chat with its own background and a person's page with theirs now show white text on dark backgrounds and black text on light ones, as the system does.
- Notifications for a conversation no longer read the conversation's secret: each conversation keeps its own local name from the moment it is created, and the notification service finds the chat by that name.
- When the app cannot check for a newer build, its private diary now says why (no store channel, network error, the site's answer or the document), once per new reason.

**Security and anonymity**

- Build 2288 narrows what the notification service touches: it no longer derives the chat alias from a conversation's established secret on every round, but reads a local name the conversation kept at birth; nothing changes on the wire.
- The diary line about the release check names the public server it asked and the kind of refusal; it carries no key, account or identity of the user.
- The text colour of a chat or a page is decided on the device from the background stored there; nothing new is sent or stored.

## 1.0 (2287) - 2026-10-22

Source commit `3679172de4ef`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2287.md).

- Notifications for a conversation no longer read the conversation's secret: each conversation keeps its own local name from the moment it is created, and the notification service finds the chat by that name.
- When the app cannot check for a newer build, its private diary now says why (no store channel, network error, the site's answer or the document), once per new reason.

**Security and anonymity**

- Build 2287 narrows what the notification service touches: it no longer derives the chat alias from a conversation's established secret on every round, but reads a local name the conversation kept at birth; nothing changes on the wire.
- The diary line about the release check names the public server it asked and the kind of refusal; it carries no key, account or identity of the user.

## 1.0 (2286) - 2026-10-22

Source commit `1b8505362514`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2286.md).

- Music on the shared Wall of Thoughts plays again: the author of a post with tracks lays its files on the nodes once more whenever a node has let them go, and the nodes now keep the Wall's files by their free space instead of for one week.
- Montana Core for Mac now downloads from the site and from GitHub as a disk image: open it and drag Montana Core into Applications. The image itself is signed with the Developer ID and notarized by Apple, as is the app inside it.

**Security and anonymity**

- The Wall of Thoughts repair (first carried by build 2285, which never shipped) changes neither security nor anonymity: the owner of a Wall post asks the nodes which of its own encrypted pieces they hold and lays the missing ones again under the same key; the nodes see the same sealed bytes and names as before.
- Build 2286 changes neither security nor anonymity of the app: the Mac product is packed in a disk image signed with the same Developer ID and notarized by Apple on its own; the app inside is the same notarized, stapled app, and its reproducible digest counts the same files.

## 1.0 (2284) - 2026-10-22

Source commit `93fd8386c1b3`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2284.md).

- Every choice in Settings now says what it does: Video note quality, Keys, Message style, Keep media, Maximum cache size, Copies to your node, View, Look and Own colours each carry a line under them.
- The sign-in pages read in both light and dark: on the burgundy page the words are light, as iOS itself would draw them.

**Security and anonymity**

- Build 2284 changes neither security nor anonymity: it adds lines of explanation in Settings, draws the sign-in pages in the scheme their background asks for, and the TestFlight road on the build machine gives its build gate back sooner.

## 1.0 (2282) - 2026-10-22

Source commit `51a2d7c9b023`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2282.md).

- Every switch in Settings now says under it what it does.
- With Show sender off, no notification names who wrote or shows their photo -- in private chats, groups and rooms alike.

**Security and anonymity**

- The lasting keys a chat derives from its secret are made once when the chat is born and kept beside it, so the screens that read them no longer touch the chat secret (the first step of the key ratchet).
- Show sender off now hides the sender in group, room and private notifications in both the app and the notification extension.

## 1.0 (2281) - 2026-10-22

Source commit `13041e634423`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2281.md).

- Montana Core for Mac is built from the core the public montana_core repository publishes, so its release on GitHub carries the core its frameworks name.

**Security and anonymity**

- Only how the Mac app is built and published changes; nothing new is stored or sent.

## 1.0 (2280) - 2026-10-22

Source commit `08d3cf128fe5`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2280.md).

- Montana Core for Mac is published with its install file: the notarized ZIP on GitHub and on the site's new Desktop page, with its SHA-256 and the minimum macOS, and a clean clone of montana_core rebuilds it byte for byte.

**Security and anonymity**

- Only how the Mac app is built and published changes; nothing new is stored or sent.

## 1.0 (2278) - 2026-10-22

Source commit `31b8b4e0afe4`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2278.md).

- Montana Core for Mac now rebuilds byte for byte from the public source in montana_core: any builder, at any path, gets the same app, and the release names its digest.

**Security and anonymity**

- Only how the Mac app is built changes; nothing new is stored or sent.

## 1.0 (2276) - 2026-10-21

Source commit `d49860c9d61f`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2276.md).

- The correspondent's bubble stays the platform's dark grey under white words on every chat background, the gold one included.

**Security and anonymity**

- Only the bubble's appearance changes; nothing new is stored or sent.

## 1.0 (2274) - 2026-10-21

Source commit `9d106575163d`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2274.md).

- Montana Core for Apple silicon Macs is downloadable from montana.quest: signed with Developer ID and notarized by Apple. On the Mac the globe opens the P2P wall; the VPN stays on phones.

**Security and anonymity**

- The Mac app carries no network extension and asks for no tunnel right; nothing new is stored or sent.

## 1.0 (2273) - 2026-10-21

Source commit `dc181f7316a5`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2273.md).

- Coins follow the chain's own clock: from the cut on, the chain itself counts the seconds a key has lived before every record, and the phone no longer lays its own mint for time.

**Security and anonymity**

- Nothing new is stored or sent: the phone checks the chain's time step by the same rules as the node and refuses a fold without it.

## 1.0 (2272) - 2026-10-21

Source commit `75e6afd6c837`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2272.md).

- Top up now says only what is true: 1 coin is 1 second. Live here and now.

**Security and anonymity**

- Nothing changes in what is stored or sent: only the text of the Top up sheet is shorter.

## 1.0 (2271) - 2026-10-21

Source commit `5e38c1985e61`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2271.md).

- Every link on a person's page now shows: a page field holding several addresses gives each its own bubble, and each opens.

**Security and anonymity**

- Nothing changes in what is stored or sent beyond the page field: several addresses travel as one field separated by spaces, each read by the same rule a single address was.

## 1.0 (2270) - 2026-10-21

Source commit `dafe38980253`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2270.md).

- The last icons on glass over the gold background are dark now too: network, channel and reactions.
- Groundwork for protecting the past: from 29 October coin transfers and the signature that marks a letter as yours rest on a key of the conversation's own, so the conversation's first secret can later be erased.

**Security and anonymity**

- From 29.10.2026 UTC a coin transfer's blind and a letter's author field are computed from the conversation root, SHA-256 of mt-conv-root, a zero byte and the established secret, instead of the established secret itself; a receiver checks the root first, then the secret. Nothing new is stored or sent and the nodes do not change. Until then every build blinds and signs as before.

## 1.0 (2269) - 2026-10-21

Source commit `94b449d4a69f`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2269.md).

- On the gold background the last light words are dark now too: the terms, settings, sign-in, profile, network screen and the GIF, sticker and emoji panel read as clearly as the rest of the app.

**Security and anonymity**

- Appearance only: 37 labels take the background's own ink instead of fixed white. Nothing new is stored or sent, and no message, key or coin road changes.

## 1.0 (2268) - 2026-10-21

Source commit `b2ae0f785901`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2268.md).

- Each chat's key now moves forward on a one-way chain once a day, starting 29 October: the letters of every day are sealed under a key of that day, derived from the day before and never back. Up to 28 October nothing changes in how letters travel, and this version talks with the earlier ones as before.

**Security and anonymity**

- The daily step is one SHA-256 under its own domain over the day before, the same rule the protocol's Canon states for a pipe; nothing of the chain crosses the wire. From 29 October a version older than this one no longer opens this one's letters, so update every phone before then. The secret the chat began with is still kept on the phone in this version (for the safety number, the backup and the copies kept by correspondents); erasing it, which is what makes earlier days unreadable from a taken phone, comes as its own step.

## 1.0 (2267) - 2026-10-21

Source commit `e4cd136fc54c`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2267.md).

- Each chat's key now moves forward on a one-way chain once a day, starting 29 October: the letters of every day are sealed under a key of that day, derived from the day before and never back. Up to 28 October nothing changes in how letters travel, and this version talks with the earlier ones as before.

**Security and anonymity**

- The daily step is one SHA-256 under its own domain over the day before, the same rule the protocol's Canon states for a pipe; nothing of the chain crosses the wire. From 29 October a version older than this one no longer opens this one's letters, so update every phone before then. The secret the chat began with is still kept on the phone in this version (for the safety number, the backup and the copies kept by correspondents); erasing it, which is what makes earlier days unreadable from a taken phone, comes as its own step.

## 1.0 (2266) - 2026-10-21

Source commit `c608f4df732b`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2266.md).

- Each chat's key now moves forward on a one-way chain once a day, starting 29 October: the letters of every day are sealed under a key of that day, derived from the day before and never back. Up to 28 October nothing changes in how letters travel, and this version talks with the earlier ones as before.

**Security and anonymity**

- The daily step is one SHA-256 under its own domain over the day before, the same rule the protocol's Canon states for a pipe; nothing of the chain crosses the wire. From 29 October a version older than this one no longer opens this one's letters, so update every phone before then. The secret the chat began with is still kept on the phone in this version (for the safety number, the backup and the copies kept by correspondents); erasing it, which is what makes earlier days unreadable from a taken phone, comes as its own step.

## 1.0 (2265) - 2026-10-21

Source commit `1fc72aed12f7`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2265.md).

- Voice messages can now be read as text. Tap the quote icon beside a voice message and its words appear under it. On iOS 26 the system's new speech analyzer does the work; on earlier iOS the phone's own on-device recognizer does, with punctuation and the names from your chat.
- Coins go to anyone you have a chat with at once. Your phone now tells its coin key to your correspondents by itself, and a transfer that meets a correspondent whose key has not come yet asks for it and goes through within seconds instead of refusing.

**Security and anonymity**

- Recognition runs only on the phone: no audio and no text is sent to Apple or anyone else. A phone that cannot recognise the language on the device says so and sends nothing. The text lives in memory only and is never written to disk, so a deleted chat leaves no transcript behind. No message, key or coin road changes.
- The coin key told to correspondents is the public tag of the coin key, never a name or a secret; it rides the same end-to-end encrypted word about oneself as before, and the ask for it carries nothing but a flag. No coin moves without the sender's signature.

## 1.0 (2263) - 2026-10-21

Source commit `da721028f130`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2263.md).

- On a video call, switching your own camera off now keeps the other person's video on your screen. Your camera rests, they see your cover, and the call returns to voice only when the last camera is switched off.

**Security and anonymity**

- The camera is stopped at the hardware level when you switch it off: no frames leave the phone. One short word tells the other side your camera rests; it carries no picture and no new data. No message, key or coin road changes.

## 1.0 (2262) - 2026-10-21

Source commit `e0b8cc7b791c`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2262.md).

- On the gold background every word is now dark and easy to read: the whole app takes the light look of the system on it, and switches back to the dark look on every dark background.

**Security and anonymity**

- Appearance only: the colour scheme follows the background chosen on this phone. Nothing new is stored or sent, and no message, key or coin road changes.

## 1.0 (2261) - 2026-10-20

Source commit `1b0f742d9473`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2261.md).

- Coins reach a new person at once: someone who has not written anything about themselves still tells your phone where their coins arrive.
- A transfer that cannot go says so right away — «Coins were not sent», your balance untouched — instead of hanging on «being confirmed».

**Security and anonymity**

- The coin receiving key now rides the word about oneself even without a bio; it is the same public receiving key, sent only inside the pair's end-to-end pipe, so nothing new leaves the phone. A refused transfer is shown on the sender's phone alone and is never presented as confirmed.

## 1.0 (2260) - 2026-10-20

Source commit `d9d602571d5a`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2260.md).

- One person is one conversation: when two people scan each other's codes at the same moment, the chat no longer appears twice — both phones fold it into one.

**Security and anonymity**

- Folding two pipes to one person discloses nothing new: the accepting phone's question carries only tags keyed by the new pipe's secret, which the other side can match only for pipes it already holds; the unused slots are random noise.

## 1.0 (2259) - 2026-10-20

Source commit `7511e4bc8f1c`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2259.md).

- Sign-in by phone number is gone from Montana: your identity is your 24 words, and nothing asks for a number.
- A call's coins are the same for both people: while the call is recorded on either side, both earn every second; when nobody records it, both spend the same.

## 1.0 (2256) - 2026-10-20

Source commit `9e2bf95036ca`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2256.md).

- Call recording follows levels: when your correspondent has recording turned off, it stays off unless your Global level is higher.
- A call recording now reaches the chat whoever made it.

## 1.0 (2255) - 2026-10-20

Source commit `a27409a882ce`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2255.md).

- Photos, videos and files in chats now lie on every Montana machine at once, so they arrive even when one machine cannot be reached.

## 1.0 (2254) - 2026-10-20

Source commit `290699b7ad54`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2254.md).

- The coins page shows what is left to the next level in units of time: one coin is one second.

## 1.0 (2252) - 2026-10-19

Source commit `52203d7b0e8e`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2252.md).

- Montana Core v1.0: from 20 October a second of life mints one coin, and the nodes take no more from any build.
- A build older than the one the network admits sees the update page at every opening, with the link to the new build.

## 1.0 (2251) - 2026-10-19

Source commit `6a2c7875fb9e`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2251.md).

- On the coins page, tap the time, the coins or $TCM to make it the main sum of your balance, large and white; the Time Balance in units of time is the default, and your choice is remembered.
- The ticker $TCM replaces the letter sign everywhere a balance is written in Montana.
- On the Send page a sum is typed in time, in coins or in $TCM; the next level and the time online are told in units of time.
- Two persons on one phone are wholly separate: each one's coins stay with that person, and nothing written for one is written for the other.

## 1.0 (2250) - 2026-10-19

Source commit `5f4152679331`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2250.md).

- The coins page shows your Time Balance: the time your coins hold, large and white, in days, hours, minutes and seconds, with the coins beneath it. One coin is one second, and one TCM is a billion coins.
- Transfers, the history, the analytics, the TimeChain and a call's receipt show the time beside every amount of coins.
- No caption to hold the coin and no tap counter under it: a tap mints at most one coin a second.
- From 19 October, 21:00 UTC, the coin chain itself counts one coin per second of a key's life.

## 1.0 (2249) - 2026-10-19

Source commit `f2bd9d56e4fe`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2249.md).

- A second of life mints one coin, everywhere and all together: no level, music, call, chat bubble or chess move mints faster. Your level of π is now a reading only.
- After a chess game for a pot, the bubble shows the coins your wallet actually took.

## 1.0 (2248) - 2026-10-19

Source commit `20451c1ad5f9`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2248.md).

- Coins sent to a person whose app could not accept them now come back to you by themselves a day after sending.
- The Montana Top and «Level the top 13» now use each person's balance as their TimeChain proves it, never the number their app reports.

## 1.0 (2247) - 2026-10-19

Source commit `dedf97cbbe3e`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2247.md).

- Montana no longer closes when a copy of your own message comes back from another of your devices; such a copy no longer rings a notification with your own name.
- Money Flow: the lines inside a bubble read top to bottom again, as before.
- Update messages now reach every phone and speak your language in full.
- Wall of Thoughts: the global feed shows every post of every wall, of any age, not only the latest ones.

## 1.0 (2246) - 2026-10-18

Source commit `1c8c9775e403`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2246.md).

- The time panel wears the system's liquid glass again, as before.
- Data and Storage names everything Montana keeps on your phone: opened files, history of chats, faces and the rest, each with its size. Opened files that stayed behind are cleared with one tap — what your node does not hold yet goes back to your chats, nothing is lost.
- Removing attachments by kind now removes only the phone's copy, and only what your node holds: the messages stay, each file comes back from your node when you open it, and files your node does not hold yet stay on the phone.
- Files you opened no longer stay on the phone after their cache was cleared.

## 1.0 (2245) - 2026-10-18

Source commit `2384fae584ab`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2245.md).

- Your own photo no longer appears on chats with other people: a picture equal to your own face is never taken as someone else's, whatever road it came by.
- The time panel wears one glass on every iPhone: the same clear, see-through plate on iOS 26 as on earlier systems, and the logo no longer looks darker inside it.
- Balances and levels next to other people's faces and in the coin rating show only what their own coin chain proves on the nodes, so a modified app can no longer claim any number.
- A new wallet announces coins only from its own past: a key with no history on the coin chain opens at zero.

## 1.0 (2244) - 2026-10-18

Source commit `f1f8193518fe`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2244.md).

- The app no longer closes when a call ends: letters that waited through a call are sealed once and leave a few at a time.
- Music and videos on walls play again: when the network has lost a file's pieces, the phone asks the wall's owner to lay them again, and a track that cannot play no longer shows as playing at 0:00.
- Your node keeps your files: with your node set up, every photo, video, voice and file of your chats goes to your own machine in sealed pieces. Data and Storage, Clear cache frees the phone of what your node holds, and each file comes back when you open it, also on a new phone with your 24 words.
- Anyone can put up a node of their own: the door and its installer are in the open source, in Code/Node.
- The Wall of Thoughts is for everyone: the posts of walls shown to everyone reach every Montana user, by the time they were published. Names on posts are the names as they stand now, and any person can be hidden from the feed.
- As in the best storage screens: Data and Storage, Keep media lets files nobody opened for 3 days, a week or a month leave the phone by themselves, and Maximum cache size keeps the cache under 5, 16 or 32 GB -- only what your node holds ever leaves.

## 1.0 (2242) - 2026-10-18

Source commit `b8b6931b21fc`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2242.md).

- The app no longer closes at launch: a settings record that could not be saved made 2239 and 2240 quit at every start, and that whole kind of record is now saved correctly.
- The phone stays cool: the copy your contacts keep for you no longer starts again and again when the phone is short of space. It checks the free space first and tries once a day.
- Chats open and scroll faster: the chat list no longer waits for the system keychain, and the coin book is read and written in the background.
- The folder tabs float over the chat list like the time panel: the list scrolls beneath them.

## 1.0 (2241) - 2026-10-18

Source commit `f140ddb261ad`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2241.md).

- All your devices are one: a person who writes to you through the link from your phone now reaches your Mac and tablet too, and the chat appears on every device.
- Your own channels and groups: posts and invitations you make on one device are recognised as yours on your other devices instead of being refused.
- Adding people to a call no longer cuts it: the person you are talking with moves into the group call at once, and everyone you invited shows as calling until they join.
- Big player: dragging the volume line changes the volume and no longer turns the page.
- Now Playing: groups and channels are shown by their names; the order icon moves tracks by hand, and the arrows next to it reverse the order.

## 1.0 (2240) - 2026-10-18

Source commit `a9c8c25aad9f`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2240.md).

- Less heat and longer battery: your coin book is no longer downloaded and decoded again every half minute -- only what is new is read, about once a minute.
- Less heat: the phone reads its mailbox from each node once, waits when a node asks it to slow down, and asks only for the days a letter can still be there.
- Less heat for the people you write to: a letter the node already keeps is re-sent more and more rarely instead of every five minutes, so their phones are not woken again and again; it still reaches them the moment they appear.
- A first message to someone who has not answered yet waits for them instead of being re-sent every twenty seconds.
- Direct connections: an address that never answers is tried more and more rarely; when your network comes back, every address is tried again.
- A lighter start: profile pictures from contact cards, deleted chats and the check of the chat list are no longer redone at every launch.
- Your coin balance in your profile reaches your contacts over a connection that is already open and no longer wakes their phones.

## 1.0 (2239) - 2026-10-18

Source commit `ccaae4d233db`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2239.md).

- Your devices with the same 24 words: every letter you receive or send, every post in your groups and channels, and every read mark reaches your other devices too; what one device took before this version is fetched from the other, photos and videos included.
- Read on one device is read on all: the notification leaves your other devices.
- Fabric of Time: the people you write to and call regularly over the last two weeks, each weighed by their ceiling -- the lower of their Person and Wallet levels; the window slides, no turns. A signature names your conversation, never you: nothing in it is the same with two people.
- Chats: the folder tabs ("All" first) stand over the list like the time panel; swipe left and right to move between them, and past the last one to the next page.
- Share into a channel or a group: posts go by the group's own road, in exactly the order you sent them -- no more "Retry".
- Settings › Data and Storage: "On this phone" shows what takes the space -- chats, walls, the backup shelf; leftovers of interrupted copies are swept; select every file of a kind and delete them at once.
- Leave a chat by a swipe from anywhere on the screen, not only from its edge.
- TimeCoin: the system's wallet icon everywhere; no red badge over it in a chat; a tap on a notification closes every open page.
- Money Flow: the words inside a bubble read from the bottom up, as the game's feed does.
- YouTube: Montana's own mini player -- drag it to any corner, open it full screen, close it.
- VPN: less heat -- the engine's memory works under one ceiling; a server that does not answer is found by a probe, and the VPN lets the phone go instead of holding it.
- Fixed: the home node's set-up messages are shown in your language.

## 1.0 (2238) - 2026-10-18

Source commit `3778213a0d0e`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2238.md).

- Fabric of Time: a third level beside the Person and Wallet levels. After a call of two in which each of you heard the other for at least a minute, both phones exchange a signed call seal; your level is now the lowest of the three. For the first two weeks the Fabric of Time takes the lower of your other two levels.
- Calls: add people from your contacts on the call page -- up to 13 in one call; the call becomes a group call.
- Groups and channels: the buttons under the message field are those of chats -- a contact, a location, a business card, link previews and your own stickers work there too. Fixed: a contact sent from your address book now reaches the other person.
- Groups: send coins to the members you check, on the same Send page; each one gets a transfer of their own.
- Groups: New Group, Add Members and Group Name steps; a group can start with nobody yet. A group's Media holds its photos and videos together.
- Channels: administrator rights one by one.
- Settings › Questions: answers about levels, minting, privacy and the Fabric of Time.
- YouTube: a video goes straight into the system's picture in picture; coming back from it opens the system's full-screen player.
- Fixed: a vertical video longer than 30 seconds could arrive lying on its side.

## 1.0 (2237) - 2026-10-17

Source commit `2f460c9506e8`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2237.md).

- Channels: Some Reactions -- under Allow Reactions the owner picks which reactions subscribers may use; the post menu offers only those.
- Groups: Chat History for new members -- hidden by default; the owner can show new members the group's recent messages. Group Type shows Private: a group is joined by its invite link.
- Fixed: the Slowmode hint stood twice in the translations and could show the wrong language's words.

## 1.0 (2236) - 2026-10-17

Source commit `8d64b8efba04`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2236.md).

- Groups: Permissions -- the owner chooses what members may do (send messages, media, stickers, links, add members, pin, change the info) and sets Slowmode; a member's field says when they cannot send and how long to wait.
- Groups: administrators with rights and their own titles -- the owner gives each one the rights they need, and the title shows in their bubbles.
- Groups: Removed Users -- a removed person cannot come back by an old link until the owner lets them. Invite Links can ask for approval: whoever opens the link waits in Member Requests until the owner adds them.
- Channels: comments under posts -- turn on Allow Comments in Channel Settings; each post shows Leave a Comment or the number of comments. One comment per 30 seconds from each subscriber.

## 1.0 (2235) - 2026-10-17

Source commit `340be126d70c`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2235.md).

- Channels: administrators -- the owner names them from the subscribers, and they post into the channel and remove posts. With Sign Messages on, each post shows its author's name beside the time.
- A post forwarded from a channel says Forwarded from and the channel's name.
- Channel Settings: Allow Reactions turns subscribers' reactions on or off. Removed Users lists the people the owner removed -- they cannot come back by an old link -- and a removed subscriber sees a grey Join. Recent Actions shows the channel's last 48 hours on the owner's phone only.

## 1.0 (2234) - 2026-10-17

Source commit `49de34971deb`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2234.md).

- Channels: the channel's page as you know it -- the subscriber count under its name, Mute, Search and, for the owner, Subscribers and Channel Settings: the name, the description, Invite Links with Copy, Share and Revoke Link, and Delete Channel. A subscriber can leave the channel or report it.
- Channel posts: a subscriber's post menu is Copy, Forward, Report and Select. The owner's bell in the message field sends a post silently -- it arrives without a banner.
- A new channel starts with What is a Channel?, then its name, an optional description and its subscribers.
- Groups: pin a message for everyone, with or without notifying the members; everyone sees who pinned it. The group's top shows who is typing.
- Groups: your messages show who has seen them -- hold a message for N Seen. Read receipts off: nobody is told you read theirs. Hidden online status: nobody sees you typing, in a group or a chat.

## 1.0 (2233) - 2026-10-17

Source commit `18dac89c82b9`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2233.md).

- Chat folders: tabs over your chats -- All Chats, Chats, Groups, Channels and Bots -- each with a badge of its unread chats, blue while one of them is not muted. Hold a tab for its menu: edit the folder, add chats, read all, mute all, remove it or reorder the tabs. Settings → Chat Folders creates your own folders: which kinds of chats and which chats a folder takes, and which it leaves out.
- Bots: our bot, MTBusiness_bot, stands in the Bots folder and in All Chats; a tap opens it.
- The green shield of Forbid deletion now stands as a badge on the corner of the person's name at the top of each chat; a tap shows the rule.
- Share to Montana: the circle of a chosen chat stands true, and while a share goes each chosen face fills its blue ring -- Waiting, Sending, then Sent once the node took that chat's letters.
- Groups: the sender's name and role (owner, admin) inside the bubble, a tap on a face opens the person, a quote names its author. The group page shows N members and who is online, with Voice Chat, Mute, Search and More, the members with their roles, and Invite Links. Editing a group no longer clears its description.
- Channels: a channel is a feed of posts again -- the channel's name in the post, the views under it, a share button beside it; a subscriber has Mute and Search below. Posts from the channel's wall move into the feed on your phone. A subscriber with Read receipts off tells the channel nothing about what they saw.

## 1.0 (2232) - 2026-10-17

Source commit `8c6f35988460`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2232.md).

- YouTube in picture in picture: a tap on a YouTube link in a chat, on a wall, among a contact's links or in the music search starts the video at once in YouTube's own player and moves it into the system's picture-in-picture window, which floats over your chats and over other apps. The window's own button brings the video back into Montana; its cross closes it.
- Nothing in the chats takes the sound before your own tap: another app's music plays on until you start something yourself, and it goes on again when YouTube pauses or closes.
- Access by level: from level 8 you can connect your own node to the Montana network -- set it up from the app and send copies of your phone to it. Restoring your copy from your node works on every level.

## 1.0 (2231) - 2026-10-17

Source commit `29c06f7eb134`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2231.md).

- YouTube inside Montana: a YouTube link in a chat, on a wall or among a contact's links now plays right here, in YouTube's own player on a sheet, without leaving the app. Your music pauses while it plays; the arrow at the top right opens the video in YouTube itself.

## 1.0 (2230) - 2026-10-17

Source commit `da43a97c9c09`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2230.md).

- Photos: a picture that would not send — the same photo refused again and again and its bubble turning red — now sends. Its small preview is drawn the same way for every camera photo, wide-colour and HDR pictures included; a picture whose file truly cannot be read says so once, instead of retrying for days. The share sheet makes its preview the same way.
- Groups and channels: after restoring from a copy they come back to the chat list and to the Groups and Channels apps, with their pictures. A copy now always carries the chat list, even when some kinds of attachments are left out of it.
- Notifications from groups and channels show the group's own picture, both when the app is asleep and when it is open.

## 1.0 (2229) - 2026-10-17

Source commit `8897ba7da629`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2229.md).

- Calls: End & Accept on a second call no longer drops the call you have just taken. When the phone briefly refuses the audio while the first call's sound winds down, the new call waits for the system instead of ending. The caller's screen shows that it is connecting until the network has carried the call, and shows the call as ringing out only after that.

## 1.0 (2228) - 2026-10-17

Source commit `cc75e9bade1e`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2228.md).

- Wallet history: a day's small expenses fold into one line, as the day's minting does. Coins on letters and posts, coins for comments and small burns become one line per source a day, and its card shows how many moves it holds and their sum. Transfers to people, calls and larger expenses stay lines of their own; the day's totals do not change.

## 1.0 (2227) - 2026-10-17

Source commit `bd0319bf90cb`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2227.md).

- Wallet: the whole balance is available at once. Coins earned while the app was away go to your TimeChain as soon as you come back, and time you have not minted yet is never lost: the chain weighs all your mints together against your key's whole time since its birth, not interval by interval. A phone that already holds a gap closes it with its next mint.

## 1.0 (2226) - 2026-10-17

Source commit `fda6a42218f4`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2226.md).

- Notifications: a letter on a locked phone shows one banner with the sender's name and sounds once. The banner is handed to the system before any other work of the notification extension, so a bare push no longer replaces it; a letter is announced through one node, not two; the app and the extension share one record of what was shown, so a letter does not ring twice while the app is open.

## 1.0 (2225) - 2026-10-17

Source commit `8d9273c8c46f`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2225.md).

- The group call screen is drawn anew: the group's name and how many are in the call at the top, the pictures of those with a camera on in a grid of two, and everyone in one list under them -- who is speaking, who is listening, whose microphone is off. At the bottom: sound, video, the microphone in the middle, screen and leave. Tap a picture to see it large.

## 1.0 (2224) - 2026-10-16

Source commit `14b2fe341c23`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2224.md).

- A group's title now says how many people are in it and, when someone besides you is here, how many of them are online. Only the people your phone already sees online are counted; a channel shows its count alone.

## 1.0 (2223) - 2026-10-16

Source commit `a37a550fbe62`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2223.md).

- Channels read as a thread of short posts: the newest post stands on top, and a reply never lifts an old one. A post's page shows the post whole with the replies below it in order, each with its author's face; everyone in the channel can reply from the bar at the bottom.

## 1.0 (2222) - 2026-10-16

Source commit `000fd41cc271`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2222.md).

- Chats: a green shield to the left of Chats, Groups and Channels shows that deletion is forbidden on your phone. Tap it to read the level rule and open Access by level. Below level 7 the level switches stand off.
- Groups: each person's messages now run together under their name in their own colour, with their face beside the last one.
- Group calls: joining a call that has gone on for more than half an hour now connects you to everyone in it, and a joining call connects sooner.
- Channels: a channel's page no longer shows a call button.

## 1.0 (2221) - 2026-10-16

Source commit `8d2c782a2c10`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2221.md).

- Channels read as a feed of short posts: each post with its author's face, the time, and one row for comments, reposts, likes, views, saving and sharing. The channel's owner writes a new post right at the bottom of the channel and publishes it from there.
- A photo or a group message now arrives as one notification, not two.
- Share: Montana's own send button stands above the keyboard, and the selection ticks sit straight. Sharing to My Wall leaves a notification: tap it to finish the post in Montana.
- Big player: a button to the left of Back shows the current playlist and where it plays from; tap a track to play it, swipe to remove it, or change the order. A thin volume line stands above the round buttons, and a tap on the player no longer reaches the page under it.
- Settings, Access by level: from level 7 you can forbid deletion on your phone and record calls automatically; below it, the call screen records by hand. A video call is recorded as you saw it. The pinned bar no longer covers messages. Deleting a message also takes its notification away, in groups and channels too.

## 1.0 (2218) - 2026-10-16

Source commit `ac2d94ed8fb9`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2218.md).

- Photos and videos arrive faster: the app no longer sends your Wall of Thoughts page to every contact again at each launch. The page now goes once for each change and waits for the contact's phone to confirm it holds it.
- In the Wallet, only Available stands under the balance. Sending coins from a chat or from the Send page shows and checks the same available amount.

## 1.0 (2217) - 2026-10-16

Source commit `fc72b8c5f3bb`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2217.md).

- Music playing in another app keeps playing when you open the feed or a chat: videos that play without sound no longer take the phone's sound, even right after a call or a video with sound.
- The leftmost button of the time panel is the apps button again. It turns into the update arrow only when a newer build is available, and a tap opens the update directly.

## 1.0 (2216) - 2026-10-16

Source commit `3e469ccae723`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2216.md).

- Video calls with recording on use far less of the phone: each moment of the recording is drawn once instead of every arriving frame, and the finished recording keeps the picture as it was recorded instead of encoding it a second time.
- When the phone runs hot, the call recording keeps the voice and slows the picture to one frame a second; at the hottest it records the voice alone. The picture returns when the phone cools.

## 1.0 (2215) - 2026-10-16

Source commit `0897094575fe`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2215.md).

- Diagnostics: the notification extension now records how much processor time each incoming notification costs the phone, so the cost of a busy chat can be measured.

## 1.0 (2214) - 2026-10-16

Source commit `49a723a3ba44`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2214.md).

- Diagnostics: the app now records how much processor time it uses and on which of its tasks, next to the phone's temperature, battery and charger state, so a phone that runs hot can be traced to its cause.
- When iOS reports that the app used too much processor or wrote too much to storage, the report is now kept in the diagnostics with the part of the app that caused it.
- A call's summary in the diagnostics names the share of the processor the call used and whether call recording was on.

## 1.0 (2213) - 2026-10-15

Source commit `527515bec2d8`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2213.md).

- Calls: everything a call does -- its connection, the camera of an incoming video call, the answers to a network change and the return to a call -- now runs in one place, in the order it happens, instead of from several threads at once. This removes a class of rare failures when a call starts, reconnects or comes back.
- The Mute button always shows what the microphone really does, also after iOS refuses a mute request and after a call on hold is resumed.
- Accepting an incoming call uses the one connection prepared while it rang: the accept no longer waits on a timer, and a call is no longer ended because that preparation took a moment longer.
- During a call the app asks the connection for its statistics less often: one measurement instead of two running side by side.

## 1.0 (2211) - 2026-10-15

Source commit `bb7ab8c5e1e8`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2211.md).

- Montana is no longer closed in the background when Do Not Disturb or a Focus holds an incoming call and the caller keeps calling: every call wake-up is now answered to iOS unless a Montana call is already on the screen.
- When iOS ends the app, the diagnostics now name the real reason (for example an unanswered call wake-up) instead of calling it a normal background end.
- Messages waiting for delivery no longer rewrite the whole outgoing queue on every retry: much less disk writing and battery use.

## 1.0 (2210) - 2026-10-14

Source commit `aaeba42ef131`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2210.md).

- Only the caller automatically sends a one-to-one call recording to the chat. Both devices keep their own recording and coin calculation; update both participants.
- Call totals use a coin badge with green earnings and red spending. Tap it to open the Explorer inside Montana.
- Cancelling the back gesture during a system interruption keeps the coin page open.
- Delayed call invitations keep their original start time through retries; an expired call cannot gain another ringing lifetime. VoIP delivery diagnostics now distinguish acceptance from confirmed ringing.
- A detailed guide and FAQ in three languages is available at montana.xxx/faq, with permanent answer links and structured files for assistants.

## 1.0 (2208) - 2026-10-14

Source commit `5e2696bbe7b1`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2208.md).

- Fixed native audio processor attachment so call audio reaches the recorder. The release check now verifies both delegates against the bundled audio library.
- Fixed avatar circle and outline sizing. Balance and level badges no longer stretch the chat rows.
- Text and call bubbles keep their width; complete time and seal fields move to another line when space is short.
- Groups and channels are now tabs on the Chats page. Channels support invitation links and creation before adding subscribers.
- Calls use one native system provider and one audio-session queue. Fixed stale call actions, held-call endings and the headset route control in group calls.
- A blinking red dot shows active recording. A call with no captured audio no longer retries an empty export forever.
- Recorded calls earn Time Coins; unrecorded calls burn them at the same personal rate. The call record shows the signed total. Each participant is counted by recording on their own device.
- Top up and montana.xxx/coins explain the same coin rules in English, Russian and Chinese.

## 1.0 (2207) - 2026-10-14

Source commit `ffd89d2e23c3`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2207.md).

- Fixed avatar circle and outline sizing. Balance and level badges no longer stretch the chat rows.
- Text and call bubbles keep their width; complete time and seal fields move to another line when space is short.
- Groups and channels are now tabs on the Chats page. Channels support invitation links and creation before adding subscribers.
- Calls use one native system provider and one audio-session queue. Fixed stale call actions, held-call endings and the headset route control in group calls.
- A blinking red dot shows active recording. A call with no captured audio no longer retries an empty export forever.
- Recorded calls earn Time Coins; unrecorded calls burn them at the same personal rate. The call record shows the signed total. Each participant is counted by recording on their own device.
- Top up and montana.xxx/coins explain the same coin rules in English, Russian and Chinese.

## 1.0 (2206) - 2026-10-14

Source commit `6aad95e277a2`. [Release notes](doc/release-notes/protocol/ios/release-notes-1.0-2206.md).

- Improved call audio startup. The call timer keeps running when the connection is restored.
- Call diagnostics record each connection stage and its duration in milliseconds.
- Audio and video calls earn coins in every chat mode. The completed call shows your actual earnings in the chat.
- Top up explains minting in plain language and shows the same live rate as Time Coins.
- The left status button shows denied permissions, notifications and available updates, and opens the update for this client.
- The Montana chat automatically shows release notes in the app language, including after restoration.
- Balance and reputation badges sit above the avatar. Balances use readable compact numbers and are omitted where already shown.
- Reputation is always visible, in white, with color based on the current reported level distribution. Balance visibility is controlled in Privacy.
- After your first consent, calls are recorded locally and sent to the chat. You can turn recording off in Privacy.

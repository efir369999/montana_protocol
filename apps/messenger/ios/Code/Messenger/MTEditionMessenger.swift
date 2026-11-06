import Foundation
import SwiftUI

/// MESSENGER'S EDITION (docs/APPS-API.md, rule 3 and slices S5 and S8): Messenger carries no coins -- their files are not in this
/// target, so the door is nil and shared code draws none of them (rule 4). The target's list of files is the switch between
/// editions; a door of an application the Messenger lacks is not even declared here (MTApps.swift, MONTANA_MESSENGER; cuts C4-C6).
enum MTEdition {
    static let coins: MTCoinDoor? = nil
    static let messenger: MTMessengerDoor? = nil   // no Organizations: no roster, shifts, supply or work chats (the author's word 04.11.2026 11:32 UTC, link 16078)
    // NO YOUTUBE PLAYS IN MESSENGER (App Review 5.2.2, critic of S5r, link 15358): a YouTube link opens where it lives -- its door
    // and uses stand under #if !MONTANA_MESSENGER (cut C4). No call is recorded in either edition (the author's word 16428,
    // 04.11.2026).
    /// NO OWN NODE IN MESSENGER (decision 531 (a)): Settings draw no row of it, and it never acts -- a function the person cannot
    /// see never acts for them.
    static let ownNode = false
    // NO LEVELS IN MESSENGER (decision 618 (b), the author's word 16339): their code stands under #if !MONTANA_MESSENGER (cut C3a)
    // -- no badge at the avatar, no line of levels in Settings, no level said to a pair, none published to the nodes' table.
    /// MONTANA MESSENGER'S SITE (App Review 5.1.1(i)): its page and its own privacy policy, montana.xxx/privacy/.
    static let site = "https://montana.xxx"
    /// MONTANA MESSENGER'S FACE (App Review 2.3.7, one face everywhere; decision 549, no coins): its own icon, never Montana
    /// Protocol's coin -- the shared catalog serves both targets, so the Messenger's pictures stand in its own catalog
    /// (Code/Messenger/MessengerAssets.xcassets, rendered by ictool from Code/Messenger/AppIcon.icon).
    static let face = "MessengerGlassPreview"
    static let cover = "MessengerIconPicture"
    /// THE FIRST SCREEN'S LINE (the author's words 16364 and 16543, 04.11.2026: Montana is the anonymous messenger).
    static let intro: LocalizedStringResource = "An anonymous messenger. Your identity comes from your secret phrase, and your chats are encrypted on your device."
}

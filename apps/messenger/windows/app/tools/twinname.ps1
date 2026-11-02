# THE PERSON'S OTHER DEVICE RENAMES ITSELF (28.10.2026: chats on Windows took the person's own name): offtwin.ps1 with
# set names. Q is born on Windows named «Qn<tag>»; the same 24 test words, read into this check's memory only (never a file, a log
# or a shot), open Q in the official APK on the test emulator, named «Qn<tag>» there too. W («Wn<tag>») shows its link, Android-Q
# writes, W answers, Windows-Q (never met W) gets the conversation by the twin lane. Then Android-Q writes once more and is renamed
# «Qm<tag>»: it announces NM: to W -- on the live road unsigned (Pipes.kt: Channels.lay before Wire.withAuthor). Windows-Q holds
# the same pipe key and hears that frame: the title of its W chat is read for 90 s. A build that takes an unsigned self-word off
# a channel names W's chat by Q's own new name; the fixed build (1e54b2d) leaves it to the box (trace «channel_self_word
# unsigned»). W's title of the Android-Q chat is the control (it should become «Qm<tag>»). Also read: on which side Android-Q's
# letters stand in Windows-Q's W chat (mine = Q's own, theirs = W's) and how many times.
# Test identities and the test emulator only; no coins; no server touched beyond a client's own traffic.
# usage: tools\twinname.ps1 -Root DIR -Exe PATH [-Device emulator-5562] [-Watch 90]
param([Parameter(Mandatory = $true)][string]$Root, [Parameter(Mandatory = $true)][string]$Exe, [string]$Device = "emulator-5562", [int]$Watch = 90)
$ErrorActionPreference = "Stop"
. "$PSScriptRoot\ui.ps1"
. "$PSScriptRoot\android.ps1" -Device $Device
# A PERSON MAY BE WORKING AT THIS COMPUTER (never take the mouse or the keyboard): no window of the
# check is brought forward -- each stands beyond the screens, never activated, and UI Automation reads and presses it there
function Front-Montana([int]$X = -1, [int]$Y = 0, [int]$Width = 0, [int]$Height = 0) { Away-Montana }
$script:exe = $PSBoundParameters.Exe   # -Exe and ui.ps1's $script:exe are one variable (names know no case): ui.ps1 above overwrote it
$env:MONTANA_TRACE = "1"
$env:MONTANA_CLOCK_SHIFT_DAYS = "0"
New-Item -ItemType Directory -Force $Root | Out-Null
$Log = Join-Path $Root "twinname.log"
$script:checks = @()
function Note([string]$s) { $l = "{0:HH:mm:ss} {1}" -f (Get-Date), $s; $l; try { Add-Content -Path $Log -Value $l -Encoding UTF8 } catch {} }   # a reader holding the log never stops the scene
function Check([string]$what, [bool]$ok, [string]$actual = "") {
    $script:checks += [pscustomobject]@{ what = $what; ok = $ok; actual = $actual }
    Note ("  {0} {1}{2}" -f $(if ($ok) { "PASS" } else { "FAIL" }), $what, $(if ($actual) { "  ($actual)" } else { "" }))
}
function Grid-Words {
    $w = New-Object string[] 24; $n = $null
    foreach ($e in (All-Named)) {
        $s = $e.Current.Name
        if ($s -match '^(\d{1,2})\.?$' -and [int]$Matches[1] -ge 1 -and [int]$Matches[1] -le 24) { $n = [int]$Matches[1]; continue }
        if ($s -eq ".") { continue }
        if ($n -and $s -match '^[a-z]{3,8}$') { $w[$n - 1] = $s }
        $n = $null
    }
    if (@($w | Where-Object { $_ }).Count -ne 24) { throw "the grid gave $(@($w | Where-Object { $_ }).Count) of 24 words" }
    $w
}
# the phrase field emptied, then the words typed four at a time (one long «input text» is cut short)
function Droid-Phrase([string[]]$w) {
    $field = @(Droid-Nodes | Where-Object { $_.Class -like "*EditText*" }) | Select-Object -First 1
    if (-not $field) { throw "android: no phrase field" }
    Droid-TapAt $field.X $field.Y
    Droid-Adb shell input keyevent KEYCODE_MOVE_END | Out-Null
    Droid-Adb shell ("input keyevent " + ((1..260 | ForEach-Object { "67" }) -join " ")) | Out-Null
    for ($i = 0; $i -lt 24; $i += 4) { Droid-Type ((($w[$i..($i + 3)]) -join " ") + $(if ($i -lt 20) { " " } else { "" })) }
}
function Win-List { for ($i = 0; $i -lt 5; $i++) { try { Click-Named "Назад" 2 } catch { break }; Start-Sleep -Milliseconds 400; if ((Get-Texts) -contains "Чаты") { break } }; Click-Named "Чаты"; Start-Sleep -Milliseconds 700 }
function Win-Send([string]$text) { Front-Montana; Type-Into "Сообщение..." $text 30; Click-Named "Отправить" 15; Sent-Settle 60 }
function Win-Terms {
    $end = (Get-Date).AddSeconds(20)
    do { $tx = Get-Texts; if ($tx -contains "Принимаю") { Click-Named "Принимаю"; break }; if ($tx -contains "Чаты") { break }; Start-Sleep -Milliseconds 400 } while ((Get-Date) -lt $end)
}
# the names of this check on a window: whatever of «Qn<tag>», «Qm<tag>», «Wn<tag>» it shows (a chat's title, a row)
function Win-Names($p) { Use-Montana $p; @(Get-Texts | Where-Object { $_ -match "^(Qn|Qm|Wn)$($script:tag)$" }) }
# ON WHICH SIDE A LETTER STANDS (Conversation.jsx: «bubble-row mine» or «bubble-row theirs»), and in how many rows
function Row-Side([string]$text) {
    $walker = [System.Windows.Automation.TreeWalker]::RawViewWalker
    $rows = @{}
    foreach ($e in @(All-Named)) {
        $n = $null; try { $n = $e.Current.Name } catch {}
        if ($n -ne $text) { continue }
        $p = $e
        for ($i = 0; $i -lt 8 -and $p; $i++) {
            $cls = ""; try { $cls = " " + $p.Current.ClassName + " " } catch {}
            if ($cls -like "* bubble-row *") {
                $side = if ($cls -like "* mine *") { "mine" } elseif ($cls -like "* theirs *") { "theirs" } else { "?" }
                $rows[($p.GetRuntimeId() -join ".")] = $side
                break
            }
            try { $p = $walker.GetParent($p) } catch { $p = $null }
        }
    }
    if (-not $rows.Count) { return "none" }
    "{0} row(s): {1}" -f $rows.Count, (($rows.Values | Sort-Object) -join ",")
}
# the conversation open is known by its own keys (the list behind it keeps its «Search» field in the tree); W's chat is pressed
# by W's name, or -- should Android have renamed it -- by whatever name of this check its row wears
function Droid-Chat {
    if (@(Droid-Nodes | Where-Object { $_.Desc -eq "Voice Call" }).Count) { return }
    try { Droid-Tap $script:nW 20; return } catch {}
    $any = @(Droid-Nodes | Where-Object { $_.Text -match "^(Qn|Qm|Wn)$($script:tag)$" }) | Select-Object -First 1
    if (-not $any) { throw "android: no row of W" }
    Note "  android: W's chat row reads «$($any.Text)»"
    Droid-TapAt $any.X $any.Y
}
function Droid-Send([string]$text) {
    # three tries: a conversation just opened by a link may not take the first touch (twin4: no send key after typing)
    for ($try = 1; $try -le 3; $try++) {
        Droid-Chat; Start-Sleep -Seconds 1
        # the view tree holds every screen of the stack: a person's card or a wall's new post laid over the conversation
        # (twin6) is left by Back -- never published -- until the conversation is the screen on top
        for ($b = 0; $b -lt 5; $b++) {
            if (-not @(Droid-Texts | Where-Object { @("New post", "Fingerprint verification") -contains $_ }).Count) { break }
            Droid-Back; Start-Sleep -Seconds 1
            $d = @(Droid-Nodes | Where-Object { @("Discard", "Delete", "Don't save", "Exit") -contains $_.Text }) | Select-Object -First 1
            if ($d) { Droid-TapAt $d.X $d.Y; Start-Sleep -Seconds 1 }
        }
        # only the conversation's own field, by its hint «Message…» (twin6 typed into a wall's new post: never another field);
        # the official app's input bar reads as bounds 0,0 until it is touched: then it is pressed where it stands
        $f = @(Droid-Nodes | Where-Object { $_.Class -like "*EditText*" -and $_.Text -like "Message*" }) | Select-Object -First 1
        if ($f -and $f.Y -gt 0) { Droid-TapAt $f.X $f.Y }
        else {
            $size = [regex]::Match(((Droid-Adb shell wm size) -join ""), '(\d+)x(\d+)')
            Droid-TapAt ([int]$size.Groups[1].Value / 2) ([int]$size.Groups[2].Value - 203)
        }
        Start-Sleep -Milliseconds 800
        Droid-Adb shell input keyevent KEYCODE_MOVE_END | Out-Null
        Droid-Adb shell ("input keyevent " + ((1..40 | ForEach-Object { "67" }) -join " ")) | Out-Null
        Droid-Type $text; Start-Sleep -Seconds 1
        $send = @(Droid-Nodes | Where-Object { @("Send", "Отправить") -contains $_.Desc }) | Select-Object -First 1
        if ($send) { Droid-TapAt $send.X $send.Y; Start-Sleep -Seconds 2; return }
        Note ("  android send try $try found no send key; keys on screen: " + ((Droid-Nodes | Where-Object { $_.Desc } | ForEach-Object { $_.Desc }) -join ", "))
        # whatever took the words is left unsent: back out of it (a post or a form is never published by the check)
        if (@(Droid-Texts | Where-Object { $_ -eq "New post" }).Count) { Droid-Back; Start-Sleep -Seconds 1 }
        Start-Sleep -Seconds 3
    }
    throw "android: no send key after 3 tries"
}
function Droid-Arrives([string]$text, [int]$timeout = 150) {
    $end = (Get-Date).AddSeconds($timeout); $ok = $false
    while (-not $ok -and (Get-Date) -lt $end) {
        try { Droid-Chat } catch {}
        $ok = [bool](@(Droid-Nodes | Where-Object { $_.Text -like "*$text*" }).Count)
        if (-not $ok) { Start-Sleep -Seconds 4 }
    }
    Check "Android: «$text» arrives" $ok
}
# THE ANDROID PERSON'S NAME (the official app, 584): the drawer's glyph (the top bar's first slot), the drawer's face opens my page,
# its «Edit» mark (pressed a moment: a bare tap is not taken) opens the profile, the name field is the first field under the face,
# the mark in the top right corner is Done
function Droid-Name([string]$name) {
    for ($b = 0; $b -lt 5; $b++) { if (-not @(Droid-Nodes | Where-Object { $_.Desc -eq "Voice Call" }).Count) { break }; Droid-Back; Start-Sleep -Seconds 1 }
    Droid-TapAt 105 238; Start-Sleep -Seconds 2
    Droid-TapAt 132 300; Start-Sleep -Seconds 3
    $e = @(Droid-Nodes | Where-Object { $_.Desc -eq "Edit" }) | Select-Object -First 1
    if (-not $e) { throw "android: my page shows no Edit mark" }
    Droid-Adb shell input swipe $e.X $e.Y $e.X $e.Y 120 | Out-Null; Start-Sleep -Seconds 3
    $f = @(Droid-Nodes | Where-Object { $_.Class -like "*EditText*" -and $_.Y -gt 600 } | Sort-Object Y) | Select-Object -First 1
    if (-not $f) { throw "android: the profile shows no name field" }
    Droid-TapAt $f.X $f.Y
    Droid-Adb shell input keyevent KEYCODE_MOVE_END | Out-Null
    Droid-Adb shell ("input keyevent " + ((1..40 | ForEach-Object { "67" }) -join " ")) | Out-Null
    Droid-Type $name; Start-Sleep -Milliseconds 800
    Droid-TapAt 998 216; Start-Sleep -Seconds 3
    $ok = [bool](@(Droid-Texts | Where-Object { $_ -eq $name }).Count)
    Droid-Back; Start-Sleep -Seconds 2
    Check "Android: the person is named «$name»" $ok
}
function Trace-Lines([string]$dir, [string]$pattern) {
    $t = Join-Path $dir "trace.log"
    if (Test-Path $t) { @(Select-String -Path $t -Pattern $pattern | ForEach-Object { $_.Line }) } else { @() }
}

$script:tag = "{0:D4}" -f (Get-Random -Maximum 9999)
$tag = $script:tag
$script:nQ = "Qn$tag"; $script:nQ2 = "Qm$tag"; $script:nW = "Wn$tag"
$words = $null
$t0 = Get-Date
try {
    Note "== 1. Q born on Windows, named «$($script:nQ)»"
    $script:Q = Start-Montana -DataDir "$Root\Q"; Front-Montana -X 0 -Y 40 -Width 900 -Height 900; Start-Sleep -Seconds 3
    Click-Named "Продолжить с Montana"; Click-Named "Создать"; Click-Named "Готово"
    Click-Named "Я понимаю и беру ответственность на себя"; Click-Named "Показать слова"
    Find-Named "Ваша сид-фраза из 24 слов" | Out-Null; Start-Sleep -Milliseconds 300
    $words = Grid-Words
    Click-Named "Я записал слова в надёжном месте"; Click-Named "Ваша личность в Montana"
    Find-Named "Готово" | Out-Null; Start-Sleep -Milliseconds 800
    Type-First-Field $script:nQ; Click-Named "Готово"; Win-Terms; Win-List

    Note "== 2. the official Android opens an identity by the same words and names it «$($script:nQ)»"
    Droid-Adb shell pm clear $script:Package | Out-Null
    Droid-Start; Start-Sleep -Seconds 12
    Droid-Tap "Continue with Montana" 30; Start-Sleep -Seconds 6
    Droid-Tap "Open" 30; Start-Sleep -Seconds 6
    Droid-Phrase $words
    $words = $null
    Start-Sleep -Seconds 1
    $typed = @(Droid-Texts | Where-Object { $_ -cmatch '^\s*([a-z]{2,8}\s+){23}[a-z]{2,8}\s*$' }).Count
    Check "Android: the phrase field holds 24 words" ([bool]$typed)
    Droid-Tap "Restore" 20 -Exact
    $end = (Get-Date).AddSeconds(60); $agreed = $false
    while (-not $agreed -and (Get-Date) -lt $end) {
        Start-Sleep -Seconds 4
        if (@(Droid-Texts | Where-Object { $_ -eq "Terms of Use" }).Count) {
            for ($i = 0; $i -lt 6; $i++) { Droid-Adb shell input swipe 540 1800 540 500 300 | Out-Null; Start-Sleep -Milliseconds 500 }
            $ag = @(Droid-Nodes | Where-Object { $_.Text -eq "I agree" -and $_.Y -gt 0 }) | Select-Object -First 1
            if ($ag) { Droid-TapAt $ag.X $ag.Y } else { Droid-TapAt 540 2103 }
            $agreed = $true
        }
    }
    $listed = $false; $end = (Get-Date).AddSeconds(60)
    while (-not $listed -and (Get-Date) -lt $end) { Start-Sleep -Seconds 4; $listed = [bool](@(Droid-Texts | Where-Object { $_ -eq "Saved Messages" }).Count) }
    Check "Android: the identity opened by the words stands on its chat list" $listed
    Droid-Name $script:nQ

    Note "== 3. W born on Windows, named «$($script:nW)»; the Android person opens W's link and writes"
    $script:W = Start-Montana -DataDir "$Root\W"; Front-Montana -X 460 -Y 40 -Width 900 -Height 900; Start-Sleep -Seconds 3
    Click-Named "Продолжить с Montana"; Click-Named "Создать"; Click-Named "Готово"
    Click-Named "Я понимаю и беру ответственность на себя"; Click-Named "Показать слова"
    Click-Named "Я записал слова в надёжном месте"; Click-Named "Ваша личность в Montana"
    Find-Named "Готово" | Out-Null; Start-Sleep -Milliseconds 500
    Type-First-Field $script:nW; Click-Named "Готово"; Win-Terms
    Win-List; Click-Named "Написать"; Click-Named "Мой код и ссылка"
    $link = $null
    for ($i = 0; $i -lt 40 -and -not $link; $i++) { $link = Get-Texts | Where-Object { $_ -like "https://montana.quest/perp/*" } | Select-Object -First 1; Start-Sleep -Milliseconds 300 }
    if (-not $link) { throw "W shows no link" }
    try { Click-Named "Закрыть" 5 } catch {}
    # a card just made may not stand on the node yet (twin5: «The invitation could not be opened»): OK, 20 s, open again
    for ($try = 1; $try -le 3; $try++) {
        Droid-Open $link; Start-Sleep -Seconds 15
        if (-not @(Droid-Texts | Where-Object { $_ -like "The invitation could not be opened*" }).Count) { break }
        Note "  android: the invitation could not be opened (try $try); OK, again in 20 s"
        try { Droid-Tap "OK" 5 -Exact } catch {}
        Start-Sleep -Seconds 20
    }
    Droid-Send "a2w-$tag"

    Note "== 4. W hears the Android person and answers"
    Front-Montana; Win-List
    $row = $null; $end = (Get-Date).AddSeconds(150)
    while (-not $row -and (Get-Date) -lt $end) {
        $tx = Get-Texts
        $row = @($script:nQ, "Собеседник") | Where-Object { $tx -contains $_ } | Select-Object -First 1
        if (-not $row) { Start-Sleep -Seconds 3 }
    }
    Check "W: the Android person comes as «$($script:nQ)»" ($row -eq $script:nQ) "row: $row"
    if (-not $row) { throw "W never heard the Android person" }
    Click-Named $row; Start-Sleep -Seconds 1
    $heard = $true; try { Find-Named "a2w-$tag" 60 | Out-Null } catch { $heard = $false }
    Check "W: the Android person's «a2w-$tag» arrives" $heard
    if (-not $heard) { throw "W never heard the Android person's letter" }
    Win-Send "w2a-$tag"; Droid-Arrives "w2a-$tag"

    Note "== 5. Q, the same words on Windows, never met W: the conversation reaches it by the twin lane"
    Use-Montana $script:Q; Front-Montana; Win-List
    $chat = $null; $end = (Get-Date).AddSeconds(240)
    while (-not $chat -and (Get-Date) -lt $end) { $chat = @(Win-Names $script:Q) | Select-Object -First 1; if (-not $chat) { Start-Sleep -Seconds 3 } }
    Check "Q: the conversation with W stands on the same person's Windows device, titled «$($script:nW)»" ($chat -eq $script:nW) "title: $chat"
    if (-not $chat) { throw "Q never got the conversation with W" }
    Click-Named $chat; Start-Sleep -Seconds 2
    foreach ($l in "a2w-$tag", "w2a-$tag") {
        $has = $true; try { Find-Named $l 60 | Out-Null } catch { $has = $false }
        Check "Q: «$l» stands in it" $has ("side: " + (Row-Side $l))
    }

    Note "== 6. Android-Q writes again, then is renamed «$($script:nQ2)»; Q's W chat and W's Android-Q chat are read"
    Note ("  before: Q shows [{0}]; W shows [{1}]" -f ((Win-Names $script:Q) -join ", "), ((Win-Names $script:W) -join ", "))
    Droid-Send "a2q-$tag"
    Use-Montana $script:Q
    $has = $true; try { Find-Named "a2q-$tag" 90 | Out-Null } catch { $has = $false }
    Start-Sleep -Seconds 5
    Check "Q: Android-Q's later «a2q-$tag» stands in Q's W chat" $has ("side: " + (Row-Side "a2q-$tag"))
    Droid-Name $script:nQ2
    $renamed = Get-Date
    $lastQ = ""; $lastW = ""; $end = $renamed.AddSeconds($Watch)
    while ((Get-Date) -lt $end) {
        # not $q and $w: names know no case, and at the script's level they are $script:Q and $script:W, the two processes
        $qNow = (Win-Names $script:Q) -join ", "; $wNow = (Win-Names $script:W) -join ", "
        if ($qNow -ne $lastQ -or $wNow -ne $lastW) { Note ("  +{0} s: Q's W chat [{1}]; W's Android-Q chat [{2}]" -f [int]((Get-Date) - $renamed).TotalSeconds, $qNow, $wNow); $lastQ = $qNow; $lastW = $wNow }
        Start-Sleep -Seconds 5
    }
    $qNames = @(Win-Names $script:Q); $wNames = @(Win-Names $script:W)
    Check "Q: the W chat keeps W's name «$($script:nW)» after the Android rename" (($qNames -contains $script:nW) -and -not ($qNames -contains $script:nQ2) -and -not ($qNames -contains $script:nQ)) ($qNames -join ", ")
    Check "W (control): the Android-Q chat takes the new name «$($script:nQ2)»" ($wNames -contains $script:nQ2) ($wNames -join ", ")
    Note ("  Q: «a2q-$tag» side: " + (Row-Side "a2q-$tag") + "; «a2w-$tag» side: " + (Row-Side "a2w-$tag"))
    try { Use-Montana $script:Q; Shot-Montana "$Root\q-end.png"; Use-Montana $script:W; Shot-Montana "$Root\w-end.png" } catch { Note "  shot: $($_.Exception.Message)" }
    try { Droid-Chat; Start-Sleep -Seconds 2; Note ("  android: W's chat shows [" + ((Droid-Texts | Where-Object { $_ -match "^(Qn|Qm|Wn)$tag$" }) -join ", ") + "]") } catch { Note "  android: $($_.Exception.Message)" }
} catch {
    Check "the scene ran to its end" $false $_.Exception.Message
} finally {
    $words = $null
    foreach ($n in "Q", "W") {
        $d = "$Root\$n"
        Note ("  {0} trace: of_mine {1}, channel_self_word {2}, twin_ {3}" -f $n, @(Trace-Lines $d "of_mine").Count, @(Trace-Lines $d "channel_self_word").Count, @(Trace-Lines $d "twin_").Count)
        foreach ($l in @(Trace-Lines $d "of_mine|channel_self_word" | Select-Object -Last 8)) { Note "    $l" }
    }
    foreach ($p in $script:W, $script:Q) { if ($p) { try { Use-Montana $p; Stop-Montana } catch {} } }
    $pass = @($script:checks | Where-Object { $_.ok }).Count
    Note ("RESULT: {0} of {1} checks pass, {2} s" -f $pass, $script:checks.Count, [int]((Get-Date) - $t0).TotalSeconds)
    $script:checks | ConvertTo-Json | Set-Content -Path (Join-Path $Root "checks.json") -Encoding UTF8
}

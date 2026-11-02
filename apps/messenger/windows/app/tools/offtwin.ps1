# THE SAME PERSON ON WINDOWS AND ON THE OFFICIAL ANDROID, BY THE TWIN LANE (the owner's word 27.10.2026): a test
# identity Q is born on Windows and named by its call sign (the name left empty: iOS ProfileView onboarding Done); the same 24
# test words, read into this check's memory only (never a file, a log or a shot), open an identity in the official APK on the
# test emulator. A second Windows person W shows its link, the Android person opens it and writes, W answers. Devices of one
# person share the twin lane the words derive (twin.rs, iOS takeMirror): what one of them hears lands on the other -- so the
# conversation with W standing on Q, a device that never met W, is the official core and Windows deriving one person from one
# set of words. Q staying bare of it is no proof of the opposite (the official app may not mirror): it is reported as seen.
# Test identities and the test emulator only; no coins; no server touched.
# usage: tools\offtwin.ps1 -Root DIR -Exe PATH [-Device emulator-5564]
param([Parameter(Mandatory = $true)][string]$Root, [Parameter(Mandatory = $true)][string]$Exe, [string]$Device = "emulator-5562")
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
$Log = Join-Path $Root "offtwin.log"
$script:checks = @()
function Note([string]$s) { $l = "{0:HH:mm:ss} {1}" -f (Get-Date), $s; $l; Add-Content -Path $Log -Value $l -Encoding UTF8 }
function Check([string]$what, [bool]$ok, [string]$actual = "") {
    $script:checks += [pscustomobject]@{ what = $what; ok = $ok; actual = $actual }
    Note ("  {0} {1}{2}" -f $(if ($ok) { "PASS" } else { "FAIL" }), $what, $(if ($actual) { "  ($actual)" } else { "" }))
}
# what a screen says, with anything that could be one of the 24 words starred
function Safe([string]$s) {
    if ($s -cmatch '([a-z]{3,8}\s+){2,}[a-z]{2,8}') { return "<field: " + @($s -csplit '\s+' | Where-Object { $_ }).Count + " words>" }
    return ($s -creplace '\b[a-z]{3,8}\b', '*')
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
# the conversation open is known by its own keys (the list behind it keeps its «Search» field in the tree)
function Droid-Chat {
    if (@(Droid-Nodes | Where-Object { $_.Desc -eq "Voice Call" }).Count) { return }
    Droid-Tap $script:nW 60
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
function Twin-Lines([string]$dir) {
    $t = Join-Path $dir "trace.log"
    if (Test-Path $t) { @(Select-String -Path $t -Pattern "twin_" -SimpleMatch | ForEach-Object { $_.Line }) } else { @() }
}

$tag = Get-Random -Maximum 9999
$script:nW = "Vera Twin $tag"
$words = $null
$t0 = Get-Date
try {
    Note "== 1. Q born on Windows, named by its call sign"
    $script:Q = Start-Montana -DataDir "$Root\Q"; Front-Montana -X 0 -Y 40 -Width 900 -Height 900; Start-Sleep -Seconds 3
    Click-Named "Продолжить с Montana"; Click-Named "Создать"; Click-Named "Готово"
    Click-Named "Я понимаю и беру ответственность на себя"; Click-Named "Показать слова"
    Find-Named "Ваша сид-фраза из 24 слов" | Out-Null; Start-Sleep -Milliseconds 300
    $words = Grid-Words
    Click-Named "Я записал слова в надёжном месте"; Click-Named "Ваша личность в Montana"
    Find-Named "Готово" | Out-Null; Start-Sleep -Milliseconds 800
    $c = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::Edit)
    $f = (Root).FindFirst([System.Windows.Automation.TreeScope]::Descendants, $c)
    $sign = if ($f) { $f.Current.Name } else { "" }
    if (-not $sign -and $f) { $sign = $f.Current.HelpText }
    Note "  Q's call sign: «$sign»"
    Click-Named "Готово"; Win-Terms; Win-List

    Note "== 2. the official Android opens an identity by the same words"
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

    Note "== 3. W born on Windows; the Android person opens W's link and writes"
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
    # the list's row of a named person shows the person's balance, not the last letter: the row is found by the name, the
    # letter inside the conversation
    Front-Montana; Win-List
    $row = $null; $end = (Get-Date).AddSeconds(150)
    while (-not $row -and (Get-Date) -lt $end) {
        $tx = Get-Texts
        $row = @($sign, "Собеседник") | Where-Object { $tx -contains $_ } | Select-Object -First 1
        if (-not $row) { Start-Sleep -Seconds 3 }
    }
    Check "W: the Android person comes under Q's call sign «$sign» (the identity the same words made on Windows)" ($row -eq $sign) "row: $row"
    if (-not $row) { throw "W never heard the Android person" }
    Click-Named $row; Start-Sleep -Seconds 1
    $heard = $true; try { Find-Named "a2w-$tag" 60 | Out-Null } catch { $heard = $false }
    Check "W: the Android person's «a2w-$tag» arrives" $heard
    if (-not $heard) { throw "W never heard the Android person's letter" }
    Win-Send "w2a-$tag"; Droid-Arrives "w2a-$tag"

    Note "== 5. Q, the same words on Windows, never met W: the conversation reaches it by the twin lane"
    Use-Montana $script:Q; Front-Montana; Win-List
    $twin = $true; try { Find-Named $script:nW 240 | Out-Null } catch { $twin = $false }
    Check "Q: the conversation with «$($script:nW)» stands on the same person's Windows device" $twin ((Get-Texts | Where-Object { $_ -match '^\S+ \S+' } | Select-Object -First 8) -join " | ")
    if ($twin) {
        Click-Named $script:nW; Start-Sleep -Seconds 2
        foreach ($l in "a2w-$tag", "w2a-$tag") {
            $has = $true; try { Find-Named $l 60 | Out-Null } catch { $has = $false }
            Check "Q: «$l» stands in it" $has
        }
    }
    Note ("  Q's twin lines: " + ((Twin-Lines "$Root\Q") -join " / "))
} catch {
    Check "the scene ran to its end" $false $_.Exception.Message
} finally {
    $words = $null
    foreach ($p in $script:W, $script:Q) { if ($p) { try { Use-Montana $p; Stop-Montana } catch {} } }
    $pass = @($script:checks | Where-Object { $_.ok }).Count
    Note ("RESULT: {0} of {1} checks pass, {2} s" -f $pass, $script:checks.Count, [int]((Get-Date) - $t0).TotalSeconds)
    $script:checks | ConvertTo-Json | Set-Content -Path (Join-Path $Root "checks.json") -Encoding UTF8
}

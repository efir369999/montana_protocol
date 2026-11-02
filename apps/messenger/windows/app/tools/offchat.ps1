# WINDOWS AND THE OFFICIAL ANDROID 561 WRITE TO EACH OTHER, LIVE (the owner's word 27.10.2026): a Windows test
# identity W (a check's folder) shows its link; the official APK on the test emulator MontanaR -- holding the identity tools\offident.ps1
# restored there from Windows' words, unnamed -- opens it. What W calls the Android person is the call sign of the reference the
# official core derived (unnamed: the sign stands for the name), so it must be the sign Windows derived from the same words (-Sign).
# Then letters both ways directly, through the box each way, a second meeting by the same link, and the days moved to 03.11 (+8)
# and 07.11 (+12) on both sides (Windows by MONTANA_CLOCK_SHIFT_DAYS, the emulator by its own clock, given back at the end).
# Test identities and the test emulator only; no coins (a check's run writes no chain); no server touched.
# usage: tools\offchat.ps1 -Root DIR -Exe PATH -Sign "<emoji> <name>"
param([Parameter(Mandatory = $true)][string]$Root, [Parameter(Mandatory = $true)][string]$Exe, [Parameter(Mandatory = $true)][string]$Sign, [string]$Device = "emulator-5562")
$ErrorActionPreference = "Stop"
. "$PSScriptRoot\ui.ps1"
. "$PSScriptRoot\android.ps1" -Device $Device
$script:exe = $PSBoundParameters.Exe   # -Exe and ui.ps1's $script:exe are one variable (names know no case): ui.ps1 above overwrote it
$env:MONTANA_TRACE = "1"
$env:MONTANA_CLOCK_SHIFT_DAYS = "0"
New-Item -ItemType Directory -Force $Root | Out-Null
$Log = Join-Path $Root "offchat.log"
$script:checks = @()
function Note([string]$s) { $l = "{0:HH:mm:ss} {1}" -f (Get-Date), $s; $l; Add-Content -Path $Log -Value $l -Encoding UTF8 }
function Check([string]$what, [bool]$ok, [string]$actual = "") {
    $script:checks += [pscustomobject]@{ what = $what; ok = $ok; actual = $actual }
    Note ("  {0} {1}{2}" -f $(if ($ok) { "PASS" } else { "FAIL" }), $what, $(if ($actual) { "  ($actual)" } else { "" }))
}
function Win-List { for ($i = 0; $i -lt 5; $i++) { try { Click-Named "Назад" 2 } catch { break }; Start-Sleep -Milliseconds 400; if ((Get-Texts) -contains "Чаты") { break } }; Click-Named "Чаты"; Start-Sleep -Milliseconds 700 }
function Win-Send([string]$text) { Front-Montana; Type-Into "Сообщение..." $text 30; Click-Named "Отправить" 15; Sent-Settle 60 }
function Win-Arrives([string]$text, [int]$timeout = 150) {
    Front-Montana; Win-List; Find-Named $Sign $timeout | Out-Null; Click-Named $Sign
    $ok = $true; try { Find-Named $text $timeout | Out-Null } catch { $ok = $false }
    Check "W: «$text» arrives" $ok
}
function Win-Open([int]$shift) {
    $env:MONTANA_CLOCK_SHIFT_DAYS = "$shift"
    $script:W = Start-Montana -DataDir "$Root\W"; Front-Montana -X 0 -Y 40 -Width 900 -Height 900; Start-Sleep -Seconds 3
    try { Click-Named "Не сейчас" 3 } catch {}
}
# the conversation open is known by its own keys (the list behind it keeps its «Search» field in the tree)
function Droid-Chat {
    if (@(Droid-Nodes | Where-Object { $_.Desc -eq "Voice Call" }).Count) { return }
    Droid-Tap $script:nW 60
}
function Droid-Send([string]$text) {
    Droid-Chat
    # the official app's input bar reads as bounds 0,0 until it is touched: then it is pressed where it stands, at the foot
    # of the screen above the system bar
    $f = @(Droid-Nodes | Where-Object { $_.Class -like "*EditText*" -and $_.Text -ne "Search" }) | Select-Object -First 1
    if ($f -and $f.Y -gt 0) { Droid-TapAt $f.X $f.Y }
    else {
        $size = [regex]::Match(((Droid-Adb shell wm size) -join ""), '(\d+)x(\d+)')
        Droid-TapAt ([int]$size.Groups[1].Value / 2) ([int]$size.Groups[2].Value - 203)
    }
    Start-Sleep -Milliseconds 600; Droid-Type $text; Start-Sleep -Milliseconds 600
    $send = @(Droid-Nodes | Where-Object { @("Send", "Отправить") -contains $_.Desc -or @("Send", "Отправить") -contains $_.Text }) | Select-Object -First 1
    if (-not $send) { throw "android: no send key" }
    Droid-TapAt $send.X $send.Y; Start-Sleep -Seconds 2
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
function Droid-Restart { Droid-Adb shell am force-stop $script:Package | Out-Null; Start-Sleep -Seconds 2; Droid-Start; Start-Sleep -Seconds 12 }
# the emulator's own day, set ahead in UTC (its network time off for the check); 0 gives the network time back
function Droid-Day([int]$shift) {
    Droid-Adb root | Out-Null; Start-Sleep -Seconds 3
    if ($shift -eq 0) { Droid-Adb shell settings put global auto_time 1 | Out-Null }
    else {
        Droid-Adb shell settings put global auto_time 0 | Out-Null
        Droid-Adb shell ("date -u " + [DateTime]::UtcNow.AddDays($shift).ToString("MMddHHmmyyyy.ss")) | Out-Null
    }
    Droid-Restart
    Note ("  android's day: " + ((Droid-Adb shell "date -u +%d.%m.%Y") -join ""))
}

$tag = Get-Random -Maximum 9999
$script:nW = "Vera Off $tag"
$t0 = Get-Date
try {
    Note "== 1. W born on Windows; the official Android opens W's link"
    Win-Open 0
    Click-Named "Продолжить с Montana"; Click-Named "Создать"; Click-Named "Готово"
    Click-Named "Я понимаю и беру ответственность на себя"; Click-Named "Показать слова"
    Click-Named "Я записал слова в надёжном месте"; Click-Named "Ваша личность в Montana"
    Find-Named "Готово" | Out-Null; Start-Sleep -Milliseconds 500
    Type-First-Field $script:nW; Click-Named "Готово"
    $end = (Get-Date).AddSeconds(20)
    do { $tx = Get-Texts; if ($tx -contains "Принимаю") { Click-Named "Принимаю"; break }; if ($tx -contains "Чаты") { break }; Start-Sleep -Milliseconds 400 } while ((Get-Date) -lt $end)
    Win-List; Click-Named "Написать"; Click-Named "Мой код и ссылка"
    $link = $null
    for ($i = 0; $i -lt 40 -and -not $link; $i++) { $link = Get-Texts | Where-Object { $_ -like "https://montana.quest/perp/*" } | Select-Object -First 1; Start-Sleep -Milliseconds 300 }
    if (-not $link) { throw "W shows no link" }
    try { Click-Named "Закрыть" 5 } catch {}
    Droid-Restart
    Droid-Open $link; Start-Sleep -Seconds 15
    Droid-Send "a2w-1-$tag"

    Note "== 2. W hears the Android person: by the call sign the official core's reference gives"
    Front-Montana; Win-List
    $named = $true; try { Find-Named $Sign 150 | Out-Null } catch { $named = $false }
    Check "W names the Android person «$Sign» -- the sign Windows derived from the same words" $named ((Get-Texts | Where-Object { $_ -match '^\S+ \S+$' } | Select-Object -First 6) -join " | ")
    if (-not $named) { throw "the Android person did not come as «$Sign»" }
    Win-Arrives "a2w-1-$tag"
    Win-Send "w2a-1-$tag"; Droid-Arrives "w2a-1-$tag"

    Note "== 3. the box, each way"
    Droid-Adb shell am force-stop $script:Package | Out-Null
    Win-Send "w2a-box-$tag"; Start-Sleep -Seconds 10
    Droid-Start; Start-Sleep -Seconds 12; Droid-Arrives "w2a-box-$tag" 180
    Stop-Montana; Start-Sleep -Seconds 2
    Droid-Send "a2w-box-$tag"; Start-Sleep -Seconds 10
    Win-Open 0; Win-Arrives "a2w-box-$tag" 180

    Note "== 4. a second meeting by the same link"
    Droid-Open $link; Start-Sleep -Seconds 12
    Droid-Adb shell input keyevent 4 | Out-Null; Start-Sleep -Seconds 2
    $rows = @(Droid-Nodes | Where-Object { $_.Text -eq $script:nW }).Count
    Check "Android: the conversation with W stands once after the second meeting" ($rows -le 1) "rows named so: $rows"
    Front-Montana; Win-List
    $wrows = @((Get-Texts) | Where-Object { $_ -eq $Sign }).Count
    Check "W: the conversation with the Android person stands once" ($wrows -eq 1) "rows named so: $wrows"

    foreach ($d in 8, 12) {
        Note "== 5. the days moved: +$d on both sides, letters direct"
        Stop-Montana; Start-Sleep -Seconds 2
        Droid-Day $d
        Win-Open $d
        Win-List; Click-Named $Sign
        Win-Send "w2a-d$d-$tag"; Droid-Arrives "w2a-d$d-$tag"
        Droid-Send "a2w-d$d-$tag"; Win-Arrives "a2w-d$d-$tag"
        $t = Get-ChildItem -Path "$Root\W" -Recurse -Filter "trace.log" -ErrorAction SilentlyContinue | Select-Object -First 1
        if ($t) { Note ("  W's last floor line: " + ((Select-String -Path $t.FullName -Pattern "floor_" -SimpleMatch | Select-Object -Last 1).Line)) }
    }
    $t = Get-ChildItem -Path "$Root\W" -Recurse -Filter "trace.log" -ErrorAction SilentlyContinue | Select-Object -First 1
    $fl = if ($t) { (Select-String -Path $t.FullName -Pattern "pipes_floored" -SimpleMatch | Select-Object -Last 1).Line } else { "" }
    Check "W's book floored at +12 while talking with the official Android" ([bool]$fl) $fl
    Front-Montana; Win-List; Click-Named $Sign
    $old = $true; try { Find-Named "a2w-1-$tag" 20 | Out-Null } catch { $old = $false }
    Check "W: the first letter still stands after the floor" $old
} catch {
    Check "the scene ran to its end" $false $_.Exception.Message
} finally {
    try { Stop-Montana } catch {}
    try { Droid-Day 0 } catch {}
    $pass = @($script:checks | Where-Object { $_.ok }).Count
    Note ("RESULT: {0} of {1} checks pass, {2} s" -f $pass, $script:checks.Count, [int]((Get-Date) - $t0).TotalSeconds)
    $script:checks | ConvertTo-Json | Set-Content -Path (Join-Path $Root "checks.json") -Encoding UTF8
}

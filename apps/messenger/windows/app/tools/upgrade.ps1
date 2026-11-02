# THE UPGRADE OF A LIVE CONVERSATION FROM THE INSTALLED LINE TO THE FLOOR (the owner's word 26.10.2026 21:4x):
# two throw-away identities, each in a check's own folder (MONTANA_DATA_DIR) and never the person's Montana, talk through
# Montana's own nodes on the OLD build (the line installed on 26.10.2026), directly and through the box; then the NEW build
# (the floor) opens the same folders: the conversation must stand as it was, once. Then a restart, direct and box letters
# again, the days moved by MONTANA_CLOCK_SHIFT_DAYS (a check's run only) to 03.11 (the clock guard holds the first floor) and
# to 07.11 (the floor applies), a restore by the words into a fresh folder, and the old build opened on a copy of the migrated
# folder -- after which the new build must still find the conversation whole. Test identities, test words read into this
# check's memory only (never a file, a log or a shot), no coins (a check's run writes no chain), no server touched.
# usage: tools\upgrade.ps1 -Root DIR -OldDir DIR -NewDir DIR   (each Dir holds montana-desktop.exe and its two DLLs)
param([Parameter(Mandatory = $true)][string]$Root, [Parameter(Mandatory = $true)][string]$OldDir,
      [Parameter(Mandatory = $true)][string]$NewDir)
$ErrorActionPreference = "Stop"
. "$PSScriptRoot\ui.ps1"
if (Test-Path $Root) { Remove-Item -Recurse -Force $Root }
New-Item -ItemType Directory -Force $Root | Out-Null
$Log = Join-Path $Root "upgrade.log"
$env:MONTANA_TRACE = "1"
$env:MONTANA_KEEP_WAIT = "5"   # a renewal waits five seconds for more keepers, a check's own (tools\keeping.ps1)
$env:MONTANA_CLOCK_SHIFT_DAYS = "0"
$script:checks = @()
function Note([string]$s) { $l = "{0:HH:mm:ss} {1}" -f (Get-Date), $s; $l; Add-Content -Path $Log -Value $l -Encoding UTF8 }
function Check([string]$what, [bool]$ok, [string]$actual = "") {
    $script:checks += [pscustomobject]@{ what = $what; ok = $ok; actual = $actual }
    Note ("  {0} {1}{2}" -f $(if ($ok) { "PASS" } else { "FAIL" }), $what, $(if ($actual) { "  ($actual)" } else { "" }))
}
function Use-Build([string]$dir) { $script:exe = Join-Path $dir "montana-desktop.exe"; Note "build: $script:exe" }
function Shift([int]$days) { $env:MONTANA_CLOCK_SHIFT_DAYS = "$days"; Note "clock shift: +$days days" }
function Settle-Asks {
    for ($i = 0; $i -lt 3; $i++) { if ((Get-Texts) -contains "Не сейчас") { try { Click-Named "Не сейчас" 3 } catch {} ; Start-Sleep -Milliseconds 600 } }
}
function Open-Person([string]$dir, [int]$x) {
    $p = Start-Montana -DataDir $dir; Front-Montana -X $x -Y 40 -Width 900 -Height 900; Start-Sleep -Seconds 3; Settle-Asks; $p
}
# the grid's words, read into this check's memory only (never a file, a log or a shot) -- as tools\leave.ps1 reads them
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
function Born([string]$name) {
    Click-Named "Продолжить с Montana"; Click-Named "Создать"; Click-Named "Готово"
    Click-Named "Я понимаю и беру ответственность на себя"; Click-Named "Показать слова"
    Find-Named "Ваша сид-фраза из 24 слов" | Out-Null; Start-Sleep -Milliseconds 300
    $w = Grid-Words
    Click-Named "Я записал слова в надёжном месте"; Click-Named "Ваша личность в Montana"
    Find-Named "Готово" | Out-Null; Start-Sleep -Milliseconds 500
    Type-First-Field $name; Click-Named "Готово"
    $end = (Get-Date).AddSeconds(20)
    do { $tx = Get-Texts; if ($tx -contains "Принимаю") { Click-Named "Принимаю"; break }; if ($tx -contains "Чаты") { break }; Start-Sleep -Milliseconds 400 } while ((Get-Date) -lt $end)
    $w
}
# back to the chat list from a chat or from deep in the settings: «Назад» until the list's tab shows, at most five times
function To-List {
    for ($i = 0; $i -lt 5; $i++) { try { Click-Named "Назад" 2 } catch { break }; Start-Sleep -Milliseconds 500; if ((Get-Texts) -contains "Чаты") { break } }
    Click-Named "Чаты"; Start-Sleep -Milliseconds 800; Settle-Asks
}
# the conversation stands once on the list and holds every letter named
function Holds([string]$who, [string]$peer, [string[]]$letters, [int]$timeout = 90) {
    To-List
    Find-Named $peer $timeout | Out-Null
    $rows = @((Get-Texts) | Where-Object { $_ -eq $peer }).Count
    Check "$who`: the conversation with $peer stands once on the list" ($rows -eq 1) "rows named so: $rows"
    Click-Named $peer
    $missing = @()
    foreach ($l in $letters) { try { Find-Named $l 20 | Out-Null } catch { $missing += $l } }
    Check "$who`: every letter stands ($($letters.Count))" ($missing.Count -eq 0) $(if ($missing) { "missing: " + ($missing -join ", ") } else { "" })
}
function Data-Page { Open-Drawer; Click-Named "Настройки"; Click-Named "Данные и память"; Find-Named "КОПИЯ У СОБЕСЕДНИКОВ" 10 | Out-Null }
# a letter sent: typed, then «Отправить»; a text the window dropped before it was sent is typed again (at most three tries, each
# retry written to the log -- what a person would see); at the last, what the window shows is kept with a shot (test identities)
# my own link (the identity's public name): «Написать» → «Мой код и ссылка», read, closed (as tools\qr.ps1)
function My-Link {
    To-List; Click-Named "Написать"; Click-Named "Мой код и ссылка"
    $l = $null
    for ($i = 0; $i -lt 40 -and -not $l; $i++) { $l = Get-Texts | Where-Object { $_ -like "https://montana.quest/perp/*" } | Select-Object -First 1; Start-Sleep -Milliseconds 300 }
    try { Click-Named "Закрыть" 5 } catch {}
    $l
}
function Send([string]$text) {
    for ($try = 1; $try -le 3; $try++) {
        Front-Montana
        try { Type-Into "Сообщение..." $text 30; Click-Named "Отправить" 8; Sent-Settle 60; return }
        catch {
            if ($try -lt 3) { Note "  retry $try`: «$text» not sent ($($_.Exception.Message)); typed again in 5 s"; Start-Sleep -Seconds 5; continue }
            Note ("  the window shows: " + ((Get-Texts | Where-Object { $_ }) -join " | "))
            try { Shot-Montana (Join-Path $Root ("send-failed-" + (Get-Date -Format "HHmmss") + ".png")); Note "  a shot of the window is in -Root" } catch {}
            throw
        }
    }
}
function Arrives([string]$who, [string]$peer, [string]$text, [int]$timeout = 120) {
    To-List; Find-Named $peer $timeout | Out-Null; Click-Named $peer
    $ok = $true; try { Find-Named $text $timeout | Out-Null } catch { $ok = $false }
    Check "$who`: «$text» arrives" $ok
}
function Trace-Has([string]$dir, [string]$pattern) {
    $t = Get-ChildItem -Path $dir -Recurse -Filter "trace.log" -ErrorAction SilentlyContinue | Select-Object -First 1
    if (-not $t) { return "" }
    (Select-String -Path $t.FullName -Pattern $pattern -SimpleMatch | Select-Object -Last 1).Line
}
# the near road stands once a channel named its machine's owner (channels::lay): a letter typed before it rides the box alone, and
# on moved days the box, keeping the node's real minute, brings nothing (run16 28.10.2026: B typed 6 s before its path stood)
function Owner-Refs([string]$dir) {
    $t = Get-ChildItem -Path $dir -Recurse -Filter "trace.log" -ErrorAction SilentlyContinue | Select-Object -First 1
    if (-not $t) { return 0 }
    @(Select-String -Path $t.FullName -Pattern "channels owner_ref" -SimpleMatch).Count
}
# (the receiver too: a frame laid before its path stands reaches nobody -- run16 phases 8 and 8b, B's path 8.6 s after the copy laid)
function Refs-Now([string[]]$dirs) { , @($dirs | ForEach-Object { Owner-Refs $_ }) }
function Wait-Paths([string[]]$dirs, [int[]]$before, [int]$timeout = 60) {
    $end = (Get-Date).AddSeconds($timeout)
    while ((Get-Date) -lt $end) {
        $late = @(0..($dirs.Count - 1) | Where-Object { (Owner-Refs $dirs[$_]) -le $before[$_] })
        if (-not $late.Count) { break }
        Start-Sleep -Milliseconds 500
    }
    Note ("  near road: " + ((0..($dirs.Count - 1) | ForEach-Object { "{0} +{1}" -f (Split-Path $dirs[$_] -Leaf), ((Owner-Refs $dirs[$_]) - $before[$_]) }) -join ", ") + " owner_ref lines")
}
function Stop-All { foreach ($p in @($script:A, $script:B, $script:A2, $script:C)) { if ($p -and -not $p.HasExited) { $p.Kill(); $p.WaitForExit(10000) | Out-Null } } ; Start-Sleep -Seconds 2 }

$t0 = Get-Date
$tag = Get-Random -Maximum 9999
$nA, $nB = "Алиса Апгрейд $tag", "Борис Апгрейд $tag"
$L = @(1..13 | ForEach-Object { "u$_ $tag" })
try {
    Note "== 1. the OLD build: two identities born, letters direct and through the box"
    Use-Build $OldDir
    $script:A = Open-Person "$Root\A" 0
    $wordsA = Born $nA
    $link = $null
    for ($i = 0; $i -lt 40 -and -not $link; $i++) { $link = Get-Texts | Where-Object { $_ -like "https://montana.quest/perp/*" } | Select-Object -First 1; Start-Sleep -Milliseconds 300 }
    if (-not $link) { throw "A shows no link" }
    $script:B = Open-Person "$Root\B" 920
    Born $nB | Out-Null
    Click-Named "Сканировать"; Type-Into "Ссылка" $link; Click-Named "Открыть"
    Find-Named $nA 60 | Out-Null
    Send $L[0]
    Use-Montana $script:A; Front-Montana; Arrives "A" $nB $L[0]; Send $L[1]
    Use-Montana $script:B; Front-Montana; Arrives "B" $nA $L[1]
    Use-Montana $script:A; Stop-Montana; Start-Sleep -Seconds 2
    Use-Montana $script:B; Front-Montana; Send $L[2]; Start-Sleep -Seconds 10
    $script:A = Open-Person "$Root\A" 0; Arrives "A (box, old)" $nB $L[2] 180
    Holds "A old" $nB $L[0..2]; Use-Montana $script:B; Front-Montana; Holds "B old" $nA $L[0..2]
    Note "  the OLD build keeps A's copy with B (the backup a restore must still take after the upgrade)"
    Use-Montana $script:A; Front-Montana; To-List; Data-Page
    Click-Named "Хранить мою копию у собеседников"
    $kept = $true; try { Find-Named "Хранят 1 из 1" 240 | Out-Null } catch { $kept = $false }
    Check "A old: its copy is kept by B («Хранят 1 из 1»)" $kept
    Stop-All

    Note "== 2. the NEW build opens the same folders"
    Use-Build $NewDir
    $script:A = Open-Person "$Root\A" 0; Holds "A upgraded" $nB $L[0..2]
    $l2 = My-Link
    Check "A upgraded: its identity's link is the one the old build gave" ($l2 -eq $link) "$(if ($l2) { $l2.Substring(0, [Math]::Min(40, $l2.Length)) })…"
    $script:B = Open-Person "$Root\B" 920; Holds "B upgraded" $nA $L[0..2]

    Note "== 3. a restart"
    Stop-All
    $script:A = Open-Person "$Root\A" 0; Holds "A restarted" $nB $L[0..2]
    $script:B = Open-Person "$Root\B" 920; Holds "B restarted" $nA $L[0..2]

    Note "== 4. direct letters on the new build"
    Send $L[3]; Use-Montana $script:A; Front-Montana; Arrives "A" $nB $L[3]; Send $L[4]
    Use-Montana $script:B; Front-Montana; Arrives "B" $nA $L[4]

    Note "== 5. a box letter on the new build"
    Use-Montana $script:B; Stop-Montana; Start-Sleep -Seconds 2
    Use-Montana $script:A; Front-Montana; Send $L[5]; Start-Sleep -Seconds 10
    $script:B = Open-Person "$Root\B" 920; Arrives "B (box, new)" $nA $L[5] 180

    Note "== 6. after the upgrade A's copy, laid by the old build, is still kept (the switch is not pressed again)"
    Use-Montana $script:A; Front-Montana; To-List; Data-Page
    $kept = $true; try { Find-Named "Хранят 1 из 1" 60 | Out-Null } catch { $kept = $false }
    Check "A upgraded: its copy is still kept by B («Хранят 1 из 1»)" $kept
    Use-Montana $script:A; Stop-Montana; Start-Sleep -Seconds 2; $script:A = Open-Person "$Root\A" 0   # a fresh window stands on the list

    Note "== 7. a restore by A's words into a fresh folder (the new build, the real day: a call and its answer ride the box, whose minute the node's own clock names)"
    $script:A2 = Start-Montana -DataDir "$Root\A2"; Front-Montana -X 460 -Y 40 -Width 900 -Height 900
    Click-Named "Продолжить с Montana"; Click-Named "Открыть личность"
    $end = (Get-Date).AddSeconds(30)
    while ($true) { try { Type-First-Field ($wordsA -join " "); break } catch { if ((Get-Date) -gt $end) { throw }; Start-Sleep -Milliseconds 300 } }
    Click-Named "Восстановить"
    $end = (Get-Date).AddSeconds(40)
    do { $tx = Get-Texts; if ($tx -contains "Принимаю") { Click-Named "Принимаю"; break }; if ($tx -contains "Чаты") { break }; Start-Sleep -Milliseconds 500 } while ((Get-Date) -lt $end)
    Settle-Asks; To-List
    $back = $true; try { Find-Named $nB 300 | Out-Null } catch { $back = $false }
    $rows = @((Get-Texts) | Where-Object { $_ -eq $nB }).Count
    Check "A2: the conversation with $nB came back, once" ($back -and $rows -eq 1) "rows named so: $rows"
    if ($back) {
        Click-Named $nB
        $have = @($L[0..5] | Where-Object { $one = $_; try { Find-Named $one 5 | Out-Null; $true } catch { $false } })
        Check "A2: the letters came back" ($have.Count -eq 6) "$($have.Count) of 6"
        # the copy is still being taken back from the keeper (keeping.rs take_back: «keep took slot», then «keep build»): the
        # conversation is read again once the copy is laid -- it must still stand once, with its letters -- and only then sent from
        $end = (Get-Date).AddSeconds(300); $built = ""
        while ((Get-Date) -lt $end -and -not $built) { $built = Trace-Has "$Root\A2" "keep build generation="; if (-not $built) { Start-Sleep -Seconds 3 } }
        Check "A2: the copy kept by B is built back" ([bool]($built -match "laid=true")) $built
        Check "A2: the copy built back is the one the OLD build laid (generation 1)" ([bool]($built -match "generation=1 ")) $built
        $l3 = My-Link
        # the permanent link is 32 random bytes born once per device in its own vault (card.rs; iOS MontanaLocalVault, this
        # device only): a device restored by the words shows a link of its own, never A's (run15 27.10.2026)
        Check "A2: the restored device shows a permanent link of its own" ([bool]$l3 -and $l3 -ne $link)
        Holds "A2 after its copy is laid" $nB $L[0..5]
        Send $L[10]; Use-Montana $script:B; Front-Montana; Arrives "B (from the restored A)" $nA $L[10]
        Send $L[12]; Use-Montana $script:A2; Front-Montana; Arrives "A2 (to the restored A)" $nB $L[12]
    }
    Use-Montana $script:A2; Stop-Montana; Start-Sleep -Seconds 2   # A2 rests: the days are moved for A and B alone

    Note "== 7b. the days moved: 03.11 (+8) and 07.11 (+12), both sides on one day, letters direct"
    $ab = @("$Root\A", "$Root\B")
    Stop-All; Shift 8; $w0 = Refs-Now $ab
    $script:A = Open-Person "$Root\A" 0; $script:B = Open-Person "$Root\B" 920
    Wait-Paths $ab $w0; To-List; Click-Named $nA; Send $L[6]; Use-Montana $script:A; Front-Montana; Arrives "A (+8)" $nB $L[6]; Send $L[7]
    Use-Montana $script:B; Front-Montana; Arrives "B (+8)" $nA $L[7]
    Note ("  A's trace at +8: " + (Trace-Has "$Root\A" "floor_"))
    Stop-All; Shift 12; $w0 = Refs-Now $ab
    $script:A = Open-Person "$Root\A" 0; $script:B = Open-Person "$Root\B" 920
    Wait-Paths $ab $w0; To-List; Click-Named $nA; Send $L[8]; Use-Montana $script:A; Front-Montana; Arrives "A (+12)" $nB $L[8]; Send $L[9]
    Use-Montana $script:B; Front-Montana; Arrives "B (+12)" $nA $L[9]
    $fl = Trace-Has "$Root\A" "pipes_floored"
    Check "A's book floored at +12 (pipes_floored)" ([bool]$fl) $fl
    Note ("  A's last floor line: " + (Trace-Has "$Root\A" "floor_"))
    Use-Montana $script:A; Front-Montana; Holds "A (+12)" $nB $L[0..9]
    $l4 = My-Link; Check "A (+12): its identity's link is unchanged after the floor" ($l4 -eq $link)
    Use-Montana $script:B; Front-Montana; Holds "B (+12)" $nA $L[0..9]

    Note "== 8. the OLD build opened on a copy of A's migrated folder, then the new build on that copy"
    Stop-All
    Copy-Item -Recurse "$Root\A" "$Root\A-oldcopy"
    Use-Build $OldDir
    $script:C = Open-Person "$Root\A-oldcopy" 0; Start-Sleep -Seconds 20
    $seen = @((Get-Texts) | Where-Object { $_ -eq $nB }).Count
    Note "  the old build on the migrated copy lists «$nB» $seen time(s)"
    Stop-All
    Use-Build $NewDir
    $cb = @("$Root\A-oldcopy", "$Root\B"); $w0 = Refs-Now $cb
    $script:C = Open-Person "$Root\A-oldcopy" 0
    Holds "the migrated copy after the old build, on the new build" $nB $L[0..9]
    $script:B = Open-Person "$Root\B" 920
    Wait-Paths $cb $w0
    Use-Montana $script:C; Front-Montana; Send $L[11]
    Use-Montana $script:B; Front-Montana; Arrives "B (from the copy the old build opened)" $nA $L[11]

    Note "== 8b. the OLD build WRITES on a second copy of the migrated folder (a letter tried), then the new build reads it"
    Stop-All
    Copy-Item -Recurse "$Root\A" "$Root\A-oldcopy2"
    Use-Build $OldDir
    $script:C = Open-Person "$Root\A-oldcopy2" 0
    $sentOld = $true; try { To-List; Click-Named $nB; Send "old-build letter $tag" } catch { $sentOld = $false }
    Note "  the old build on the migrated copy: a letter $(if ($sentOld) { 'left by its ladder' } else { 'could not be sent' })"
    Start-Sleep -Seconds 15; Stop-All
    Use-Build $NewDir
    $cb = @("$Root\A-oldcopy2", "$Root\B"); $w0 = Refs-Now $cb
    $script:C = Open-Person "$Root\A-oldcopy2" 0
    Holds "the copy the old build wrote on, on the new build" $nB $L[0..9]
    $script:B = Open-Person "$Root\B" 920
    Wait-Paths $cb $w0
    Use-Montana $script:C; Front-Montana; Send ("u14 $tag")
    Use-Montana $script:B; Front-Montana; Arrives "B (from the copy the old build wrote on)" $nA ("u14 $tag")
} catch {
    Check "the scene ran to its end" $false $_.Exception.Message
} finally {
    $wordsA = $null
    Stop-All
    $pass = @($script:checks | Where-Object { $_.ok }).Count
    Note ("RESULT: {0} of {1} checks pass, {2} s" -f $pass, $script:checks.Count, [int]((Get-Date) - $t0).TotalSeconds)
    $script:checks | ConvertTo-Json | Set-Content -Path (Join-Path $Root "checks.json") -Encoding UTF8
}

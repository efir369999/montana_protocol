# ONE PERSON BY ONE SET OF WORDS ON WINDOWS AND ON THE OFFICIAL ANDROID (the owner's word 27.10.2026): a test
# identity is born on Windows (a check's folder, MONTANA_DATA_DIR), its call sign read from the name field before a name is given
# (the sign the core derives from the words: profile.rs callsign_of over seed.rs twin_of); the same 24 test words, read into this
# check's memory only (never a file, a log or a shot), are typed into the official APK's «Recovery from phrase» on the test
# emulator MontanaR; what the official app then names the person is read and compared. Test identities and the test emulator only.
# usage: tools\offident.ps1 -Root DIR -Exe PATH   (MontanaR up on emulator-5562 with the official APK on its recovery screen)
param([Parameter(Mandatory = $true)][string]$Root, [Parameter(Mandatory = $true)][string]$Exe, [string]$Device = "emulator-5562")
$ErrorActionPreference = "Stop"
. "$PSScriptRoot\ui.ps1"
. "$PSScriptRoot\android.ps1" -Device $Device
$script:exe = $PSBoundParameters.Exe   # -Exe and ui.ps1's $script:exe are one variable (names know no case): ui.ps1 above overwrote it
$env:MONTANA_TRACE = "1"
New-Item -ItemType Directory -Force $Root | Out-Null
$Log = Join-Path $Root "offident.log"
function Note([string]$s) { $l = "{0:HH:mm:ss} {1}" -f (Get-Date), $s; $l; Add-Content -Path $Log -Value $l -Encoding UTF8 }
# what a screen says, with anything that could be one of the 24 words starred
function Safe([string]$s) {
    if ($s -cmatch '([a-z]{3,8}\s+){2,}[a-z]{2,8}') { return "<field: " + @($s -csplit '\s+' | Where-Object { $_ }).Count + " words>" }
    return ($s -creplace '\b[a-z]{3,8}\b', '*')
}
function Safe-Texts { (Droid-Texts | ForEach-Object { Safe $_ }) -join " | " }
# the phrase field emptied (a field restored by the system may still hold an earlier attempt), then the words typed four at a time
# (one long «input text» is cut short)
function Droid-Phrase([string[]]$w) {
    $field = @(Droid-Nodes | Where-Object { $_.Class -like "*EditText*" }) | Select-Object -First 1   # the field itself, not its label
    if (-not $field) { throw "android: no phrase field" }
    Droid-TapAt $field.X $field.Y
    Droid-Adb shell input keyevent KEYCODE_MOVE_END | Out-Null
    Droid-Adb shell ("input keyevent " + ((1..260 | ForEach-Object { "67" }) -join " ")) | Out-Null
    for ($i = 0; $i -lt 24; $i += 4) { Droid-Type ((($w[$i..($i + 3)]) -join " ") + $(if ($i -lt 20) { " " } else { "" })) }
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
$tag = Get-Random -Maximum 9999
$words = $null
try {
    Note "== Windows: a test identity born; its call sign read before a name is given"
    $p = Start-Montana -DataDir "$Root\QW"; Front-Montana -X 0 -Y 40 -Width 900 -Height 900; Start-Sleep -Seconds 3
    Click-Named "Продолжить с Montana"; Click-Named "Создать"; Click-Named "Готово"
    Click-Named "Я понимаю и беру ответственность на себя"; Click-Named "Показать слова"
    Find-Named "Ваша сид-фраза из 24 слов" | Out-Null; Start-Sleep -Milliseconds 300
    $words = Grid-Words
    Click-Named "Я записал слова в надёжном месте"; Click-Named "Ваша личность в Montana"
    Find-Named "Готово" | Out-Null; Start-Sleep -Milliseconds 800
    $c = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::Edit)
    $f = (Root).FindFirst([System.Windows.Automation.TreeScope]::Descendants, $c)
    $winSign = if ($f) { $f.Current.Name } else { "" }
    if (-not $winSign -and $f) { $winSign = $f.Current.HelpText }
    Note "  Windows' call sign for the words: «$winSign»"
    Stop-Montana   # no name given: the Windows side is left unnamed, its folder kept as evidence

    # the same row of the call-sign table in the emulator's English (app/src-tauri/assets/callsigns.tsv: id, en, ru, …, emoji)
    $ru = ($winSign -replace '^\S+\s+', '').Trim()
    $row = Get-Content (Join-Path $PSScriptRoot "..\src-tauri\assets\callsigns.tsv") -Encoding UTF8 | Where-Object { $_ -and -not $_.StartsWith("#") } |
        ForEach-Object { , ($_ -split "`t") } | Where-Object { $_.Count -ge 5 -and $_[2] -eq $ru } | Select-Object -First 1
    $want = if ($row) { "$($row[5]) $($row[1])".Trim() } else { "" }
    Note "  the same call sign in English (the table's row): «$want»"

    Note "== the official Android (561 on MontanaR): «Open» an identity by the same words"
    Droid-Adb shell pm clear $script:Package | Out-Null
    Droid-Start; Start-Sleep -Seconds 12
    Droid-Tap "Continue with Montana" 30; Start-Sleep -Seconds 6
    Droid-Tap "Open" 30; Start-Sleep -Seconds 6
    Note ("  android shows: " + (Safe-Texts))
    Droid-Phrase $words
    $words = $null
    Start-Sleep -Seconds 1
    $typed = @(Droid-Texts | Where-Object { $_ -cmatch '^\s*([a-z]{2,8}\s+){23}[a-z]{2,8}\s*$' }).Count
    Note "  the phrase field holds 24 words: $(if ($typed) { 'yes' } else { 'NO' })"
    Droid-Tap "Restore" 20 -Exact
    # the terms the official app asks to accept after a restore: scrolled to their end, «I agree» pressed (its node may carry no
    # bounds; the button stands at the foot of the screen then)
    $end = (Get-Date).AddSeconds(60)
    while ((Get-Date) -lt $end) {
        Start-Sleep -Seconds 4
        if (@(Droid-Texts | Where-Object { $_ -eq "Terms of Use" }).Count) {
            for ($i = 0; $i -lt 6; $i++) { Droid-Adb shell input swipe 540 1800 540 500 300 | Out-Null; Start-Sleep -Milliseconds 500 }
            $ag = @(Droid-Nodes | Where-Object { $_.Text -eq "I agree" -and $_.Y -gt 0 }) | Select-Object -First 1
            if ($ag) { Droid-TapAt $ag.X $ag.Y } else { Droid-TapAt 540 2103 }
            Note "  the terms accepted"; break
        }
    }
    $seen = $false; $end = (Get-Date).AddSeconds(90); $last = ""
    while (-not $seen -and (Get-Date) -lt $end) {
        Start-Sleep -Seconds 6
        $tx = @(Droid-Texts)
        $seen = [bool](@($tx | Where-Object { $_ -and $want -and ($_ -eq $want -or $_ -like "*$want*") }).Count)
        $now = (($tx | ForEach-Object { Safe $_ }) -join " | ")
        if ($now -ne $last) { Note ("  android: " + $now); $last = $now }
    }
    Note ("  the official Android names the person by the same call sign: " + $(if ($seen) { "YES («$want»)" } else { "not seen in 90 s" }))
} catch {
    Note ("FAILED: " + $_.Exception.Message)
} finally {
    $words = $null
    try { Stop-Montana } catch {}
}

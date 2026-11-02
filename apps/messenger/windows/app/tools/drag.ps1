# THE TRAY'S PLATES CARRIED BY THE MOUSE, LIVE (after tools\ab.ps1 with the same -Root; iOS 2315 MTStagedOrder.moved, the owner's
# word 28.10.2026 05:5x): B stages two documents and a picture; the real mouse holds the third plate and lets it down on the first,
# then the picture on the head; the numbers follow the order; the cross takes the middle out after the carry; the arrow sends the
# tray once, in the order it stands, the caption with the first; A gets the picture above the document. The mouse is the person's
# own pointer for a few seconds: it is put back where it stood.
# usage: tools\drag.ps1 -Root DIR [-Shots DIR]
param([Parameter(Mandatory = $true)][string]$Root, [string]$Shots = "")
$ErrorActionPreference = "Stop"
. "$PSScriptRoot\ui.ps1"
Add-Type -AssemblyName System.Drawing, System.Windows.Forms
function Snap($name) { if ($Shots) { Shot-Montana (Join-Path $Shots "$name.png") } }
function Says([string]$text, [int]$timeout = 60) { Find-Named $text $timeout | Out-Null; "  sees: $text" }
function Wrote([string]$who, [string]$line, [int]$timeout = 20) {
    $end = (Get-Date).AddSeconds($timeout)
    do {
        $hit = Get-Content "$Root\$who\trace.log" -Encoding UTF8 -ErrorAction SilentlyContinue | Where-Object { $_ -like "*$line*" } | Select-Object -Last 1
        if ($hit) { "  ${who}: $($hit -replace '^\d+ ', '')"; return }
        Start-Sleep -Milliseconds 500
    } while ((Get-Date) -lt $end)
    throw "$who never wrote «$line»"
}
function Picture([string]$path, $color, [string]$words) {
    $bmp = New-Object System.Drawing.Bitmap 1200, 800
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.Clear($color)
    $g.DrawString($words, (New-Object System.Drawing.Font "Segoe UI", 72), [System.Drawing.Brushes]::White, 60, 300)
    $bmp.Save($path, [System.Drawing.Imaging.ImageFormat]::Png)
}
# the plates the tray holds now, in their order
function Plates {
    $tray = Exact-Named "Вложения" | Select-Object -First 1
    if (-not $tray) { return @() }
    $c = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::ListItem)
    @($tray.FindAll([System.Windows.Automation.TreeScope]::Descendants, $c))
}
function Plates-Are([string[]]$Want, [int]$Timeout = 15) {
    $end = (Get-Date).AddSeconds($Timeout)
    do { $now = @(Plates | ForEach-Object { $_.Current.Name }); if (($now -join " | ") -eq ($Want -join " | ")) { "  the tray: $($now -join ' | ')"; return }; Start-Sleep -Milliseconds 300 } while ((Get-Date) -lt $end)
    throw "the tray holds «$($now -join ' | ')», not «$($Want -join ' | ')»"
}
# the real pointer: pressed on the plate at place $From, nudged past the drag threshold and held until the system's drag has begun
# (it starts a moment after the nudge, and a move made before it is not seen), carried in small steps over the plate at place $To,
# stirred there a little so the plate below feels the carry, let go
function Carry([int]$From, [int]$To) {
    $p = Plates
    $a = $p[$From - 1].Current.BoundingRectangle; $b = $p[$To - 1].Current.BoundingRectangle
    $x = [int]($a.Left + $a.Width / 2); $y = [int]($a.Top + $a.Height * 0.7)
    $x2 = [int]($b.Left + $b.Width / 2); $y2 = [int]($b.Top + $b.Height * 0.7)
    [MontanaMouse]::SetCursorPos($x, $y) | Out-Null; Start-Sleep -Milliseconds 150
    [MontanaMouse]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 150
    try {
        $dir = [Math]::Sign($x2 - $x)
        for ($i = 1; $i -le 4; $i++) { [MontanaMouse]::SetCursorPos($x + $dir * 4 * $i, $y) | Out-Null; [MontanaMouse]::mouse_event(0x0001, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 40 }
        Start-Sleep -Milliseconds 400
        $x = $x + $dir * 16
        for ($i = 1; $i -le 30; $i++) {
            [MontanaMouse]::SetCursorPos([int]($x + ($x2 - $x) * $i / 30), [int]($y + ($y2 - $y) * $i / 30)) | Out-Null
            [MontanaMouse]::mouse_event(0x0001, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 40
        }
        for ($i = 0; $i -lt 12; $i++) {
            [MontanaMouse]::SetCursorPos($x2 + 4 * (($i % 2) * 2 - 1), $y2) | Out-Null
            [MontanaMouse]::mouse_event(0x0001, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 60
        }
    } finally { [MontanaMouse]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero) }
    Start-Sleep -Milliseconds 600
}
$env:MONTANA_TRACE = "1"
$t0 = Get-Date
$was = [System.Windows.Forms.Cursor]::Position
$first = Join-Path $Root "Первый.txt"; Set-Content -Path $first -Value "первый документ" -Encoding UTF8
$third = Join-Path $Root "Третий.txt"; Set-Content -Path $third -Value "третий документ" -Encoding UTF8
$pic = Join-Path $Root "drag-pic.png"; Picture $pic ([System.Drawing.Color]::DarkRed) "Перенос"
Remove-Item "$Root\A\trace.log", "$Root\B\trace.log" -ErrorAction SilentlyContinue

$A = $null; $B = $null
try {
    $A = Start-Montana -DataDir "$Root\A"; Front-Montana -X 0 -Y 40 -Width 900 -Height 900
    $B = Start-Montana -DataDir "$Root\B"; Front-Montana -X 920 -Y 40 -Width 900 -Height 900
    "B stages a document, a picture and a document"
    Click-Named "Алиса Тест"
    Type-Into "Сообщение..." "Перенос мышью"
    Files-Key; Pick-File $first
    Files-Key; Pick-File $pic
    Files-Key; Pick-File $third
    Plates-Are @("1. Первый.txt", "2. Фото", "3. Третий.txt")
    Snap "150-B-drag-before"

    "the mouse carries the third plate onto the first: it takes the head, the numbers follow"
    Front-Montana
    Carry 3 1
    Plates-Are @("1. Третий.txt", "2. Первый.txt", "3. Фото")
    Wrote "B" "tray_move place=1 of=3" 10
    Snap "151-B-drag-third-first"

    "the mouse carries the picture from the end onto the head"
    Carry 3 1
    Plates-Are @("1. Фото", "2. Третий.txt", "3. Первый.txt")
    Snap "152-B-drag-picture-first"

    "the cross takes the middle plate out after the carry: the numbers close up"
    $mid = Exact-Named "2. Третий.txt" | Select-Object -First 1
    $cross = Exact-Named "Убрать" $mid | Select-Object -First 1
    if (-not $cross) { throw "the carried plate has no cross" }
    $cross.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern).Invoke()
    Plates-Are @("1. Фото", "2. Первый.txt")
    if ((Get-Texts) -contains "Отправка…") { throw "a carried plate left before the arrow" }
    Snap "153-B-drag-crossed"

    "the arrow sends the tray once, in the carried order, the caption with the picture"
    Send-Tray 2
    Says "Первый.txt" 30
    Sent-Settle 90
    Wrote "B" "tray_send n=2 files=1" 10 | Out-Null
    $sends = @(Get-Content "$Root\B\trace.log" -Encoding UTF8 | Where-Object { $_ -like "*tray_send *" }).Count
    if ($sends -ne 1) { throw "the tray was sent $sends times" }
    $moves = @(Get-Content "$Root\B\trace.log" -Encoding UTF8 | Where-Object { $_ -like "*tray_move *" }).Count
    if ($moves -ne 2) { throw "B traced $moves carries, not 2" }
    $log = Get-Content "$Root\B\trace.log" -Encoding UTF8 | Where-Object { $_ -like "*media send *" } | ForEach-Object { ($_ -split "media send ")[1] }
    if (($log -join ", ") -ne "img jpg, doc txt") { throw "B sent «$($log -join ', ')», not the picture then the document" }
    "  B sent once, in the carried order: $($log -join ', ')"
    if ((Get-Texts) -contains "Третий.txt") { throw "the crossed plate left" }
    Snap "154-B-drag-sent"

    "A gets the picture with the caption above the document, and nothing crossed"
    Use-Montana $A; Front-Montana
    Click-Named "Борис Тест"
    Says "Первый.txt" 90
    Says "Перенос мышью" 30
    Start-Sleep -Seconds 6
    $texts = Get-Texts
    if ($texts -contains "Не получено") { throw "a file was not received" }
    if ($texts -contains "Третий.txt") { throw "A got the crossed plate" }
    $cap = (Exact-Named "Перенос мышью" | Select-Object -Last 1).Current.BoundingRectangle
    $doc = (Find-Named "Первый.txt" 10).Current.BoundingRectangle   # the document's bubble names more than the file: found as «Says» finds it
    if ($cap.Top -ge $doc.Top) { throw "A shows the document above the picture's caption" }
    "  A: the picture with its caption, then the document"
    Snap "155-A-drag"
    "done in $([int]((Get-Date) - $t0).TotalSeconds) s"
} finally {
    [MontanaMouse]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
    [MontanaMouse]::SetCursorPos($was.X, $was.Y) | Out-Null
    try { Stop-Montana } catch {}
    foreach ($p in @($A, $B)) { if ($p) { Use-Montana $p; try { Stop-Montana } catch {} } }
}

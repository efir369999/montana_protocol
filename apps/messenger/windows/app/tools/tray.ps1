# THE TRAY ABOVE THE FIELD, LIVE (after tools\ab.ps1 with the same -Root; iOS pastedPreview, MTStagedOrder, sendPasted; COMPOSE-039,
# PICK-013): B types a caption and picks a picture, a document and a second picture in Windows' Open dialog -- nothing leaves, the
# three wait above the field numbered 1, 2, 3; the cross takes the first out and the numbers close up (1 the document, 2 the
# picture); the arrow sends the tray in the order it stands, the caption with the first; A gets both. The carry of a plate to
# another place (2315) wants a held mouse, which a check does not take from the person: its rule (MTStagedOrder.moved) is read.
# usage: tools\tray.ps1 -Root DIR [-Shots DIR]
param([Parameter(Mandatory = $true)][string]$Root, [string]$Shots = "")
$ErrorActionPreference = "Stop"
. "$PSScriptRoot\ui.ps1"
Add-Type -AssemblyName System.Drawing
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
# the plates the tray holds now, by their names («N. what»), in their order
function Plates {
    $tray = Exact-Named "Вложения" | Select-Object -First 1
    if (-not $tray) { return @() }
    $c = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::ListItem)
    @($tray.FindAll([System.Windows.Automation.TreeScope]::Descendants, $c) | ForEach-Object { $_.Current.Name })
}
function Plates-Are([string[]]$Want, [int]$Timeout = 15) {
    $end = (Get-Date).AddSeconds($Timeout)
    do { $now = Plates; if (($now -join " | ") -eq ($Want -join " | ")) { "  the tray: $($now -join ' | ')"; return }; Start-Sleep -Milliseconds 300 } while ((Get-Date) -lt $end)
    throw "the tray holds «$($now -join ' | ')», not «$($Want -join ' | ')»"
}
$env:MONTANA_TRACE = "1"
$t0 = Get-Date
$one = Join-Path $Root "tray-one.png"; Picture $one ([System.Drawing.Color]::DarkBlue) "Первая"
$two = Join-Path $Root "tray-two.png"; Picture $two ([System.Drawing.Color]::DarkGreen) "Вторая"
$doc = Join-Path $Root "Список дел.txt"; Set-Content -Path $doc -Value "Лоток Windows: порядок задаёт человек." -Encoding UTF8
Remove-Item "$Root\A\trace.log", "$Root\B\trace.log" -ErrorAction SilentlyContinue

$A = $null; $B = $null
try {
    $A = Start-Montana -DataDir "$Root\A"; Front-Montana -X 0 -Y 40 -Width 900 -Height 900
    $B = Start-Montana -DataDir "$Root\B"; Front-Montana -X 920 -Y 40 -Width 900 -Height 900
    "B stages a picture, a document and a picture: nothing leaves, the three are numbered"
    Click-Named "Алиса Тест"
    Type-Into "Сообщение..." "Лоток с Windows"
    Files-Key; Pick-File $one
    Plates-Are @("1. Фото")
    Files-Key; Pick-File $doc
    Plates-Are @("1. Фото", "2. Список дел.txt")
    Files-Key; Pick-File $two
    Plates-Are @("1. Фото", "2. Список дел.txt", "3. Фото")
    if ((Get-Texts) -contains "Отправка…") { throw "a staged file left before the arrow" }
    Snap "140-B-tray"

    "the cross takes the first out: the numbers close up"
    $first = Exact-Named "1. Фото" | Select-Object -First 1
    $cross = Exact-Named "Убрать" $first | Select-Object -First 1
    if (-not $cross) { throw "the first plate has no cross" }
    $cross.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern).Invoke()
    Plates-Are @("1. Список дел.txt", "2. Фото")
    if (-not (Exact-Named "Файлы" (Exact-Named "Вложения" | Select-Object -First 1) | Select-Object -First 1)) { throw "a tray with a file has no «+»" }
    "  the tray with a file wears its «+»"
    Snap "141-B-tray-crossed"

    "the arrow sends the tray in its order, the caption with the first"
    Send-Tray 2
    Says "Список дел.txt" 30
    Says "Лоток с Windows" 30
    Sent-Settle 90
    Wrote "B" "tray_send n=2 files=1" 10 | Out-Null
    $log = Get-Content "$Root\B\trace.log" -Encoding UTF8 | Where-Object { $_ -like "*media send *" } | ForEach-Object { ($_ -split "media send ")[1] }
    if (($log -join ", ") -ne "doc txt, img jpg") { throw "B sent «$($log -join ', ')», not the document then the picture" }
    "  B sent in the tray's order: $($log -join ', ')"
    Snap "142-B-tray-sent"

    "A gets the document with the caption, then the picture"
    Use-Montana $A; Front-Montana
    Click-Named "Борис Тест"
    Says "Список дел.txt" 90
    Says "Лоток с Windows" 30
    Start-Sleep -Seconds 6
    if ((Get-Texts) -contains "Не получено") { throw "a file was not received" }
    Snap "143-A-tray"

    "a second tray: two documents, the first taken off the disk before the arrow; the arrow pressed twice at once"
    $lost = Join-Path $Root "Пропадёт.txt"; Set-Content -Path $lost -Value "этого файла не станет" -Encoding UTF8
    $stay = Join-Path $Root "Останется.txt"; Set-Content -Path $stay -Value "этот уйдёт один раз" -Encoding UTF8
    Use-Montana $B; Front-Montana
    Files-Key; Pick-File $lost
    Files-Key; Pick-File $stay
    Plates-Are @("1. Пропадёт.txt", "2. Останется.txt")
    Remove-Item $lost
    $arrow = Exact-Named "Отправить" | Select-Object -First 1
    $press = $arrow.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)
    $press.Invoke(); try { $press.Invoke() } catch { "  (the second press found the arrow gone)" }
    Plates-Are @("1. Пропадёт.txt") 60
    "  the file that could not be read came back to the tray; the other left"
    Says "Не отправлено" 10
    Says "Останется.txt" 30
    Sent-Settle 90
    Wrote "B" "tray_back n=1 of=2" 10
    $twice = @(Get-Content "$Root\B\trace.log" -Encoding UTF8 | Where-Object { $_ -like "*tray_send n=2 files=2*" }).Count
    if ($twice -ne 1) { throw "the second tray was sent $twice times" }
    # «media send» is traced as a file is taken up, before it is read (Media.jsx sendFile): the file gone from the disk is tried
    # once too, then comes back; a second press would trace both again
    $log = Get-Content "$Root\B\trace.log" -Encoding UTF8 | Where-Object { $_ -like "*media send *" } | ForEach-Object { ($_ -split "media send ")[1] }
    if (($log -join ", ") -ne "doc txt, img jpg, doc txt, doc txt") { throw "B tried «$($log -join ', ')»: a file twice or one lost" }
    "  the double press sent once: $($log -join ', ')"
    Snap "144-B-tray-back"
    $cross = Exact-Named "Убрать" (Exact-Named "1. Пропадёт.txt" | Select-Object -First 1) | Select-Object -First 1
    $cross.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern).Invoke()
    Plates-Are @()

    Use-Montana $A; Front-Montana
    Says "Останется.txt" 90
    if ((Get-Texts) -contains "Пропадёт.txt") { throw "A got the file that was never read" }
    "  A got the one file once"
    Snap "145-A-tray-second"
    "done in $([int]((Get-Date) - $t0).TotalSeconds) s"
} finally {
    try { Stop-Montana } catch {}
    foreach ($p in @($A, $B)) { if ($p) { Use-Montana $p; try { Stop-Montana } catch {} } }
}

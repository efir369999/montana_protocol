# A FILE DROPPED FROM WINDOWS EXPLORER INTO THE TRAY, LIVE (after tools\ab.ps1 with the same -Root; the owner's word 28.10.2026 06:2x,
# after dragDropEnabled false): Explorer shows a test file; the real mouse carries it from Explorer's list onto B's conversation;
# it stands in the tray as «1. name» and nothing leaves; the arrow sends it once; A gets it. Only the Explorer window this check
# opened is closed; the pointer goes back where it stood.
# usage: tools\dropin.ps1 -Root DIR [-Shots DIR]
param([Parameter(Mandatory = $true)][string]$Root, [string]$Shots = "")
$ErrorActionPreference = "Stop"
. "$PSScriptRoot\ui.ps1"
Add-Type -AssemblyName System.Windows.Forms
function Snap($name) { if ($Shots) { Shot-Montana (Join-Path $Shots "$name.png") } }
function Says([string]$text, [int]$timeout = 60) { Find-Named $text $timeout | Out-Null; "  sees: $text" }
function Plates {
    $tray = Exact-Named "Вложения" | Select-Object -First 1
    if (-not $tray) { return @() }
    $c = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::ListItem)
    @($tray.FindAll([System.Windows.Automation.TreeScope]::Descendants, $c) | ForEach-Object { $_.Current.Name })
}
$env:MONTANA_TRACE = "1"
$t0 = Get-Date
$was = [System.Windows.Forms.Cursor]::Position
$dir = Join-Path (Resolve-Path $Root).Path "dropin"
New-Item -ItemType Directory -Force $dir | Out-Null
$file = Join-Path $dir "Из Проводника.txt"; Set-Content -Path $file -Value "брошен мышью из Проводника" -Encoding UTF8
Remove-Item "$Root\A\trace.log", "$Root\B\trace.log" -ErrorAction SilentlyContinue
$desk = [System.Windows.Automation.AutomationElement]::RootElement
$cab = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ClassNameProperty, "CabinetWClass")
$before = @($desk.FindAll([System.Windows.Automation.TreeScope]::Children, $cab) | ForEach-Object { $_.Current.NativeWindowHandle })

$A = $null; $B = $null; $ex = $null
try {
    $A = Start-Montana -DataDir "$Root\A"; Front-Montana -X 0 -Y 40 -Width 900 -Height 900
    $B = Start-Montana -DataDir "$Root\B"; Front-Montana -X 920 -Y 40 -Width 900 -Height 900
    Click-Named "Алиса Тест"
    "Explorer shows the test file"
    Start-Process explorer.exe "/select,`"$file`""
    $end = (Get-Date).AddSeconds(20)
    do { Start-Sleep -Milliseconds 500; $ex = @($desk.FindAll([System.Windows.Automation.TreeScope]::Children, $cab) | Where-Object { $before -notcontains $_.Current.NativeWindowHandle }) | Select-Object -First 1 } while (-not $ex -and (Get-Date) -lt $end)
    if (-not $ex) { throw "no new Explorer window" }
    $h = [IntPtr]$ex.Current.NativeWindowHandle
    [MontanaWin]::SetWindowPos($h, [IntPtr]::Zero, 40, 300, 820, 600, 0x0040) | Out-Null; [MontanaWin]::SetForegroundWindow($h) | Out-Null
    Start-Sleep -Seconds 2
    $item = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::NameProperty, "Из Проводника.txt")
    $it = $ex.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $item)
    if (-not $it) { $item = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::NameProperty, "Из Проводника"); $it = $ex.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $item) }
    if (-not $it) { throw "Explorer does not show the file" }
    $src = $it.Current.BoundingRectangle
    $r = New-Object MontanaWin+Rect; [MontanaWin]::GetWindowRect($script:proc.MainWindowHandle, [ref]$r) | Out-Null
    $x = [int]($src.Left + [Math]::Min(60, $src.Width / 2)); $y = [int]($src.Top + $src.Height / 2)
    $x2 = [int](($r.Left + $r.Right) / 2); $y2 = [int]($r.Top + ($r.Bottom - $r.Top) * 0.5)

    "the real mouse carries the file from Explorer onto B's conversation"
    [MontanaMouse]::SetCursorPos($x, $y) | Out-Null; Start-Sleep -Milliseconds 200
    [MontanaMouse]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 150
    try {
        for ($i = 1; $i -le 4; $i++) { [MontanaMouse]::SetCursorPos($x + 5 * $i, $y) | Out-Null; [MontanaMouse]::mouse_event(0x0001, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 40 }
        Start-Sleep -Milliseconds 400
        $x = $x + 20
        for ($i = 1; $i -le 40; $i++) { [MontanaMouse]::SetCursorPos([int]($x + ($x2 - $x) * $i / 40), [int]($y + ($y2 - $y) * $i / 40)) | Out-Null; [MontanaMouse]::mouse_event(0x0001, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 40 }
        for ($i = 0; $i -lt 12; $i++) { [MontanaMouse]::SetCursorPos($x2 + 4 * (($i % 2) * 2 - 1), $y2) | Out-Null; [MontanaMouse]::mouse_event(0x0001, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 60 }
    } finally { [MontanaMouse]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero) }
    $end = (Get-Date).AddSeconds(15)
    do { $now = Plates; if (($now -join " | ") -eq "1. Из Проводника.txt") { break }; Start-Sleep -Milliseconds 300 } while ((Get-Date) -lt $end)
    if (($now -join " | ") -ne "1. Из Проводника.txt") { throw "the tray holds «$($now -join ' | ')» after the drop" }
    "  the tray: $($now -join ' | ')"
    if ((Get-Texts) -contains "Отправка…") { throw "the dropped file left before the arrow" }
    Snap "160-B-dropin"

    "the arrow sends it once; A gets it"
    Front-Montana
    Send-Tray 1
    Says "Из Проводника.txt" 30
    Sent-Settle 90
    $sends = @(Get-Content "$Root\B\trace.log" -Encoding UTF8 | Where-Object { $_ -like "*tray_send *" }).Count
    if ($sends -ne 1) { throw "the tray was sent $sends times" }
    $log = Get-Content "$Root\B\trace.log" -Encoding UTF8 | Where-Object { $_ -like "*media send *" } | ForEach-Object { ($_ -split "media send ")[1] }
    if (($log -join ", ") -ne "doc txt") { throw "B sent «$($log -join ', ')»" }
    "  B sent once: $($log -join ', ')"
    Use-Montana $A; Front-Montana
    Click-Named "Борис Тест"
    Says "Из Проводника.txt" 90
    Start-Sleep -Seconds 4
    if ((Get-Texts) -contains "Не получено") { throw "the file was not received" }
    Snap "161-A-dropin"
    "done in $([int]((Get-Date) - $t0).TotalSeconds) s"
} finally {
    [MontanaMouse]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
    [MontanaMouse]::SetCursorPos($was.X, $was.Y) | Out-Null
    if ($ex) { try { $ex.GetCurrentPattern([System.Windows.Automation.WindowPattern]::Pattern).Close() } catch {} }
    try { Stop-Montana } catch {}
    foreach ($p in @($A, $B)) { if ($p) { Use-Montana $p; try { Stop-Montana } catch {} } }
}

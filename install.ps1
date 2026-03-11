$ErrorActionPreference = 'Stop'

$Repo = 'fluo10/tryluck'
$InstallDir = Join-Path $HOME '.local\bin'

$Response = Invoke-WebRequest -Uri "https://github.com/$Repo/releases/latest" -MaximumRedirection 0 -ErrorAction Ignore
$Version = $Response.Headers.Location -replace '.*/tag/', ''
if (-not $Version) {
    Write-Error 'Failed to fetch latest version.'
    exit 1
}

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

$Asset = 'tryluck-windows-x86_64.exe'
$Url = "https://github.com/$Repo/releases/download/$Version/$Asset"
$Dest = Join-Path $InstallDir 'tryluck.exe'

Write-Host "Installing tryluck $Version to $InstallDir..."
Invoke-WebRequest -Uri $Url -OutFile $Dest
Write-Host "Done! $Dest installed."

$UserPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable('PATH', "$InstallDir;$UserPath", 'User')
    Write-Host ""
    Write-Host "Added $InstallDir to your PATH. Restart your terminal to apply."
}

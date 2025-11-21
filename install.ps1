# Windows-only installer for git-find

# config
$CLI_NAME   = "git-find"
$REPO       = "edenian-prince/rust-secrets"
$InstallDir = "$HOME\.local\bin"

# color messages
function Info  { Write-Host "[INFO] "  -ForegroundColor Blue    -NoNewline; Write-Host $args }
function Ok    { Write-Host "[OK] "    -ForegroundColor Green   -NoNewline; Write-Host $args }
function Warn  { Write-Host "[WARN] "  -ForegroundColor Yellow  -NoNewline; Write-Host $args }
function Error { Write-Host "[ERROR] " -ForegroundColor Red     -NoNewline; Write-Host $args; exit 1 }

$Asset = "git-find-windows-x86_64.exe"

Info "Fetching latest release asset..."

$release = Invoke-RestMethod -Uri "https://api.github.com/repos/$REPO/releases/latest" `
           -Headers @{ "User-Agent" = "pwsh" }

$assetUrl = $release.assets |
    Where-Object { $_.browser_download_url -match $Asset } |
    Select-Object -ExpandProperty browser_download_url -ErrorAction Ignore

if (-not $assetUrl) {
    Error "Could not find asset matching: $Asset"
}

# download it
Info "Downloading $Asset ..."
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

$targetPath = Join-Path $InstallDir "$CLI_NAME.exe"

Invoke-WebRequest -Uri $assetUrl -OutFile $targetPath -UseBasicParsing -ErrorAction Stop

Ok "Installed $CLI_NAME to $InstallDir"

# path check
$pathParts = $Env:PATH.Split([IO.Path]::PathSeparator)
if (-not ($pathParts -contains $InstallDir)) {
    Warn "$InstallDir not in PATH. Adding it now..."

    $current = [Environment]::GetEnvironmentVariable("PATH", "User")
    [Environment]::SetEnvironmentVariable("PATH", "$InstallDir;$current", "User")

    Ok "Added to PATH. Restart PowerShell to use '$CLI_NAME'."
}

Ok "Installation complete!"


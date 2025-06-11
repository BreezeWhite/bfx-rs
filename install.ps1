# PowerShell install script for Windows

# Set variables
$REPO = "BreezeWhite/bfx-rs" # <-- Replace with actual repo, e.g. kohara/lendbot
$VERSION = "latest" # or specify a version/tag
$BINARY_WINDOWS = "bfx-windows.exe"

# Get latest release tag if VERSION is 'latest'
if ($VERSION -eq "latest") {
    $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$REPO/releases/latest"
    $VERSION = $response.tag_name
}

# Download URL
$URL = "https://github.com/$REPO/releases/download/$VERSION/$BINARY_WINDOWS"

# Find a writable directory in PATH
$installDir = $null
$env:PATH.Split(';') | ForEach-Object {
    if ($_ -and (Test-Path $_) -and (Get-Item $_).Attributes -notmatch 'ReadOnly') {
        try {
            $testFile = Join-Path $_ "bfx_test.tmp"
            New-Item -Path $testFile -ItemType File -Force | Out-Null
            Remove-Item $testFile -Force
            $installDir = $_
            break
        } catch {}
    }
}

if (-not $installDir) {
    Write-Host "No writable directory found in PATH. Run as Administrator or add a writable directory to your PATH."
    exit 1
}

# Download and install
$tmpFile = [System.IO.Path]::GetTempFileName()
Write-Host "Downloading $BINARY_WINDOWS from $URL..."
Invoke-WebRequest -Uri $URL -OutFile $tmpFile
Move-Item $tmpFile (Join-Path $installDir "bfx.exe") -Force
Write-Host "Installed 'bfx.exe' to $installDir"
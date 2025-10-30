# Ratifact Installation Script for Windows
# Complete one-liner setup: installs all dependencies and runs the app
# This script is designed to be piped directly: powershell -Command "iwr -useb https://...install.ps1 | iex"

Write-Host "Installing Ratifact dependencies..." -ForegroundColor Green

$logPath = "$env:TEMP\ratifact-install-$(Get-Date -Format 'yyyyMMdd_HHmmss').log"
Start-Transcript -Path $logPath

Write-Host "Installation log: $logPath" -ForegroundColor Cyan
Write-Host "Review this log after installation to verify all actions" -ForegroundColor Yellow

# Function to install or upgrade packages via winget
function Install-OrUpgradePackage {
    param (
        [string]$PackageId,
        [string]$FriendlyName
    )

    Write-Host "`nChecking $FriendlyName..." -ForegroundColor Cyan

    $installed = winget list --id $PackageId --exact 2>$null

    if ($LASTEXITCODE -eq 0 -and $installed -match $PackageId) {
        Write-Host "$FriendlyName is already installed." -ForegroundColor Green
    } else {
        Write-Host "Installing $FriendlyName..." -ForegroundColor Green
        winget install --id=$PackageId -e --accept-source-agreements --accept-package-agreements 2>$null
        Write-Host "$FriendlyName installed successfully" -ForegroundColor Green
    }
}

# Check current execution policy
$currentPolicy = Get-ExecutionPolicy -Scope CurrentUser
Write-Host "`nCurrent execution policy: $currentPolicy" -ForegroundColor Cyan

if ($currentPolicy -eq "Restricted" -or $currentPolicy -eq "AllSigned") {
    Write-Host "Setting execution policy to RemoteSigned..." -ForegroundColor Green
    Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser -Force
    Write-Host "Execution policy changed to RemoteSigned" -ForegroundColor Green
}

# Install required packages
Write-Host "`nInstalling required packages..." -ForegroundColor Cyan
Install-OrUpgradePackage -PackageId "Rustlang.Rustup" -FriendlyName "Rust"
Install-OrUpgradePackage -PackageId "Docker.DockerDesktop" -FriendlyName "Docker Desktop"
Install-OrUpgradePackage -PackageId "Git.Git" -FriendlyName "Git"

# Refresh PATH after installations
Write-Host "`nRefreshing PATH environment..." -ForegroundColor Green
$env:PATH = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
$env:PATH += ";$env:USERPROFILE\.cargo\bin"

# Wait for Docker to start
Write-Host "`nWaiting for Docker to be ready..." -ForegroundColor Cyan
$maxRetries = 60
$retryCount = 0
$dockerReady = $false

while ($retryCount -lt $maxRetries) {
    try {
        & docker ps 2>$null | Out-Null
        $dockerReady = $true
        break
    } catch {
        $retryCount++
        if ($retryCount -lt $maxRetries) {
            Write-Host "Waiting for Docker daemon... ($retryCount/60)" -ForegroundColor Yellow
            Start-Sleep -Seconds 1
        }
    }
}

if (-not $dockerReady) {
    Write-Host "Docker Desktop is not running after installation." -ForegroundColor Red
    Write-Host "Please start Docker Desktop manually from the Start Menu and run this script again." -ForegroundColor Yellow
    exit 1
}

Write-Host "Docker is running" -ForegroundColor Green

# Clone repository if not already in it
Write-Host "`nSetting up Ratifact..." -ForegroundColor Cyan
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "Cloning Ratifact repository..." -ForegroundColor Green
    git clone https://github.com/adolfousier/ratifact.git ratifact
    Set-Location ratifact
    Write-Host "Repository cloned" -ForegroundColor Green
} else {
    Write-Host "Already in Ratifact directory" -ForegroundColor Green
}

# Generate .env file
Write-Host "Generating .env file with random credentials..." -ForegroundColor Green
$rand = New-Object System.Random
$bytes = New-Object byte[] 16
$rand.NextBytes($bytes)

$PASSWORD = [Convert]::ToBase64String($bytes) -replace '[+/=]', '' | Select-Object -ExpandProperty Substring 0 16

$envContent = @"
DATABASE_URL=postgres://ratifact:${PASSWORD}@localhost:25851/ratifact
POSTGRES_USERNAME=ratifact
POSTGRES_PASSWORD=${PASSWORD}
DEBUG_LOGS_ENABLED=true
"@

[System.IO.File]::WriteAllText("$((Get-Location).Path)\.env", $envContent)
Write-Host ".env file generated" -ForegroundColor Green

# Start PostgreSQL container
Write-Host "`nStarting PostgreSQL container..." -ForegroundColor Green
try {
    & docker compose up -d 2>$null
    Write-Host "PostgreSQL container started" -ForegroundColor Green
    Write-Host "Waiting for PostgreSQL to be ready (30 seconds)..." -ForegroundColor Cyan
    Start-Sleep -Seconds 30
} catch {
    Write-Host "Failed to start PostgreSQL: $_" -ForegroundColor Red
    exit 1
}

# Build the project
Write-Host "`nBuilding Ratifact..." -ForegroundColor Green
try {
    & cargo build 2>&1 | Select-Object -Last 1
    Write-Host "Build completed successfully" -ForegroundColor Green
} catch {
    Write-Host "Build failed: $_" -ForegroundColor Red
    exit 1
}

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "Installation Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "`nTo run Ratifact:" -ForegroundColor Cyan
Write-Host "  cargo run" -ForegroundColor White
Write-Host "`nTo build a release version:" -ForegroundColor Cyan
Write-Host "  cargo build --release" -ForegroundColor White
Write-Host "`nTo run tests:" -ForegroundColor Cyan
Write-Host "  cargo test" -ForegroundColor White
Write-Host "`nInstallation log saved to: $logPath" -ForegroundColor Yellow

Stop-Transcript

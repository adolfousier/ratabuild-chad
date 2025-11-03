# Ratifact Uninstallation Script for Windows
# Removes Ratifact application, Docker container, and PostgreSQL volume
# Requires: PowerShell as Administrator

# Set error action preference
$ErrorActionPreference = "Stop"

# Color codes and formatting
$Colors = @{
    Red    = [System.ConsoleColor]::Red
    Green  = [System.ConsoleColor]::Green
    Yellow = [System.ConsoleColor]::Yellow
    Cyan   = [System.ConsoleColor]::Cyan
    White  = [System.ConsoleColor]::White
}

# Logging
$LogFile = "$env:TEMP\ratifact-uninstall-$(Get-Date -Format 'yyyyMMdd_HHmmss').log"
"Uninstall log: $LogFile" | Tee-Object -FilePath $LogFile | Write-Host

# Helper functions
function Print-Header {
    Write-Host ""
    Write-Host "╔═══════════════════════════════════════════════════════════╗" -ForegroundColor $Colors.Cyan
    Write-Host "║      Ratifact Uninstallation Script - Windows            ║" -ForegroundColor $Colors.Cyan
    Write-Host "╚═══════════════════════════════════════════════════════════╝" -ForegroundColor $Colors.Cyan
    Write-Host ""
}

function Print-Success {
    param([string]$Message)
    Write-Host "✓ $Message" -ForegroundColor $Colors.Green
    Add-Content -Path $LogFile -Value "✓ $Message"
}

function Print-Error {
    param([string]$Message)
    Write-Host "❌ ERROR: $Message" -ForegroundColor $Colors.Red
    Add-Content -Path $LogFile -Value "❌ ERROR: $Message"
}

function Print-Warning {
    param([string]$Message)
    Write-Host "⚠️  WARNING: $Message" -ForegroundColor $Colors.Yellow
    Add-Content -Path $LogFile -Value "⚠️  WARNING: $Message"
}

function Print-Info {
    param([string]$Message)
    Write-Host "ℹ️  $Message" -ForegroundColor $Colors.Cyan
    Add-Content -Path $LogFile -Value "ℹ️  $Message"
}

function Print-Section {
    param([string]$Message)
    Write-Host ""
    Write-Host "▶ $Message" -ForegroundColor $Colors.Cyan
    Add-Content -Path $LogFile -Value "▶ $Message"
}

function Confirm-Action {
    param([string]$Prompt)
    $response = Read-Host "$Prompt (yes/no)"
    return $response -eq "yes"
}

function Check-Administrator {
    $isAdmin = [bool]([System.Security.Principal.WindowsIdentity]::GetCurrent().Groups -match 'S-1-5-32-544')
    if (-not $isAdmin) {
        Print-Error "This script requires Administrator privileges. Please run as Administrator."
        exit 1
    }
}

function Check-Docker {
    try {
        $null = docker --version 2>&1
        return $true
    } catch {
        return $false
    }
}

# Main uninstall process
function Main {
    Print-Header

    # Check for Administrator privileges
    Check-Administrator

    # Determine installation directory
    $InstallDir = if ($env:RATIFACT_INSTALL_DIR) { $env:RATIFACT_INSTALL_DIR } else { "." }

    # Try to find installation directory if not valid
    if (-not (Test-Path "$InstallDir\Cargo.toml")) {
        if (Test-Path "$env:USERPROFILE\ratifact\Cargo.toml") {
            $InstallDir = "$env:USERPROFILE\ratifact"
        } elseif (Test-Path "C:\ratifact\Cargo.toml") {
            $InstallDir = "C:\ratifact"
        } else {
            Print-Warning "Could not locate Ratifact installation directory"
            $InstallDir = Read-Host "Enter the path to your Ratifact installation"
            if (-not (Test-Path "$InstallDir\Cargo.toml")) {
                Print-Error "Invalid installation directory: $InstallDir"
                exit 1
            }
        }
    }

    Print-Info "Found Ratifact installation at: $InstallDir"
    Write-Host ""

    # Confirmation
    Write-Host "This will:" -ForegroundColor $Colors.Yellow
    Write-Host "  1. Stop the PostgreSQL Docker container"
    Write-Host "  2. Remove the PostgreSQL Docker volume (optional)"
    Write-Host "  3. Clean build artifacts"
    Write-Host "  4. Remove the installation directory (optional)"
    Write-Host ""

    if (-not (Confirm-Action "Do you want to continue with uninstallation?")) {
        Print-Info "Uninstallation cancelled"
        exit 0
    }

    # Check and stop Docker
    if (Check-Docker) {
        Print-Section "Stopping Docker containers..."
        $composePath = Join-Path $InstallDir "compose.yml"
        if (Test-Path $composePath) {
            try {
                Push-Location $InstallDir
                docker compose down 2>&1 | Tee-Object -FilePath $LogFile -Append | Out-Null
                Print-Success "Docker containers stopped"
            } catch {
                Print-Warning "Could not stop Docker containers (they may not be running)"
            } finally {
                Pop-Location
            }
        } else {
            Print-Warning "compose.yml not found"
        }

        # Ask about removing volume
        Write-Host ""
        Print-Info "PostgreSQL data volume: ratifact-postgres-data"
        if (Confirm-Action "Do you want to remove the PostgreSQL volume (this will delete all database data)?") {
            Print-Section "Removing PostgreSQL volume..."
            try {
                docker volume rm ratifact-postgres-data 2>&1 | Tee-Object -FilePath $LogFile -Append | Out-Null
                Print-Success "PostgreSQL volume removed"
            } catch {
                Print-Warning "Could not remove volume (it may not exist)"
            }
        } else {
            Print-Info "Keeping PostgreSQL volume"
        }
    } else {
        Print-Warning "Docker is not available or not in PATH. Skipping container and volume removal."
    }

    # Clean build artifacts
    Print-Section "Cleaning build artifacts..."
    $targetPath = Join-Path $InstallDir "target"
    if (Test-Path $targetPath) {
        try {
            Push-Location $InstallDir
            cargo clean 2>&1 | Tee-Object -FilePath $LogFile -Append | Out-Null
            Print-Success "Build artifacts cleaned"
        } catch {
            Print-Warning "Could not clean build artifacts (Rust may not be in PATH)"
        } finally {
            Pop-Location
        }
    } else {
        Print-Info "No build artifacts found"
    }

    # Ask about removing installation directory
    Write-Host ""
    if (Confirm-Action "Do you want to remove the entire installation directory?") {
        Print-Section "Removing installation directory..."
        try {
            Remove-Item -Path $InstallDir -Recurse -Force -ErrorAction Stop
            Print-Success "Installation directory removed: $InstallDir"
        } catch {
            Print-Error "Could not remove installation directory (permission denied?)"
            Print-Info "You can manually remove it with: Remove-Item -Path '$InstallDir' -Recurse -Force"
            exit 1
        }
    } else {
        Print-Info "Keeping installation directory at: $InstallDir"
    }

    Write-Host ""
    Print-Section "Uninstallation complete!"
    Write-Host ""
    Print-Success "Ratifact has been successfully uninstalled"
    Write-Host ""
    Print-Info "To reinstall, visit: https://github.com/adolfousier/ratifact"
    Write-Host ""
}

# Run main
Main

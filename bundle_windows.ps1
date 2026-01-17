#!/usr/bin/env pwsh
# Windows Bundle Script for Juice: Zero Bugs Given
# Run this in PowerShell or convert to .bat

$ErrorActionPreference = "Stop"

Write-Host "=== Building for Windows ===" -ForegroundColor Cyan

# Check if cross-compilation tools are installed
$crossCompile = $false
if (Get-Command "x86_64-w64-mingw32-gcc" -ErrorAction SilentlyContinue) {
    Write-Host "MinGW toolchain found - using cross-compilation" -ForegroundColor Green
    $crossCompile = $true
    $target = "x86_64-pc-windows-gnu"
} else {
    Write-Host "Building for current platform (Windows)" -ForegroundColor Green
    $target = "x86_64-pc-windows-msvc"
}

# Build release
Write-Host "Building release binary..." -ForegroundColor Yellow
cargo build --release --target $target

# Create distribution directory
Write-Host "Creating distribution directory..." -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path "dist/Windows" | Out-Null

# Copy binary
$binaryName = if ($target -eq "x86_64-pc-windows-gnu") { "fighter_game.exe" } else { "fighter_game.exe" }
$binaryPath = "target/$target/release/$binaryName"
Copy-Item $binaryPath "dist/Windows/"

# Copy assets
Write-Host "Copying assets..." -ForegroundColor Yellow
Copy-Item "assets" -Recurse -Destination "dist/Windows/"

# Copy README and LICENSE
Copy-Item "README.md" "dist/Windows/"
Copy-Item "LICENSE" "dist/Windows/"

# Handle DLL dependencies (for MinGW)
if ($crossCompile) {
    Write-Host "Finding required DLLs..." -ForegroundColor Yellow

    # Find and copy essential DLLs
    $dlls = @(
        "libbevy_dylib-*.dll",
        "libbevy-*.dll",
        "libgcc_s_seh-1.dll",
        "libstdc++-6.dll",
        "libwinpthread-1.dll"
    )

    foreach ($dllPattern in $dlls) {
        $dllsFound = Get-Item "target/$target/deps/$dllPattern" -ErrorAction SilentlyContinue | Select-Object -First 1
        if ($dllsFound) {
            Write-Host "  Copying $($dllsFound.Name)" -ForegroundColor Gray
            Copy-Item $dllsFound.FullName "dist/Windows/"
        }
    }

    # Use ldd-like tool to find more dependencies
    Write-Host "Checking dependencies with objdump..." -ForegroundColor Yellow
    $deps = & x86_64-w64-mingw32-objdump -p "dist/Windows/$binaryName" 2>$null | Select-String "DLL Name"

    # Copy any additional system DLLs needed
    $systemDlls = @(
        "libuxtheme.dll",
        "dwmapi.dll",
        "shcore.dll",
        "shell32.dll",
        "user32.dll",
        "kernel32.dll"
    )
}

# List contents
Write-Host "`nDistribution contents:" -ForegroundColor Cyan
Get-ChildItem "dist/Windows" -Recurse | Select-Object FullName | Format-Table -HideTableHeaders

# Create ZIP
Write-Host "`nCreating ZIP file..." -ForegroundColor Cyan
Compress-Archive -Path "dist/Windows/*" -DestinationPath "dist/Juice-Zero-Bugs-Given-Windows.zip" -Force

Write-Host "`n=== Bundle Complete! ===" -ForegroundColor Green
Write-Host "ZIP file: dist/Juice-Zero-Bugs-Given-Windows.zip"
Write-Host "`nTo run on Windows:" -ForegroundColor Yellow
Write-Host "  1. Extract the ZIP file" -ForegroundColor Gray
Write-Host "  2. Run fighter_game.exe" -ForegroundColor Gray

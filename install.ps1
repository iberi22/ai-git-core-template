# install.ps1 - Remote installer for Git-Core Protocol (Windows)
# Usage: irm https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.ps1 | iex
#
# Or with parameters:
#   $env:GIT_CORE_ORGANIZE = "1"; irm .../install.ps1 | iex
#   $env:GIT_CORE_AUTO = "1"; irm .../install.ps1 | iex
#
# 🎯 This script can be executed by AI agents to bootstrap any project with Git-Core Protocol

$ErrorActionPreference = "Stop"

$REPO_URL = "https://github.com/iberi22/ai-git-core-template"
$TEMP_DIR = ".git-core-temp"

Write-Host "🧠 Git-Core Protocol - Remote Installer (Windows)" -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host ""

# Check for environment variable flags
$OrganizeFiles = $env:GIT_CORE_ORGANIZE -eq "1"
$AutoMode = $env:GIT_CORE_AUTO -eq "1"

# Function to organize existing files
function Invoke-OrganizeFiles {
    Write-Host "📂 Organizando archivos existentes..." -ForegroundColor Yellow
    
    # Create directories
    $dirs = @("docs/archive", "scripts", "tests", "src")
    foreach ($dir in $dirs) {
        New-Item -ItemType Directory -Force -Path $dir -ErrorAction SilentlyContinue | Out-Null
    }
    
    # Files to keep in root
    $keepInRoot = @("README.md", "AGENTS.md", "CHANGELOG.md", "CONTRIBUTING.md", "LICENSE.md", "LICENSE")
    
    # Move markdown files to docs/archive
    Get-ChildItem -Filter "*.md" -File -ErrorAction SilentlyContinue | ForEach-Object {
        if ($_.Name -notin $keepInRoot) {
            Move-Item $_.FullName -Destination "docs/archive/" -Force -ErrorAction SilentlyContinue
            Write-Host "  → $($_.Name) movido a docs/archive/" -ForegroundColor Cyan
        } else {
            Write-Host "  ✓ Manteniendo $($_.Name) en root" -ForegroundColor Green
        }
    }
    
    # Move test files
    $testPatterns = @("test_*.py", "*_test.py", "*.test.js", "*.test.ts", "*.spec.js", "*.spec.ts")
    foreach ($pattern in $testPatterns) {
        Get-ChildItem -Filter $pattern -File -ErrorAction SilentlyContinue | ForEach-Object {
            Move-Item $_.FullName -Destination "tests/" -Force -ErrorAction SilentlyContinue
            Write-Host "  → $($_.Name) movido a tests/" -ForegroundColor Cyan
        }
    }
    
    # Move loose scripts
    Get-ChildItem -Filter "*.bat" -File -ErrorAction SilentlyContinue | ForEach-Object {
        if ($_.DirectoryName -eq (Get-Location).Path) {
            Move-Item $_.FullName -Destination "scripts/" -Force -ErrorAction SilentlyContinue
            Write-Host "  → $($_.Name) movido a scripts/" -ForegroundColor Cyan
        }
    }
    
    Write-Host "✅ Archivos organizados" -ForegroundColor Green
}

# Check if should organize
if ($OrganizeFiles) {
    Invoke-OrganizeFiles
}

# Check if directory has files
$hasFiles = (Get-ChildItem -File -ErrorAction SilentlyContinue | Where-Object { $_.Name -notlike ".*" } | Measure-Object).Count -gt 0

if ($hasFiles -and -not $AutoMode) {
    Write-Host "⚠️  El directorio actual no está vacío." -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Opciones:"
    Write-Host "  1) Continuar y mezclar archivos"
    Write-Host "  2) Organizar archivos existentes primero (mover .md a docs/archive/)"
    Write-Host "  3) Cancelar"
    Write-Host ""
    $choice = Read-Host "Selecciona (1/2/3)"
    
    switch ($choice) {
        "1" { Write-Host "Continuando..." }
        "2" { Invoke-OrganizeFiles }
        "3" { Write-Host "Cancelado."; exit 0 }
        default { Write-Host "Opción inválida."; exit 1 }
    }
}

# Download template
Write-Host "`n📥 Descargando Git-Core Protocol template..." -ForegroundColor Cyan

try {
    git clone --depth 1 $REPO_URL $TEMP_DIR 2>$null
} catch {
    Write-Host "❌ Error al clonar el repositorio" -ForegroundColor Red
    exit 1
}

# Remove template's git history
Remove-Item -Recurse -Force "$TEMP_DIR/.git" -ErrorAction SilentlyContinue

# Copy files
Write-Host "📦 Instalando archivos del protocolo..." -ForegroundColor Cyan

# Copy directories
$dirs = @(".ai", ".github", "scripts")
foreach ($dir in $dirs) {
    if (Test-Path "$TEMP_DIR/$dir") {
        if (-not (Test-Path $dir)) {
            Copy-Item -Recurse "$TEMP_DIR/$dir" .
        } else {
            Copy-Item -Recurse -Force "$TEMP_DIR/$dir/*" $dir
        }
        Write-Host "  ✓ $dir/" -ForegroundColor Green
    }
}

# Copy config files (only if they don't exist)
$configFiles = @(".cursorrules", ".windsurfrules", ".gitignore", "AGENTS.md")
foreach ($file in $configFiles) {
    if ((Test-Path "$TEMP_DIR/$file") -and -not (Test-Path $file)) {
        Copy-Item "$TEMP_DIR/$file" .
        Write-Host "  ✓ $file" -ForegroundColor Green
    } elseif (Test-Path $file) {
        Write-Host "  ~ $file (ya existe, no sobrescrito)" -ForegroundColor Yellow
    }
}

# Copy README only if it doesn't exist
if (-not (Test-Path "README.md")) {
    Copy-Item "$TEMP_DIR/README.md" .
    Write-Host "  ✓ README.md" -ForegroundColor Green
} else {
    Write-Host "  ~ README.md (ya existe, no sobrescrito)" -ForegroundColor Yellow
}

# Cleanup
Remove-Item -Recurse -Force $TEMP_DIR -ErrorAction SilentlyContinue

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "✅ Git-Core Protocol instalado" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "📋 Archivos instalados:"
Write-Host "   .ai/ARCHITECTURE.md    - Documenta tu arquitectura aquí"
Write-Host "   .ai/CONTEXT_LOG.md     - Notas de sesión (efímeras)"
Write-Host "   .github/               - Copilot rules + Issue templates"
Write-Host "   scripts/               - Scripts de inicialización"
Write-Host "   AGENTS.md              - Reglas para todos los AI agents"
Write-Host "   .cursorrules           - Reglas para Cursor"
Write-Host "   .windsurfrules         - Reglas para Windsurf"
Write-Host ""
Write-Host "🚀 Siguiente paso:" -ForegroundColor Yellow
Write-Host "   .\scripts\init_project.ps1"
Write-Host ""
Write-Host "💡 Tip para AI Agents: Usa variables de entorno para modo no-interactivo" -ForegroundColor Cyan
Write-Host '   $env:GIT_CORE_AUTO = "1"; $env:GIT_CORE_ORGANIZE = "1"' -ForegroundColor Cyan

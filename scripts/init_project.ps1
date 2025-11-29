# scripts/init_project.ps1
# 🧠 Git-Core Protocol - Project Initializer (PowerShell)
# 
# Options:
#   -Organize    Organize existing files before creating repo
#   -Auto        Non-interactive mode (auto-accept defaults)
#
# Usage:
#   .\init_project.ps1
#   .\init_project.ps1 -Organize
#   .\init_project.ps1 -Auto -Organize

param(
    [switch]$Organize,
    [switch]$Auto
)

$ErrorActionPreference = "Stop"

# Function to organize existing files
function Invoke-OrganizeFiles {
    Write-Host "`n📂 Organizando archivos existentes..." -ForegroundColor Yellow
    
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
    
    # Move loose scripts (except init scripts)
    $scriptKeep = @("install.sh")
    Get-ChildItem -Filter "*.sh" -File -ErrorAction SilentlyContinue | ForEach-Object {
        if ($_.Name -notin $scriptKeep -and $_.DirectoryName -eq (Get-Location).Path) {
            Move-Item $_.FullName -Destination "scripts/" -Force -ErrorAction SilentlyContinue
            Write-Host "  → $($_.Name) movido a scripts/" -ForegroundColor Cyan
        }
    }
    Get-ChildItem -Filter "*.bat" -File -ErrorAction SilentlyContinue | ForEach-Object {
        if ($_.DirectoryName -eq (Get-Location).Path) {
            Move-Item $_.FullName -Destination "scripts/" -Force -ErrorAction SilentlyContinue
            Write-Host "  → $($_.Name) movido a scripts/" -ForegroundColor Cyan
        }
    }
    
    Write-Host "✅ Archivos organizados" -ForegroundColor Green
}

Write-Host "🧠 Inicializando Protocolo AI Git-Core..." -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

# Run organize if requested
if ($Organize) {
    Invoke-OrganizeFiles
}

# 1. Validate environment
Write-Host "`n📋 Validando entorno..." -ForegroundColor Yellow

if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Error: Git no está instalado." -ForegroundColor Red
    exit 1
}
Write-Host "✓ Git instalado" -ForegroundColor Green

if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Error: GitHub CLI (gh) no está instalado." -ForegroundColor Red
    Write-Host "  Instálalo desde: https://cli.github.com/" -ForegroundColor Yellow
    exit 1
}
Write-Host "✓ GitHub CLI instalado" -ForegroundColor Green

# Check if gh is authenticated
$authStatus = gh auth status 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error: No estás autenticado en GitHub CLI." -ForegroundColor Red
    Write-Host "  Ejecuta: gh auth login" -ForegroundColor Yellow
    exit 1
}
Write-Host "✓ GitHub CLI autenticado" -ForegroundColor Green

# 2. Get project name
$PROJECT_NAME = Split-Path -Leaf (Get-Location)
Write-Host "`n📁 Proyecto: $PROJECT_NAME" -ForegroundColor Yellow

# 3. Initialize Git if needed
if (-not (Test-Path ".git")) {
    Write-Host "`n🔧 Inicializando repositorio Git..." -ForegroundColor Yellow
    git init
    git add .
    git commit -m "feat: 🚀 Initial commit with Git-Core Protocol"
}

# 4. Create GitHub repository
Write-Host "`n☁️  Creando repositorio en GitHub..." -ForegroundColor Yellow

if ($Auto) {
    $PRIVATE_CHOICE = "N"  # Default to public in auto mode
    Write-Host "  (Modo auto: creando repositorio público)" -ForegroundColor Cyan
} else {
    $PRIVATE_CHOICE = Read-Host "¿Repositorio privado? (y/N)"
}

if ($PRIVATE_CHOICE -match "^[Yy]$") {
    gh repo create $PROJECT_NAME --private --source=. --remote=origin --push
} else {
    gh repo create $PROJECT_NAME --public --source=. --remote=origin --push
}

# 5. Setup Architecture file if empty
$archFile = ".ai/ARCHITECTURE.md"
if (-not (Test-Path $archFile) -or (Get-Item $archFile).Length -eq 0) {
    Write-Host "`n📐 Configurando ARCHITECTURE.md..." -ForegroundColor Yellow
    New-Item -ItemType Directory -Force -Path ".ai" | Out-Null
    @"
# 🏗️ Architecture

## Stack
- **Language:** TBD
- **Framework:** TBD
- **Database:** TBD

## Key Decisions
_Document architectural decisions here_

## Project Structure
```
TBD
```
"@ | Set-Content $archFile -Encoding UTF8
}

# 6. Create Semantic Labels for AI
Write-Host "`n🏷️  Creando etiquetas semánticas..." -ForegroundColor Yellow

function Create-Label {
    param($name, $description, $color)
    
    $existingLabels = gh label list --json name | ConvertFrom-Json
    if ($existingLabels.name -notcontains $name) {
        gh label create $name --description $description --color $color 2>$null
        Write-Host "  ✓ $name" -ForegroundColor Green
    } else {
        Write-Host "  ~ $name (ya existe)" -ForegroundColor Yellow
    }
}

Create-Label "ai-plan" "Tareas de planificación de alto nivel" "0E8A16"
Create-Label "ai-context" "Información crítica para el contexto" "FBCA04"
Create-Label "ai-blocked" "Bloqueado - requiere intervención humana" "D93F0B"
Create-Label "in-progress" "Tarea en progreso" "1D76DB"
Create-Label "needs-review" "Requiere revisión" "5319E7"

# 7. Create Initial Issues
Write-Host "`n📝 Creando issues iniciales..." -ForegroundColor Yellow

gh issue create `
    --title "🏗️ SETUP: Definir Arquitectura y Stack Tecnológico" `
    --body @"
## Objetivo
Definir y documentar las decisiones arquitectónicas del proyecto.

## Tareas
- [ ] Definir lenguaje/framework principal
- [ ] Definir base de datos (si aplica)
- [ ] Definir estructura de carpetas
- [ ] Documentar en ``.ai/ARCHITECTURE.md``

## Notas para AI Agent
Lee los requisitos del proyecto y propón un stack adecuado.
"@ `
    --label "ai-plan"

gh issue create `
    --title "⚙️ INFRA: Configuración inicial del entorno de desarrollo" `
    --body @"
## Objetivo
Configurar las herramientas de desarrollo.

## Tareas
- [ ] Configurar linter
- [ ] Configurar formatter
- [ ] Configurar pre-commit hooks (opcional)
- [ ] Crear estructura de carpetas base
- [ ] Agregar dependencias iniciales

## Notas para AI Agent
Usa las mejores prácticas del stack elegido.
"@ `
    --label "ai-plan"

gh issue create `
    --title "📚 DOCS: Documentación inicial del proyecto" `
    --body @"
## Objetivo
Crear documentación básica.

## Tareas
- [ ] Actualizar README.md con descripción del proyecto
- [ ] Documentar cómo ejecutar el proyecto
- [ ] Documentar cómo contribuir

## Notas para AI Agent
Mantén la documentación concisa y práctica.
"@ `
    --label "ai-plan"

# 8. Final message
Write-Host "`n==========================================" -ForegroundColor Cyan
Write-Host "✅ ¡Proyecto inicializado exitosamente!" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""
$username = (gh api user --jq .login)
Write-Host "📍 Repositorio: https://github.com/$username/$PROJECT_NAME" -ForegroundColor White
Write-Host ""
Write-Host "🚀 Próximos pasos:" -ForegroundColor Yellow
Write-Host "   1. Abre el proyecto en tu editor AI (Cursor/Windsurf/VS Code)"
Write-Host "   2. Escribe: 'Empieza con el primer issue asignado'"
Write-Host "   3. El agente leerá las reglas y comenzará a trabajar"
Write-Host ""
Write-Host "📋 Issues creados:" -ForegroundColor Yellow
gh issue list --limit 5

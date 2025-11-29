#!/bin/bash
# scripts/init_project.sh
# 🧠 Git-Core Protocol - Project Initializer

set -e

echo "🧠 Inicializando Protocolo AI Git-Core..."
echo "=========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 1. Validate environment
echo -e "\n📋 Validando entorno..."

if ! command -v git &> /dev/null; then
    echo -e "${RED}❌ Error: Git no está instalado.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Git instalado${NC}"

if ! command -v gh &> /dev/null; then
    echo -e "${RED}❌ Error: GitHub CLI (gh) no está instalado.${NC}"
    echo "  Instálalo desde: https://cli.github.com/"
    exit 1
fi
echo -e "${GREEN}✓ GitHub CLI instalado${NC}"

# Check if gh is authenticated
if ! gh auth status &> /dev/null; then
    echo -e "${RED}❌ Error: No estás autenticado en GitHub CLI.${NC}"
    echo "  Ejecuta: gh auth login"
    exit 1
fi
echo -e "${GREEN}✓ GitHub CLI autenticado${NC}"

# 2. Get project name
PROJECT_NAME=$(basename "$PWD")
echo -e "\n📁 Proyecto: ${YELLOW}${PROJECT_NAME}${NC}"

# 3. Initialize Git if needed
if [ ! -d ".git" ]; then
    echo -e "\n🔧 Inicializando repositorio Git..."
    git init
    git add .
    git commit -m "feat: 🚀 Initial commit with Git-Core Protocol"
fi

# 4. Create GitHub repository
echo -e "\n☁️  Creando repositorio en GitHub..."
read -p "¿Repositorio privado? (y/N): " PRIVATE_CHOICE

if [[ $PRIVATE_CHOICE =~ ^[Yy]$ ]]; then
    gh repo create "$PROJECT_NAME" --private --source=. --remote=origin --push
else
    gh repo create "$PROJECT_NAME" --public --source=. --remote=origin --push
fi

# 5. Setup Architecture file if empty
if [ ! -s .ai/ARCHITECTURE.md ] || [ ! -f .ai/ARCHITECTURE.md ]; then
    echo -e "\n📐 Configurando ARCHITECTURE.md..."
    mkdir -p .ai
    cat > .ai/ARCHITECTURE.md << 'EOF'
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
EOF
fi

# 6. Create Semantic Labels for AI
echo -e "\n🏷️  Creando etiquetas semánticas..."

# Function to create label if it doesn't exist
create_label() {
    local name=$1
    local description=$2
    local color=$3
    
    if ! gh label list | grep -q "$name"; then
        gh label create "$name" --description "$description" --color "$color" 2>/dev/null || true
        echo -e "  ${GREEN}✓ $name${NC}"
    else
        echo -e "  ${YELLOW}~ $name (ya existe)${NC}"
    fi
}

create_label "ai-plan" "Tareas de planificación de alto nivel" "0E8A16"
create_label "ai-context" "Información crítica para el contexto" "FBCA04"
create_label "ai-blocked" "Bloqueado - requiere intervención humana" "D93F0B"
create_label "in-progress" "Tarea en progreso" "1D76DB"
create_label "needs-review" "Requiere revisión" "5319E7"

# 7. Create Initial Issues
echo -e "\n📝 Creando issues iniciales..."

gh issue create \
    --title "🏗️ SETUP: Definir Arquitectura y Stack Tecnológico" \
    --body "## Objetivo
Definir y documentar las decisiones arquitectónicas del proyecto.

## Tareas
- [ ] Definir lenguaje/framework principal
- [ ] Definir base de datos (si aplica)
- [ ] Definir estructura de carpetas
- [ ] Documentar en \`.ai/ARCHITECTURE.md\`

## Notas para AI Agent
Lee los requisitos del proyecto y propón un stack adecuado." \
    --label "ai-plan"

gh issue create \
    --title "⚙️ INFRA: Configuración inicial del entorno de desarrollo" \
    --body "## Objetivo
Configurar las herramientas de desarrollo.

## Tareas
- [ ] Configurar linter
- [ ] Configurar formatter
- [ ] Configurar pre-commit hooks (opcional)
- [ ] Crear estructura de carpetas base
- [ ] Agregar dependencias iniciales

## Notas para AI Agent
Usa las mejores prácticas del stack elegido." \
    --label "ai-plan"

gh issue create \
    --title "📚 DOCS: Documentación inicial del proyecto" \
    --body "## Objetivo
Crear documentación básica.

## Tareas
- [ ] Actualizar README.md con descripción del proyecto
- [ ] Documentar cómo ejecutar el proyecto
- [ ] Documentar cómo contribuir

## Notas para AI Agent
Mantén la documentación concisa y práctica." \
    --label "ai-plan"

# 8. Final message
echo -e "\n=========================================="
echo -e "${GREEN}✅ ¡Proyecto inicializado exitosamente!${NC}"
echo -e "=========================================="
echo ""
echo "📍 Repositorio: https://github.com/$(gh api user --jq .login)/$PROJECT_NAME"
echo ""
echo "🚀 Próximos pasos:"
echo "   1. Abre el proyecto en tu editor AI (Cursor/Windsurf/VS Code)"
echo "   2. Escribe: 'Empieza con el primer issue asignado'"
echo "   3. El agente leerá las reglas y comenzará a trabajar"
echo ""
echo "📋 Issues creados:"
gh issue list --limit 5

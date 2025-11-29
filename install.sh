#!/bin/bash
# install.sh - Remote installer for Git-Core Protocol
# Usage: curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash
#
# 🎯 This script can be executed by AI agents to bootstrap any project with Git-Core Protocol
# Options:
#   --organize, -o    Organize existing files before installing
#   --auto, -y        Non-interactive mode (auto-accept)

set -e

REPO_URL="https://github.com/iberi22/ai-git-core-template"
TEMP_DIR=".git-core-temp"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}🧠 Git-Core Protocol - Remote Installer${NC}"
echo "========================================"
echo ""

# Parse arguments
ORGANIZE_FILES=false
AUTO_MODE=false
for arg in "$@"; do
    case $arg in
        --organize|-o)
            ORGANIZE_FILES=true
            ;;
        --auto|-y)
            AUTO_MODE=true
            ;;
    esac
done

# Function to organize existing files
organize_existing_files() {
    echo -e "${YELLOW}📂 Organizando archivos existentes...${NC}"

    # Create necessary directories
    mkdir -p docs/archive scripts tests src

    # Move markdown files to docs/archive (except special ones)
    for file in *.md; do
        if [ -f "$file" ]; then
            case "$file" in
                README.md|AGENTS.md|CHANGELOG.md|CONTRIBUTING.md|LICENSE.md)
                    echo -e "  ${GREEN}✓ Manteniendo $file en root${NC}"
                    ;;
                *)
                    mv "$file" "docs/archive/" 2>/dev/null && \
                    echo -e "  ${CYAN}→ $file movido a docs/archive/${NC}" || true
                    ;;
            esac
        fi
    done

    # Move test files
    for pattern in test_*.py *_test.py *_test.js *.test.js *.test.ts test_*.js *.spec.js *.spec.ts; do
        for file in $pattern; do
            if [ -f "$file" ] && [ "$file" != "$pattern" ]; then
                mv "$file" "tests/" 2>/dev/null && \
                echo -e "  ${CYAN}→ $file movido a tests/${NC}" || true
            fi
        done
    done

    # Move loose script files to scripts/ (except install.sh)
    for file in *.sh *.ps1 *.bat; do
        if [ -f "$file" ] && [ "$file" != "$pattern" ]; then
            case "$file" in
                install.sh)
                    echo -e "  ${GREEN}✓ Manteniendo $file en root${NC}"
                    ;;
                *)
                    mv "$file" "scripts/" 2>/dev/null && \
                    echo -e "  ${CYAN}→ $file movido a scripts/${NC}" || true
                    ;;
            esac
        fi
    done

    echo -e "${GREEN}✅ Archivos organizados${NC}"
}

# Check if we should auto-organize
if [ "$ORGANIZE_FILES" = true ]; then
    organize_existing_files
fi

# Check if current directory has files
if [ "$(ls -A 2>/dev/null | grep -v '^\.' | head -1)" ] && [ "$AUTO_MODE" = false ]; then
    echo -e "${YELLOW}⚠️  El directorio actual no está vacío.${NC}"
    echo ""
    echo "Opciones:"
    echo "  1) Continuar y mezclar archivos"
    echo "  2) Organizar archivos existentes primero (mover .md a docs/archive/)"
    echo "  3) Cancelar"
    echo ""
    read -p "Selecciona (1/2/3): " CHOICE

    case $CHOICE in
        1)
            echo "Continuando..."
            ;;
        2)
            organize_existing_files
            ;;
        3)
            echo "Cancelado."
            exit 0
            ;;
        *)
            echo "Opción inválida. Cancelando."
            exit 1
            ;;
    esac
fi

# Download template
echo -e "\n${CYAN}📥 Descargando Git-Core Protocol template...${NC}"
git clone --depth 1 "$REPO_URL" "$TEMP_DIR" 2>/dev/null || {
    echo -e "${RED}❌ Error al clonar el repositorio${NC}"
    exit 1
}

# Remove template's git history
rm -rf "$TEMP_DIR/.git"

# Move template files (don't overwrite existing)
echo -e "${CYAN}📦 Instalando archivos del protocolo...${NC}"

# Copy directories
for dir in .ai .github scripts; do
    if [ -d "$TEMP_DIR/$dir" ]; then
        cp -rn "$TEMP_DIR/$dir" . 2>/dev/null || cp -r "$TEMP_DIR/$dir" .
        echo -e "  ${GREEN}✓ $dir/${NC}"
    fi
done

# Copy config files (only if they don't exist)
for file in .cursorrules .windsurfrules .gitignore AGENTS.md; do
    if [ -f "$TEMP_DIR/$file" ] && [ ! -f "$file" ]; then
        cp "$TEMP_DIR/$file" .
        echo -e "  ${GREEN}✓ $file${NC}"
    elif [ -f "$file" ]; then
        echo -e "  ${YELLOW}~ $file (ya existe, no sobrescrito)${NC}"
    fi
done

# Copy README only if it doesn't exist
if [ ! -f "README.md" ]; then
    cp "$TEMP_DIR/README.md" .
    echo -e "  ${GREEN}✓ README.md${NC}"
else
    echo -e "  ${YELLOW}~ README.md (ya existe, no sobrescrito)${NC}"
fi

# Cleanup temp
rm -rf "$TEMP_DIR"

# Make scripts executable
chmod +x scripts/*.sh 2>/dev/null || true

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}✅ Git-Core Protocol instalado${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "📋 Archivos instalados:"
echo "   .ai/ARCHITECTURE.md    - Documenta tu arquitectura aquí"
echo "   .ai/CONTEXT_LOG.md     - Notas de sesión (efímeras)"
echo "   .github/               - Copilot rules + Issue templates"
echo "   scripts/               - Scripts de inicialización"
echo "   AGENTS.md              - Reglas para todos los AI agents"
echo "   .cursorrules           - Reglas para Cursor"
echo "   .windsurfrules         - Reglas para Windsurf"
echo ""
echo -e "${YELLOW}🚀 Siguiente paso:${NC}"
echo "   ./scripts/init_project.sh"
echo ""
echo -e "${CYAN}💡 Tip para AI Agents: Usa --auto para modo no-interactivo${NC}"
echo -e "${CYAN}   curl -sL .../install.sh | bash -s -- --auto --organize${NC}"

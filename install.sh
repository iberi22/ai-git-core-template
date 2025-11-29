#!/bin/bash
# install.sh - Remote installer for Git-Core Protocol
# Usage: curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash

set -e

echo "🧠 Git-Core Protocol - Remote Installer"
echo "========================================"

# Check if current directory is empty (except hidden files)
if [ "$(ls -A 2>/dev/null | grep -v '^\..*')" ]; then
    echo "⚠️  Warning: Current directory is not empty."
    read -p "Continue anyway? (y/N): " CONTINUE
    if [[ ! $CONTINUE =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 1
    fi
fi

# Download template
echo "📥 Descargando template..."
git clone --depth 1 https://github.com/iberi22/ai-git-core-template.git .git-core-temp

# Move files
mv .git-core-temp/* .git-core-temp/.* . 2>/dev/null || true
rm -rf .git-core-temp .git

echo "✅ Template descargado."
echo ""
echo "🚀 Ejecuta ahora: ./scripts/init_project.sh"

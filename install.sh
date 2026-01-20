#!/bin/bash
# install.sh - Remote installer for Git-Core Protocol
# Usage: curl -fsSL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash
#
# 🎯 This script can be executed by AI agents to bootstrap any project with Git-Core Protocol
# Options:
#   --organize, -o    Organize existing files before installing
#   --auto, -y        Non-interactive mode (auto-accept)
#   --upgrade, -u     Upgrade existing installation (PRESERVES ARCHITECTURE.md)
#   --force, -f       Force upgrade (overwrites EVERYTHING including ARCHITECTURE.md)

set -e

REPO_URL="https://github.com/iberi22/Git-Core-Protocol"
RAW_URL="https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main"
TEMP_DIR=".git-core-temp"
BACKUP_DIR=".git-core-backup"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${CYAN}🧠 Git-Core Protocol - Remote Installer v3.5.1${NC}"
echo "=============================================="
echo ""

# Cleanup trap
cleanup() {
    if [ -d "$TEMP_DIR" ]; then
        rm -rf "$TEMP_DIR"
    fi
}
trap cleanup EXIT

# Check dependencies
check_dependencies() {
    local missing_deps=0
    for dep in git curl; do
        if ! command -v "$dep" &> /dev/null; then
            echo -e "${RED}❌ Error: '$dep' is not installed.${NC}"
            missing_deps=1
        fi
    done

    if [ $missing_deps -ne 0 ]; then
        echo -e "${YELLOW}Please install missing dependencies and try again.${NC}"
        exit 1
    fi
}
check_dependencies

# Parse arguments
ORGANIZE_FILES=false
AUTO_MODE=false
UPGRADE_MODE=false
FORCE_MODE=false
NO_BINARIES=false

for arg in "$@"; do
    case $arg in
        --organize|-o)
            ORGANIZE_FILES=true
            ;;
        --auto|-y)
            AUTO_MODE=true
            ;;
        --upgrade|-u)
            UPGRADE_MODE=true
            AUTO_MODE=true
            ;;
        --force|-f)
            FORCE_MODE=true
            UPGRADE_MODE=true
            AUTO_MODE=true
            ;;
        --no-binaries)
            NO_BINARIES=true
            ;;
        --help|-h)
            echo "Usage: install.sh [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --organize, -o    Organize existing files before installing"
            echo "  --auto, -y        Non-interactive mode"
            echo "  --upgrade, -u     Upgrade protocol files (PRESERVES your ARCHITECTURE.md)"
            echo "  --force, -f       Force full upgrade (overwrites everything)"
            echo "  --no-binaries     Skip installing pre-compiled binaries"
            echo "  --help, -h        Show this help"
            echo ""
            echo "Examples:"
            echo "  curl -fsSL .../install.sh | bash                    # New install"
            echo "  curl -fsSL .../install.sh | bash -s -- --upgrade    # Safe upgrade"
            echo "  curl -fsSL .../install.sh | bash -s -- --force      # Full reset"
            exit 0
            ;;
    esac
done

# Show mode
if [ "$FORCE_MODE" = true ]; then
    echo -e "${RED}⚠️  FORCE MODE: ALL files will be overwritten (including ARCHITECTURE.md)${NC}"
elif [ "$UPGRADE_MODE" = true ]; then
    echo -e "${YELLOW}🔄 UPGRADE MODE: Protocol files updated, your ARCHITECTURE.md preserved${NC}"
fi
echo ""

# Function to get current version
get_current_version() {
    if [ -f ".git-core-protocol-version" ]; then
        cat ".git-core-protocol-version" | tr -d '[:space:]'
    else
        echo "0.0.0"
    fi
}

# Function to get remote version
get_remote_version() {
    curl -fsSL "$RAW_URL/.git-core-protocol-version" 2>/dev/null | tr -d '[:space:]' || echo "unknown"
}

# Show version info
CURRENT_VERSION=$(get_current_version)
if [ "$CURRENT_VERSION" != "0.0.0" ]; then
    REMOTE_VERSION=$(get_remote_version)
    echo -e "${BLUE}📊 Version Info:${NC}"
    echo -e "   Current: ${YELLOW}$CURRENT_VERSION${NC}"
    echo -e "   Latest:  ${GREEN}$REMOTE_VERSION${NC}"
    echo ""
fi

# Function to migrate from legacy directories to .gitcore/
migrate_ai_directory() {
    HAS_LEGACY=false
    # Priority: .gitcore (old) > .ai (older)
    # Logic: Rename (mv) if .gitcore doesn't exist to preserve exact state (Smart Update)
    # Fallback: Copy (cp) if .gitcore exists to merge

    for legacy in ".gitcore" ".ai"; do
        if [ -d "$legacy" ]; then
            echo -e "${YELLOW}🔄 Detected legacy $legacy directory...${NC}"

            if [ ! -d ".gitcore" ]; then
                # Smart Migration: Atomic Rename
                mv "$legacy" ".gitcore"
                echo -e "  ${GREEN}✓ Renamed $legacy → .gitcore/ (Smart Update)${NC}"
                HAS_LEGACY=true
            else
                # Fallback: Merge
                echo -e "  ${YELLOW}→ .gitcore exists, merging content...${NC}"
                cp -r "$legacy"/* .gitcore/ 2>/dev/null || true
                echo -e "  ${GREEN}✓ Merged $legacy → .gitcore/${NC}"
                echo -e "  ${CYAN}ℹ️  You can remove $legacy manually${NC}"
                HAS_LEGACY=true
            fi
        fi
    done

    if [ "$HAS_LEGACY" = true ]; then
        return 0
    else
        return 1
    fi
}

# Function to backup user files
backup_user_files() {
    echo -e "${CYAN}💾 Backing up user files...${NC}"
    mkdir -p "$BACKUP_DIR"

    # Check both .gitcore, .gitcore and .ai for backwards compatibility
    AI_DIR=""
    if [ -d ".gitcore" ]; then
        AI_DIR=".gitcore"
    elif [ -d ".gitcore" ]; then
        AI_DIR=".gitcore"
    elif [ -d ".ai" ]; then
        AI_DIR=".ai"
    fi

    # Backup ARCHITECTURE.md if it exists
    if [ -n "$AI_DIR" ] && [ -f "$AI_DIR/ARCHITECTURE.md" ]; then
        cp "$AI_DIR/ARCHITECTURE.md" "$BACKUP_DIR/ARCHITECTURE.md"
        echo -e "  ${GREEN}✓ $AI_DIR/ARCHITECTURE.md backed up${NC}"
    fi

    # Backup CONTEXT_LOG.md if it exists
    if [ -n "$AI_DIR" ] && [ -f "$AI_DIR/CONTEXT_LOG.md" ]; then
        cp "$AI_DIR/CONTEXT_LOG.md" "$BACKUP_DIR/CONTEXT_LOG.md"
        echo -e "  ${GREEN}✓ $AI_DIR/CONTEXT_LOG.md backed up${NC}"
    fi

    # Backup custom workflows
    if [ -d ".github/workflows" ]; then
        mkdir -p "$BACKUP_DIR/workflows"
        for file in .github/workflows/*.yml; do
            if [ -f "$file" ]; then
                filename=$(basename "$file")
                # Only backup non-protocol workflows
                case "$filename" in
                    update-protocol.yml|structure-validator.yml|codex-review.yml|agent-dispatcher.yml)
                        # Protocol workflows - don't backup
                        ;;
                    *)
                        cp "$file" "$BACKUP_DIR/workflows/"
                        echo -e "  ${GREEN}✓ Custom workflow: $filename${NC}"
                        ;;
                esac
            fi
        done
    fi
}

# Function to restore user files
restore_user_files() {
    echo -e "${CYAN}📥 Restoring user files...${NC}"

    # Ensure .gitcore directory exists for restoration
    mkdir -p ".gitcore"

    # Restore ARCHITECTURE.md (unless force mode)
    if [ "$FORCE_MODE" != true ] && [ -f "$BACKUP_DIR/ARCHITECTURE.md" ]; then
        cp "$BACKUP_DIR/ARCHITECTURE.md" ".gitcore/ARCHITECTURE.md"
        echo -e "  ${GREEN}✓ .gitcore/ARCHITECTURE.md restored${NC}"
    fi

    # Always restore CONTEXT_LOG.md
    if [ -f "$BACKUP_DIR/CONTEXT_LOG.md" ]; then
        cp "$BACKUP_DIR/CONTEXT_LOG.md" ".gitcore/CONTEXT_LOG.md"
        echo -e "  ${GREEN}✓ .gitcore/CONTEXT_LOG.md restored${NC}"
    fi

    # Restore custom workflows
    if [ -d "$BACKUP_DIR/workflows" ]; then
        for file in "$BACKUP_DIR/workflows"/*.yml; do
            if [ -f "$file" ]; then
                cp "$file" ".github/workflows/"
                echo -e "  ${GREEN}✓ Custom workflow restored: $(basename $file)${NC}"
            fi
        done
    fi

    # Cleanup backup
    rm -rf "$BACKUP_DIR"
}

# Function to organize existing files
organize_existing_files() {
    echo -e "${YELLOW}📂 Organizing existing files...${NC}"

    mkdir -p docs/archive scripts tests src

    for file in *.md; do
        if [ -f "$file" ]; then
            case "$file" in
                README.md|AGENTS.md|CHANGELOG.md|CONTRIBUTING.md|LICENSE.md)
                    echo -e "  ${GREEN}✓ Keeping $file in root${NC}"
                    ;;
                *)
                    mv "$file" "docs/archive/" 2>/dev/null && \
                    echo -e "  ${CYAN}→ $file moved to docs/archive/${NC}" || true
                    ;;
            esac
        fi
    done

    for pattern in test_*.py *_test.py *.test.js *.test.ts *.spec.js *.spec.ts; do
        for file in $pattern; do
            if [ -f "$file" ] && [ "$file" != "$pattern" ]; then
                mv "$file" "tests/" 2>/dev/null && \
                echo -e "  ${CYAN}→ $file moved to tests/${NC}" || true
            fi
        done
    done

    echo -e "${GREEN}✅ Files organized${NC}"
}

# Check if should organize
if [ "$ORGANIZE_FILES" = true ]; then
    organize_existing_files
fi

# Check if current directory has files
if [ "$(ls -A 2>/dev/null | grep -v '^\.' | head -1)" ] && [ "$AUTO_MODE" = false ]; then
    echo -e "${YELLOW}⚠️  Current directory is not empty.${NC}"
    echo ""
    echo "Options:"
    echo "  1) Continue and merge files"
    echo "  2) Organize existing files first"
    echo "  3) Cancel"
    echo ""
    read -p "Select (1/2/3): " CHOICE

    case $CHOICE in
        1) echo "Continuing..." ;;
        2) organize_existing_files ;;
        3) echo "Cancelled."; exit 0 ;;
        *) echo "Invalid option."; exit 1 ;;
    esac
fi

# Ask about binaries if not in auto mode
INSTALL_BINARIES=true
if [ "$AUTO_MODE" = false ] && [ "$NO_BINARIES" = false ]; then
    echo ""
    echo -e "${CYAN}📦 Optional Components:${NC}"
    echo "   Do you want to install pre-compiled AI agent binaries (bin/)?"
    echo "   These are useful for local execution but increase download size."
    read -p "   Install binaries? (Y/n) " bin_choice
    case "$bin_choice" in
        [nN][oO]|[nN])
            INSTALL_BINARIES=false
            echo -e "   ${YELLOW}→ Skipping binaries.${NC}"
            ;;
        *)
            echo -e "   ${GREEN}→ Installing binaries.${NC}"
            ;;
    esac
elif [ "$NO_BINARIES" = true ]; then
    INSTALL_BINARIES=false
fi

# Backup user files before upgrade
if [ "$UPGRADE_MODE" = true ]; then
    backup_user_files
fi

# Download template
echo -e "\n${CYAN}📥 Downloading Git-Core Protocol...${NC}"
git clone --depth 1 "$REPO_URL" "$TEMP_DIR" 2>/dev/null || {
    echo -e "${RED}❌ Error cloning repository${NC}"
    exit 1
}

rm -rf "$TEMP_DIR/.git"

# Install files
echo -e "${CYAN}📦 Installing protocol files...${NC}"

# Run migration from legacy to .gitcore/ if needed
migrate_ai_directory

# Handle .gitcore directory (protocol uses .gitcore, template may have .ai or .gitcore)
TEMPLATE_AI_DIR=""
if [ -d "$TEMP_DIR/.gitcore" ]; then
    TEMPLATE_AI_DIR="$TEMP_DIR/.gitcore"
elif [ -d "$TEMP_DIR/.gitcore" ]; then
    TEMPLATE_AI_DIR="$TEMP_DIR/.gitcore"
elif [ -d "$TEMP_DIR/.ai" ]; then
    TEMPLATE_AI_DIR="$TEMP_DIR/.ai"
fi

if [ -n "$TEMPLATE_AI_DIR" ]; then
    if [ "$UPGRADE_MODE" = true ]; then
        # Remove old directories
        rm -rf .gitcore .gitcore .ai 2>/dev/null || true

        # Copy to .gitcore
        mkdir -p ".gitcore"
        cp -r "$TEMPLATE_AI_DIR"/* .gitcore/
        echo -e "  ${GREEN}✓ .gitcore/ (upgraded)${NC}"
    elif [ ! -d ".gitcore" ] && [ ! -d ".gitcore" ] && [ ! -d ".ai" ]; then
        mkdir -p ".gitcore"
        cp -r "$TEMPLATE_AI_DIR"/* .gitcore/
        echo -e "  ${GREEN}✓ .gitcore/${NC}"
    else
        # Ensure target dir exists
        TARGET_DIR=".gitcore"
        if [ ! -d "$TARGET_DIR" ]; then
            if [ -d ".gitcore" ]; then TARGET_DIR=".gitcore"; else TARGET_DIR=".ai"; fi
        fi
        echo -e "  ${YELLOW}~ $TARGET_DIR/ (exists, merging new files)${NC}"
        for item in "$TEMPLATE_AI_DIR"/*; do
            filename=$(basename "$item")
            if [ ! -e "$TARGET_DIR/$filename" ]; then
                cp -r "$item" "$TARGET_DIR/"
                echo -e "    ${GREEN}+ $filename${NC}"
            fi
        done
    fi
fi

# Copy other directories
for dir in .github scripts docs bin; do
    if [ -d "$TEMP_DIR/$dir" ]; then
        if [ "$UPGRADE_MODE" = true ]; then
            rm -rf "$dir"
            cp -r "$TEMP_DIR/$dir" .

            # Cleanup internal files
            if [ "$dir" = ".github" ]; then
                rm -f ".github/workflows/build-tools.yml"
                rm -f ".github/workflows/release.yml"
            fi
            if [ "$dir" = "scripts" ]; then
                rm -f "scripts/bump-version.ps1"
                rm -f "scripts/bump-version.sh"
            fi

            echo -e "  ${GREEN}✓ $dir/ (upgraded)${NC}"
        elif [ ! -d "$dir" ]; then
            cp -r "$TEMP_DIR/$dir" .

            # Cleanup internal files
            if [ "$dir" = ".github" ]; then
                rm -f ".github/workflows/build-tools.yml"
                rm -f ".github/workflows/release.yml"
            fi
            if [ "$dir" = "scripts" ]; then
                rm -f "scripts/bump-version.ps1"
                rm -f "scripts/bump-version.sh"
            fi

            echo -e "  ${GREEN}✓ $dir/${NC}"
        else
            cp -rn "$TEMP_DIR/$dir"/* "$dir/" 2>/dev/null || true

            # Cleanup internal files
            if [ "$dir" = ".github" ]; then
                rm -f ".github/workflows/build-tools.yml"
                rm -f ".github/workflows/release.yml"
            fi
            if [ "$dir" = "scripts" ]; then
                rm -f "scripts/bump-version.ps1"
                rm -f "scripts/bump-version.sh"
            fi

            echo -e "  ${GREEN}✓ $dir/ (merged)${NC}"
        fi
    fi
done

# Protocol files
PROTOCOL_FILES=".cursorrules .windsurfrules AGENTS.md .git-core-protocol-version"
for file in $PROTOCOL_FILES; do
    if [ -f "$TEMP_DIR/$file" ]; then
        if [ "$UPGRADE_MODE" = true ]; then
            cp "$TEMP_DIR/$file" .
            echo -e "  ${GREEN}✓ $file (upgraded)${NC}"
        elif [ ! -f "$file" ]; then
            cp "$TEMP_DIR/$file" .
            echo -e "  ${GREEN}✓ $file${NC}"
        else
            echo -e "  ${YELLOW}~ $file (exists)${NC}"
        fi
    fi
done

# Files that should never be overwritten
PRESERVE_FILES=".gitignore README.md"
for file in $PRESERVE_FILES; do
    if [ -f "$TEMP_DIR/$file" ] && [ ! -f "$file" ]; then
        cp "$TEMP_DIR/$file" .
        echo -e "  ${GREEN}✓ $file${NC}"
    elif [ -f "$file" ]; then
        echo -e "  ${YELLOW}~ $file (preserved)${NC}"
    fi
done

# Cleanup temp
rm -rf "$TEMP_DIR"

# Restore user files after upgrade
if [ "$UPGRADE_MODE" = true ]; then
    restore_user_files
fi

# Make scripts executable
chmod +x scripts/*.sh 2>/dev/null || true

# Show final version
NEW_VERSION=$(get_current_version)

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}✅ Git-Core Protocol v$NEW_VERSION installed${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

if [ "$UPGRADE_MODE" = true ]; then
    echo -e "${CYAN}📋 Upgraded from v$CURRENT_VERSION → v$NEW_VERSION${NC}"
    if [ "$FORCE_MODE" != true ]; then
        echo -e "${GREEN}✓ Your ARCHITECTURE.md was preserved${NC}"
    fi
else
    echo -e "📋 Files installed:"
    echo "   .gitcore/ARCHITECTURE.md    - Document your architecture here"
    echo "   .github/               - Copilot rules + workflows"
    echo "   scripts/               - Init and update scripts"
    echo "   AGENTS.md              - Rules for all AI agents"
fi

echo ""
echo -e "${YELLOW}🚀 Next step:${NC}"
echo "   ./scripts/init_project.sh"
echo ""
echo -e "${CYAN}💡 Commands:${NC}"
echo "   Safe upgrade:  curl -fsSL .../install.sh | bash -s -- --upgrade"
echo "   Full reset:    curl -fsSL .../install.sh | bash -s -- --force"
echo "   Check updates: ./scripts/check-protocol-update.sh"

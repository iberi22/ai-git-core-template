# install.ps1 - Remote installer for Git-Core Protocol (Windows)
# Usage: irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
#
# 🎯 This script can be executed by AI agents to bootstrap any project with Git-Core Protocol
#
# Environment variables for options:
#   $env:GIT_CORE_ORGANIZE = "1"  - Organize existing files
#   $env:GIT_CORE_AUTO = "1"      - Non-interactive mode
#   $env:GIT_CORE_UPGRADE = "1"   - Upgrade (preserves ARCHITECTURE.md)
#   $env:GIT_CORE_FORCE = "1"     - Force upgrade (overwrites everything)
#   $env:GIT_CORE_MIGRATE = "1"   - Create new branch and auto-commit

$ErrorActionPreference = "Stop"

$REPO_URL = "https://github.com/iberi22/Git-Core-Protocol"
$RAW_URL = "https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main"
$TEMP_DIR = ".git-core-temp"
$BACKUP_DIR = ".git-core-backup"

# Define colors for consistency
$Red = "Red"
$Green = "Green"
$Yellow = "Yellow"
$Cyan = "Cyan"
$Blue = "Blue"

Write-Host "🧠 Git-Core Protocol - Remote Installer v3.5.1" -ForegroundColor $Cyan
Write-Host "==============================================" -ForegroundColor $Cyan
Write-Host ""

# Check dependencies
function Test-Dependency {
    param([string]$name)
    if (-not (Get-Command $name -ErrorAction SilentlyContinue)) {
        return $false
    }
    return $true
}

$dependencies = @("git", "curl")
$missingDeps = @()

foreach ($dep in $dependencies) {
    if (-not (Test-Dependency $dep)) {
        $missingDeps += $dep
    }
}

if ($missingDeps.Count -gt 0) {
    Write-Host "❌ Error: The following dependencies are missing: $($missingDeps -join ', ')" -ForegroundColor $Red
    Write-Host "Please install them and try again." -ForegroundColor $Yellow
    exit 1
}

# Check for environment variable flags
$OrganizeFiles = $env:GIT_CORE_ORGANIZE -eq "1"
$AutoMode = $env:GIT_CORE_AUTO -eq "1"
$UpgradeMode = $env:GIT_CORE_UPGRADE -eq "1"
$ForceMode = $env:GIT_CORE_FORCE -eq "1"
$NoBinaries = $env:GIT_CORE_NO_BINARIES -eq "1"
$MigrateMode = $env:GIT_CORE_MIGRATE -eq "1"

try {
    # Force implies upgrade and auto
    if ($ForceMode) {
        $UpgradeMode = $true
        $AutoMode = $true
        Write-Host "⚠️  FORCE MODE: ALL files will be overwritten (including ARCHITECTURE.md)" -ForegroundColor $Red
        Write-Host ""
    } elseif ($MigrateMode) {
        $AutoMode = $true
        Write-Host "🔀 MIGRATE MODE: Creating new branch and auto-committing changes" -ForegroundColor $Cyan
        Write-Host ""

        # Create branch name with timestamp
        $BranchName = "protocol-migration-$(Get-Date -Format 'yyyyMMdd-HHmmss')"

        # Check if we're in a git repo
        $gitDir = git rev-parse --git-dir 2>$null
        if (-not $gitDir) {
            Write-Host "⚠️  Not a git repository. Initializing..." -ForegroundColor $Yellow
            git init
        }

        # Create and switch to new branch
        Write-Host "🔀 Creating branch: $BranchName" -ForegroundColor $Cyan
        git checkout -b $BranchName 2>$null
        if ($LASTEXITCODE -ne 0) {
            throw "Failed to create branch"
        }
        Write-Host "✓ Switched to branch: $BranchName" -ForegroundColor $Green
        Write-Host ""
    } elseif ($UpgradeMode) {
        $AutoMode = $true
        Write-Host "🔄 UPGRADE MODE: Protocol files updated, your ARCHITECTURE.md preserved" -ForegroundColor $Yellow
        Write-Host ""
    }

    # Function to get current version
    function Get-CurrentVersion {
        if (Test-Path ".git-core-protocol-version") {
            return (Get-Content ".git-core-protocol-version" -Raw).Trim()
        }
        return "0.0.0"
    }

    # Function to get remote version
    function Get-RemoteVersion {
        try {
            $response = Invoke-WebRequest -Uri "$RAW_URL/.git-core-protocol-version" -UseBasicParsing -ErrorAction SilentlyContinue
            return $response.Content.Trim()
        } catch {
            return "unknown"
        }
    }

    # Show version info
    $CurrentVersion = Get-CurrentVersion
    if ($CurrentVersion -ne "0.0.0") {
        $RemoteVersion = Get-RemoteVersion
        Write-Host "📊 Version Info:" -ForegroundColor $Blue
        Write-Host "   Current: $CurrentVersion" -ForegroundColor $Yellow
        Write-Host "   Latest:  $RemoteVersion" -ForegroundColor $Green
        Write-Host ""
    }

    # Function to check if CLI is installed globally
    function Test-CliInstalled {
        $cliName = "context-research-agent"
        if ($IsWindows) { $cliName += ".exe" }

        $command = Get-Command $cliName -ErrorAction SilentlyContinue
        return $null -ne $command
    }

    # Function to migrate from legacy directories to .gitcore/
    function Invoke-Migration {
        $hasLegacy = $false
        $legacyPaths = @(".gitcore", ".ai")

        foreach ($legacy in $legacyPaths) {
            if (Test-Path $legacy) {
                Write-Host "🔄 Detected legacy $legacy directory..." -ForegroundColor $Yellow
                $hasLegacy = $true

                if (-not (Test-Path ".gitcore")) {
                    Move-Item -Path $legacy -Destination ".gitcore" -Force
                    Write-Host "  ✓ Renamed $legacy → .gitcore/ (Smart Update)" -ForegroundColor $Green
                } else {
                    Write-Host "  → .gitcore exists, merging content..." -ForegroundColor $Yellow
                    Get-ChildItem $legacy -Recurse | ForEach-Object {
                        $destPath = $_.FullName -replace [regex]::Escape($legacy), ".gitcore"
                        if ($_.PSIsContainer) {
                            if (-not (Test-Path $destPath)) {
                                New-Item -ItemType Directory -Force -Path $destPath | Out-Null
                            }
                        } else {
                            Copy-Item $_.FullName $destPath -Force
                        }
                    }
                    Write-Host "  ✓ Merged $legacy → .gitcore/" -ForegroundColor $Green
                    Write-Host "  ℹ️  You can remove $legacy manually" -ForegroundColor $Cyan
                }
            }
        }
        return $hasLegacy
    }

    # Function to backup user files
    function Backup-UserFiles {
        Write-Host "💾 Backing up user files..." -ForegroundColor $Cyan
        New-Item -ItemType Directory -Force -Path $BACKUP_DIR | Out-Null

        $aiDir = if (Test-Path ".gitcore") { ".gitcore" }  elseif (Test-Path ".ai") { ".ai" } else { $null }

        if ($aiDir -and (Test-Path "$aiDir/ARCHITECTURE.md")) {
            Copy-Item "$aiDir/ARCHITECTURE.md" "$BACKUP_DIR/ARCHITECTURE.md"
            Write-Host "  ✓ $aiDir/ARCHITECTURE.md backed up" -ForegroundColor $Green
        }

        if ($aiDir -and (Test-Path "$aiDir/CONTEXT_LOG.md")) {
            Copy-Item "$aiDir/CONTEXT_LOG.md" "$BACKUP_DIR/CONTEXT_LOG.md"
            Write-Host "  ✓ $aiDir/CONTEXT_LOG.md backed up" -ForegroundColor $Green
        }

        if (Test-Path ".github/workflows") {
            New-Item -ItemType Directory -Force -Path "$BACKUP_DIR/workflows" | Out-Null
            $protocolWorkflows = @("update-protocol.yml", "structure-validator.yml", "codex-review.yml", "agent-dispatcher.yml")

            Get-ChildItem ".github/workflows/*.yml" -ErrorAction SilentlyContinue | ForEach-Object {
                if ($_.Name -notin $protocolWorkflows) {
                    Copy-Item $_.FullName "$BACKUP_DIR/workflows/"
                    Write-Host "  ✓ Custom workflow: $($_.Name)" -ForegroundColor $Green
                }
            }
        }
    }

    # Function to restore user files
    function Restore-UserFiles {
        Write-Host "📥 Restoring user files..." -ForegroundColor $Cyan
        if (-not (Test-Path ".gitcore")) {
            New-Item -ItemType Directory -Force -Path ".gitcore" | Out-Null
        }

        if (-not $ForceMode -and (Test-Path "$BACKUP_DIR/ARCHITECTURE.md")) {
            Copy-Item "$BACKUP_DIR/ARCHITECTURE.md" ".gitcore/ARCHITECTURE.md" -Force
            Write-Host "  ✓ .gitcore/ARCHITECTURE.md restored" -ForegroundColor $Green
        }

        if (Test-Path "$BACKUP_DIR/CONTEXT_LOG.md") {
            Copy-Item "$BACKUP_DIR/CONTEXT_LOG.md" ".gitcore/CONTEXT_LOG.md" -Force
            Write-Host "  ✓ .gitcore/CONTEXT_LOG.md restored" -ForegroundColor $Green
        }

        if (Test-Path "$BACKUP_DIR/workflows") {
            Get-ChildItem "$BACKUP_DIR/workflows/*.yml" -ErrorAction SilentlyContinue | ForEach-Object {
                Copy-Item $_.FullName ".github/workflows/" -Force
                Write-Host "  ✓ Custom workflow restored: $($_.Name)" -ForegroundColor $Green
            }
        }
        Remove-Item -Recurse -Force $BACKUP_DIR -ErrorAction SilentlyContinue
    }

    # Function to organize existing files
    function Invoke-OrganizeFiles {
        Write-Host "📂 Organizing existing files..." -ForegroundColor $Yellow
        $dirs = @("docs/archive", "scripts", "tests", "src")
        foreach ($dir in $dirs) {
            New-Item -ItemType Directory -Force -Path $dir -ErrorAction SilentlyContinue | Out-Null
        }

        $keepInRoot = @("README.md", "AGENTS.md", "CHANGELOG.md", "CONTRIBUTING.md", "LICENSE.md", "LICENSE")
        Get-ChildItem -Filter "*.md" -File -ErrorAction SilentlyContinue | ForEach-Object {
            if ($_.Name -notin $keepInRoot) {
                Move-Item $_.FullName -Destination "docs/archive/" -Force -ErrorAction SilentlyContinue
                Write-Host "  → $($_.Name) moved to docs/archive/" -ForegroundColor $Cyan
            } else {
                Write-Host "  ✓ Keeping $($_.Name) in root" -ForegroundColor $Green
            }
        }

        $testPatterns = @("test_*.py", "*_test.py", "*.test.js", "*.test.ts", "*.spec.js", "*.spec.ts")
        foreach ($pattern in $testPatterns) {
            Get-ChildItem -Filter $pattern -File -ErrorAction SilentlyContinue | ForEach-Object {
                Move-Item $_.FullName -Destination "tests/" -Force -ErrorAction SilentlyContinue
                Write-Host "  → $($_.Name) moved to tests/" -ForegroundColor $Cyan
            }
        }
        Write-Host "✅ Files organized" -ForegroundColor $Green
    }

    if ($OrganizeFiles) { Invoke-OrganizeFiles }

    $hasFiles = (Get-ChildItem -File -ErrorAction SilentlyContinue | Where-Object { $_.Name -notlike ".*" } | Measure-Object).Count -gt 0
    if ($hasFiles -and -not $AutoMode) {
        Write-Host "⚠️  Current directory is not empty." -ForegroundColor $Yellow
        Write-Host ""
        Write-Host "Options:"
        Write-Host "  1) Continue and merge files"
        Write-Host "  2) Organize existing files first"
        Write-Host "  3) Cancel"
        Write-Host "  4) 🧠 Analyze Architecture & Generate Copilot Prompt"
        Write-Host ""
        $choice = Read-Host "Select (1/2/3/4)"

        switch ($choice) {
            "1" { Write-Host "Continuing..." }
            "2" { Invoke-OrganizeFiles }
            "3" { Write-Host "Cancelled."; exit 0 }
            "4" {
                 if (Test-Path "scripts/analyze-architecture.ps1") {
                     & "scripts/analyze-architecture.ps1"
                     exit 0
                 } else {
                     Write-Host "❌ Script not found." -ForegroundColor $Red
                     exit 1
                 }
            }
            default { Write-Host "Invalid option."; exit 1 }
        }
    }

    $InstallBinaries = $true
    $GlobalCliDetected = Test-CliInstalled
    if (-not $AutoMode -and -not $NoBinaries) {
        Write-Host ""
        Write-Host "📦 Components:" -ForegroundColor $Cyan
        if ($GlobalCliDetected) {
            Write-Host "   ✨ Global Git-Core CLI detected in PATH." -ForegroundColor $Green
            $binChoice = Read-Host "   Skip local binaries? (Y/n)"
            if ($binChoice -match "^[nN]") {
                $InstallBinaries = $true
                Write-Host "   → Downloading local binaries." -ForegroundColor $Green
            } else {
                $InstallBinaries = $false
                Write-Host "   → Using global CLI." -ForegroundColor $Yellow
            }
        } else {
            $binChoice = Read-Host "   Install binaries? (Y/n)"
            if ($binChoice -match "^[nN]") {
                $InstallBinaries = $false
                Write-Host "   → Skipping binaries." -ForegroundColor $Yellow
            } else {
                $InstallBinaries = $true
                Write-Host "   → Installing binaries." -ForegroundColor $Green
            }
        }
    } elseif ($NoBinaries) {
        $InstallBinaries = $false
    }

    if ($UpgradeMode) { Backup-UserFiles }

    Write-Host "`n📥 Downloading Git-Core Protocol..." -ForegroundColor $Cyan
    git clone --depth 1 $REPO_URL $TEMP_DIR
    if ($LASTEXITCODE -ne 0) { throw "Error cloning repository" }
    Remove-Item -Recurse -Force "$TEMP_DIR/.git" -ErrorAction SilentlyContinue

    Write-Host "📦 Installing protocol files..." -ForegroundColor $Cyan
    Invoke-Migration | Out-Null

    $templateAiDir = if (Test-Path "$TEMP_DIR/.gitcore") { "$TEMP_DIR/.gitcore" }  elseif (Test-Path "$TEMP_DIR/.ai") { "$TEMP_DIR/.ai" } else { $null }
    if ($templateAiDir) {
        if ($UpgradeMode) {
            @(".gitcore", ".ai") | ForEach-Object { if (Test-Path $_) { Remove-Item -Recurse -Force $_ } }
            New-Item -ItemType Directory -Force -Path ".gitcore" | Out-Null
            Copy-Item -Recurse "$templateAiDir/*" ".gitcore/"
            Write-Host "  ✓ .gitcore/ (upgraded)" -ForegroundColor $Green
        } elseif (-not (Test-Path ".gitcore") -and -not (Test-Path ".ai")) {
            New-Item -ItemType Directory -Force -Path ".gitcore" | Out-Null
            Copy-Item -Recurse "$templateAiDir/*" ".gitcore/"
            Write-Host "  ✓ .gitcore/" -ForegroundColor $Green
        } else {
            $existingDir = if (Test-Path ".gitcore") { ".gitcore" }  else { ".ai" }
            Write-Host "  ~ $existingDir/ (exists, merging new files)" -ForegroundColor $Yellow
            Get-ChildItem $templateAiDir | ForEach-Object {
                if (-not (Test-Path "$existingDir/$($_.Name)")) {
                    Copy-Item $_.FullName "$existingDir/"
                    Write-Host "    + $($_.Name)" -ForegroundColor $Green
                }
            }
        }
    }

    $InternalWorkflows = @("build-tools.yml", "release.yml", "protocol-propagation.yml")
    $dirs = @(".github", "scripts", "docs")
    if ($InstallBinaries) { $dirs += "bin" }

    foreach ($dir in $dirs) {
        if (Test-Path "$TEMP_DIR/$dir") {
            if ($UpgradeMode) {
                if (Test-Path $dir) { Remove-Item -Recurse -Force $dir }
                Copy-Item -Recurse "$TEMP_DIR/$dir" .
                if ($dir -eq ".github") {
                    $InternalWorkflows | ForEach-Object { if (Test-Path ".github/workflows/$_") { Remove-Item ".github/workflows/$_" -Force } }
                }
                if ($dir -eq "scripts") {
                    @("bump-version.ps1", "bump-version.sh") | ForEach-Object { if (Test-Path "scripts/$_") { Remove-Item "scripts/$_" -Force } }
                }
                Write-Host "  ✓ $dir/ (upgraded)" -ForegroundColor $Green
            } elseif (-not (Test-Path $dir)) {
                Copy-Item -Recurse "$TEMP_DIR/$dir" .
                if ($dir -eq ".github") {
                    $InternalWorkflows | ForEach-Object { if (Test-Path ".github/workflows/$_") { Remove-Item ".github/workflows/$_" -Force } }
                }
                if ($dir -eq "scripts") {
                    @("bump-version.ps1", "bump-version.sh") | ForEach-Object { if (Test-Path "scripts/$_") { Remove-Item "scripts/$_" -Force } }
                }
                Write-Host "  ✓ $dir/" -ForegroundColor $Green
            } else {
                Copy-Item -Recurse -Force "$TEMP_DIR/$dir/*" $dir -ErrorAction SilentlyContinue
                if ($dir -eq ".github") {
                    $InternalWorkflows | ForEach-Object { if (Test-Path ".github/workflows/$_") { Remove-Item ".github/workflows/$_" -Force } }
                }
                if ($dir -eq "scripts") {
                    @("bump-version.ps1", "bump-version.sh") | ForEach-Object { if (Test-Path "scripts/$_") { Remove-Item "scripts/$_" -Force } }
                }
                Write-Host "  ✓ $dir/ (merged)" -ForegroundColor $Green
            }
        }
    }

    $protocolFiles = @(".cursorrules", ".windsurfrules", "AGENTS.md", ".git-core-protocol-version")
    foreach ($file in $protocolFiles) {
        if (Test-Path "$TEMP_DIR/$file") {
            if ($UpgradeMode) {
                Copy-Item -Force "$TEMP_DIR/$file" .
                Write-Host "  ✓ $file (upgraded)" -ForegroundColor $Green
            } elseif (-not (Test-Path $file)) {
                Copy-Item "$TEMP_DIR/$file" .
                Write-Host "  ✓ $file" -ForegroundColor $Green
            } else {
                Write-Host "  ~ $file (exists)" -ForegroundColor $Yellow
            }
        }
    }

    if (Test-Path ".agent/rules") {
        Write-Host "`n🔮 Detected Antigravity IDE configuration" -ForegroundColor Magenta
        $ruleFile = Get-ChildItem ".agent/rules/rule-*.md" -ErrorAction SilentlyContinue | Select-Object -First 1
        if ($ruleFile -and (Get-Content $ruleFile.FullName -Raw) -notmatch "Git-Core Protocol Integration") {
            Add-Content -Path $ruleFile.FullName -Value "`n---`n`n## 🔗 Git-Core Protocol Integration`n`n> This project follows the Git-Core Protocol. See root-level files for full configuration.`n"
            Write-Host "  ✓ Updated $($ruleFile.Name) with protocol integration" -ForegroundColor $Green
        }
    }

    $preserveFiles = @(".gitignore", "README.md")
    foreach ($file in $preserveFiles) {
        if ((Test-Path "$TEMP_DIR/$file") -and -not (Test-Path $file)) {
            Copy-Item "$TEMP_DIR/$file" .
            Write-Host "  ✓ $file" -ForegroundColor $Green
        } elseif (Test-Path $file) {
            Write-Host "  ~ $file (preserved)" -ForegroundColor $Yellow
        }
    }

    if ($UpgradeMode) { Restore-UserFiles }

    $NewVersion = Get-CurrentVersion
    Write-Host "`n========================================" -ForegroundColor $Green
    Write-Host "✅ Git-Core Protocol v$NewVersion installed" -ForegroundColor $Green
    Write-Host "========================================`n" -ForegroundColor $Green

    if ($UpgradeMode) {
        Write-Host "📋 Upgraded from v$CurrentVersion → v$NewVersion" -ForegroundColor $Cyan
    } else {
        Write-Host "📋 Files installed:"
        Write-Host "   .gitcore/ARCHITECTURE.md    - Document your architecture here"
        Write-Host "   .github/               - Copilot rules + workflows"
        Write-Host "   scripts/               - Init and update scripts"
        Write-Host "   AGENTS.md              - Rules for all AI agents"
    }

    # Migrate mode: auto-commit changes
    if ($MigrateMode) {
        Write-Host ""
        Write-Host "📦 Committing changes..." -ForegroundColor $Cyan
        git add -A
        $commitMessage = @"
chore: install Git-Core Protocol v$NewVersion

- Installed protocol files and structure
- Added .gitcore/, .github/, scripts/, docs/
- Branch: $BranchName

Installed via: `$env:GIT_CORE_MIGRATE = "1"; irm .../install.ps1 | iex
"@
        git commit -m $commitMessage

        Write-Host ""
        Write-Host "✅ Changes committed to branch: $BranchName" -ForegroundColor $Green
        Write-Host "🚀 Next steps:" -ForegroundColor $Yellow
        Write-Host "   1. Review changes: git diff main...$BranchName"
        Write-Host "   2. Push branch:    git push -u origin $BranchName"
        Write-Host "   3. Create PR to merge into main"
    } else {
        Write-Host "`n🚀 Next step:" -ForegroundColor $Yellow
        Write-Host "   .\scripts\init_project.ps1`n"
    }

    Write-Host ""
    Write-Host "💡 Commands:" -ForegroundColor $Cyan
    Write-Host "   Safe upgrade:  `$env:GIT_CORE_UPGRADE = '1'; irm .../install.ps1 | iex"
    Write-Host "   Full reset:    `$env:GIT_CORE_FORCE = '1'; irm .../install.ps1 | iex"
    Write-Host "   Auto-migrate:  `$env:GIT_CORE_MIGRATE = '1'; irm .../install.ps1 | iex"
} catch {
    Write-Host "`n❌ Installation failed: $($_.Exception.Message)" -ForegroundColor $Red
    exit 1
} finally {
    if (Test-Path $TEMP_DIR) { Remove-Item -Recurse -Force $TEMP_DIR -ErrorAction SilentlyContinue }
}

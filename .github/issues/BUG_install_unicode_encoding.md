---
title: "BUG: install.ps1 fails when downloaded remotely due to Unicode encoding (.ai-core)"
labels:
  - bug
  - critical
  - cli
assignees: []
---

## Description
When running the remote installer via `irm ... | iex`, the PowerShell parser fails with syntax errors because the `.ai-core` directory name contains a Unicode emoji that gets corrupted during the HTTP download/piped execution.

## Error Message
```
iex : En línea: 274 Carácter: 66
+      } elseif (-not (Test-Path ".ai-core") -and -not (Test-Path ".ai")) {
Falta la llave de cierre "}" en el bloque de instrucciones
```

## Root Cause
PowerShell's `Invoke-RestMethod` or `Invoke-WebRequest` may not preserve UTF-8 encoding correctly when piping directly to `Invoke-Expression`.

## Proposed Solutions

### Option A: Replace .ai-core with ASCII-safe name
Replace `.ai-core` with `.ai-core` or similar ASCII-only directory name across the entire protocol.

### Option B: Encode-safe remote execution
Change the installer to download the script to a temp file with correct encoding first, then execute:
```powershell
$script = Invoke-WebRequest -Uri "..." -UseBasicParsing
[System.IO.File]::WriteAllText("$env:TEMP\install.ps1", $script.Content, [System.Text.Encoding]::UTF8)
& "$env:TEMP\install.ps1"
```

### Option C: Escape Unicode in script
Use PowerShell's escaped Unicode syntax: `".`u{2728}"` instead of literal `.ai-core`.

## Recommendation
Option A is the safest and most portable. Unicode in directory names can cause issues across different systems and shells.

## Tasks
- [ ] Decide on replacement directory name
- [ ] Update all references in install.ps1
- [ ] Update all references in CLI (Rust)
- [ ] Update documentation
- [ ] Test remote installation on fresh system

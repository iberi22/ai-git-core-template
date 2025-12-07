---
title: "Create Rust Issue Syncer Tool for 10-20x Performance Improvement"
labels:
  - enhancement
  - performance
  - rust
  - developer-experience
  - ai-plan
assignees: []
---

## 🎯 Objetivo

Crear nueva tool en Rust `tools/issue-syncer` para reemplazar `scripts/sync-issues.ps1` (317 líneas) y mejorar drasticamente la experiencia de desarrollo.

## 📊 Impacto

- **Prioridad:** 🔥 ALTA (80% impacto)
- **Frecuencia:** Usado constantemente por developers + CI en cada push
- **Performance:** 10-20x speedup (5-10s → <500ms)
- **Ahorro estimado:** ~285 segundos/día (~2.4 horas/mes)
- **UX:** Developer tool crítico para workflow local

## 🔍 Análisis Actual

**Workflow afectado:** `.github/workflows/sync-issues.yml`

**Script actual:** `scripts/sync-issues.ps1`
- Sync bidireccional: `.github/issues/*.md` ↔ GitHub Issues
- YAML frontmatter parsing
- JSON mapping (.issue-mapping.json)
- File system watching (modo watch)
- Auto-cleanup de issues cerrados

**Cuellos de botella:**
- PowerShell file I/O lento
- YAML parsing manual
- Múltiples llamadas `gh` secuenciales
- Watch mode consume recursos
- No async operations

## 🏗️ Implementación Propuesta

### Fase 1: Estructura Base

**Ubicación:** `tools/issue-syncer/`

**Arquitectura:**
```
tools/issue-syncer/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── syncer.rs         # Core sync logic
│   ├── parser.rs         # YAML frontmatter parser
│   ├── watcher.rs        # File system watcher
│   ├── github.rs         # GitHub API wrapper
│   └── mapping.rs        # .issue-mapping.json handler
├── Cargo.toml
└── README.md
```

**Funciones principales:**
```rust
pub struct IssueSyncer {
    github_client: Octocrab,
    issues_dir: PathBuf,
    mapping: IssueMapping,
}

impl IssueSyncer {
    pub async fn sync_all(&mut self) -> Result<SyncReport> {
        self.push().await?;
        self.pull().await?;
        Ok(self.generate_report())
    }
    
    pub async fn push(&self) -> Result<Vec<Created>> {
        // .md files -> GitHub Issues
        let files = self.scan_issue_files()?;
        
        for file in files {
            let issue_data = self.parse_frontmatter(&file)?;
            
            if let Some(issue_number) = self.mapping.get(&file) {
                self.update_issue(issue_number, issue_data).await?;
            } else {
                let number = self.create_issue(issue_data).await?;
                self.mapping.add(&file, number)?;
            }
        }
        
        Ok(created)
    }
    
    pub async fn pull(&mut self) -> Result<Vec<Deleted>> {
        // GitHub Issues -> Delete closed .md files
        let closed = self.fetch_closed_issues().await?;
        
        for issue in closed {
            if let Some(file) = self.mapping.get_file(issue.number) {
                fs::remove_file(&file)?;
                self.mapping.remove(issue.number)?;
            }
        }
        
        Ok(deleted)
    }
    
    pub async fn watch(&self) -> Result<()> {
        // File system watcher con debouncing
        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(tx)?;
        
        watcher.watch(&self.issues_dir, RecursiveMode::NonRecursive)?;
        
        loop {
            match rx.recv() {
                Ok(Ok(Event { kind: EventKind::Modify(_), paths, .. })) => {
                    self.handle_file_change(paths).await?;
                }
                _ => {}
            }
        }
    }
}
```

### Fase 2: CLI Commands

```bash
# One-time sync (both directions)
issue-syncer sync

# Push local files to GitHub
issue-syncer push

# Pull and cleanup closed issues
issue-syncer pull

# Watch mode (file system monitoring)
issue-syncer watch

# Dry run
issue-syncer sync --dry-run

# Specific repo
issue-syncer sync --repo owner/repo
```

### Fase 3: Workflow Update

**Cambio en `.github/workflows/sync-issues.yml`:**
```yaml
- name: 📋 Sync Issues
  run: |
    if command -v issue-syncer &> /dev/null; then
      issue-syncer sync
    else
      # Fallback to PowerShell
      pwsh ./scripts/sync-issues.ps1
    fi
```

### Fase 4: Local Development

**Script wrapper para compatibilidad:**
```bash
# scripts/sync-issues.sh (nuevo)
#!/bin/bash
if command -v issue-syncer &> /dev/null; then
    issue-syncer "$@"
else
    pwsh ./scripts/sync-issues.ps1 "$@"
fi
```

## 📦 Dependencies

```toml
[dependencies]
octocrab = "0.38"                     # GitHub API
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"                    # YAML frontmatter
notify = "6"                          # File system watcher
clap = { version = "4", features = ["derive"] }
anyhow = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
```

## ✅ Criterios de Aceptación

- [ ] Tool `issue-syncer` creada en `tools/`
- [ ] Commands: sync, push, pull, watch
- [ ] YAML frontmatter parsing completo
- [ ] Sync bidireccional funcional
- [ ] File system watcher con debouncing
- [ ] Mapping persistence en .issue-mapping.json
- [ ] Tests unitarios >80% coverage
- [ ] Integration tests con mock GitHub
- [ ] Benchmarks >10x faster que PowerShell
- [ ] Workflow actualizado
- [ ] Instalador cross-platform
- [ ] Documentación completa

## 📝 Tareas

### Core Implementation
- [ ] Setup proyecto `tools/issue-syncer/`
- [ ] Implementar `parser.rs` para YAML frontmatter
- [ ] Implementar `mapping.rs` para JSON persistence
- [ ] Implementar `github.rs` wrapper sobre octocrab
- [ ] Implementar `syncer.rs` core logic
- [ ] Implementar `watcher.rs` file system monitoring

### CLI
- [ ] Setup clap CLI con subcommands
- [ ] Command `sync` (push + pull)
- [ ] Command `push` (files -> GitHub)
- [ ] Command `pull` (cleanup closed)
- [ ] Command `watch` (monitor files)
- [ ] Flag `--dry-run`
- [ ] Flag `--repo`

### Testing & Integration
- [ ] Unit tests para cada módulo
- [ ] Integration tests con mock API
- [ ] Benchmarks comparativos
- [ ] Actualizar workflow con fallback
- [ ] Cross-platform testing (Linux, macOS, Windows)

### Distribution
- [ ] Build script para releases
- [ ] Instalador cross-platform
- [ ] Actualizar install.sh/install.ps1
- [ ] Documentación en README

## 🔗 Referencias

- PowerShell original: `scripts/sync-issues.ps1`
- Workflow actual: `.github/workflows/sync-issues.yml`
- Issues directory: `.github/issues/`
- Mapping file: `.github/issues/.issue-mapping.json`

## 🎯 Roadmap

**Sprint 1 (Semana 1-2):**
- Setup proyecto y core implementation
- Parser + Syncer + GitHub wrapper

**Sprint 2 (Semana 3):**
- CLI commands + Watcher
- Testing local

**Sprint 3 (Semana 4):**
- Integration tests + Workflow update
- Cross-platform testing

**Sprint 4 (Semana 5):**
- Production cutover
- Monitoreo

---

**AI-Context:** Candidato #3 para migración a Rust. Herramienta crítica de developer experience usada constantemente. Nueva tool independiente (no módulo de orchestrator) porque tiene un caso de uso standalone.

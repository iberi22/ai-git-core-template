---
title: "Reorganizar archivos en docs/agent-docs/ según convenciones"
labels:
  - jules
  - documentation
  - good-first-issue
assignees: []
---

## 🎯 Objetivo

Reorganizar los archivos en `docs/agent-docs/` que están en la raíz y moverlos a sus subcarpetas correspondientes según las convenciones del README.md.

## 📋 Plan de Reorganización

### Archivos a Mover

#### 1. Especificaciones → `specs/`
```bash
git mv docs/agent-docs/CONTEXT_PROTOCOL.md docs/agent-docs/specs/SPEC_CONTEXT_PROTOCOL.md
git mv docs/agent-docs/EVOLUTION_PROTOCOL.md docs/agent-docs/specs/SPEC_EVOLUTION_PROTOCOL.md
git mv docs/agent-docs/HUMAN_LAYER_PROTOCOL.md docs/agent-docs/specs/SPEC_HUMAN_LAYER_PROTOCOL.md
git mv docs/agent-docs/MICRO_AGENTS.md docs/agent-docs/specs/SPEC_MICRO_AGENTS.md
```

#### 2. Análisis → `analysis/`
```bash
# Ya están con el prefijo correcto, solo mover si están en raíz
# ANALYSIS_TELEMETRY_SCALE.md
# ANALYSIS_WORKFLOW_RUST_MIGRATION.md
```

#### 3. Reportes → `reports/`
```bash
# Ya están con el prefijo correcto
# REPORT_GLOBAL_SELFHEALING_DIAGNOSIS.md
# REPORT_PROJECT_AUDIT.md
# REPORT_WORKFLOW_OPTIMIZATION.md
```

#### 4. Investigación → `research/`
```bash
# Ya están con el prefijo correcto
# RESEARCH_LLM_TOOL_CALLING.md
# RESEARCH_SELFHEALING_CICD.md
# RESEARCH_STACK_CONTEXT.md
```

#### 5. Prompts → `prompts/`
```bash
# Ya está con el prefijo correcto
# PROMPT_PROTOCOL_INSTALLER.md
```

## ✅ Criterios de Aceptación

1. **Todos los archivos deben estar en sus subcarpetas correspondientes:**
   - `specs/` - Especificaciones de protocolo (SPEC_*.md)
   - `analysis/` - Análisis técnicos (ANALYSIS_*.md)
   - `reports/` - Reportes de auditoría (REPORT_*.md)
   - `research/` - Investigaciones (RESEARCH_*.md)
   - `prompts/` - Prompts reutilizables (PROMPT_*.md)

2. **Naming conventions aplicadas:**
   - Archivos de protocolo deben tener prefijo `SPEC_`
   - Otros prefijos deben mantenerse según el tipo

3. **Crear README.md en cada subcarpeta** con índice de documentos:
   - `specs/README.md` - Índice de especificaciones
   - `analysis/README.md` - Índice de análisis
   - `reports/README.md` - Índice de reportes
   - `research/README.md` - Índice de investigaciones
   - `prompts/README.md` - Catálogo de prompts

4. **Actualizar el README.md principal** (`docs/agent-docs/README.md`):
   - Reflejar la nueva estructura
   - Incluir tabla de documentos organizados por categoría
   - Agregar sección "Cómo Navegar Este Directorio"

## 📝 Formato de README por Subcarpeta

Cada subcarpeta debe tener un README.md con esta estructura:

```markdown
# [Categoría] - Índice

> Descripción breve de qué contiene esta carpeta

## 📋 Documentos Disponibles

| Documento | Descripción | Fecha | Tags |
|-----------|-------------|-------|------|
| [NOMBRE.md](./NOMBRE.md) | Descripción breve | YYYY-MM-DD | #tag1 #tag2 |

## 🔍 Buscar por Tags

- **#tag1** - Descripción
- **#tag2** - Descripción

## 📚 Documentos Relacionados

- [Otro doc](../otra-carpeta/doc.md)
```

## 🎯 Ejemplo de README.md Principal Actualizado

El `docs/agent-docs/README.md` debe incluir una sección como esta:

```markdown
## 📚 Navegación Rápida

### Por Tipo de Documento

- **📐 Especificaciones** - [specs/README.md](./specs/README.md)
  - SPEC_CONTEXT_PROTOCOL.md - Protocolo de contexto stateless
  - SPEC_EVOLUTION_PROTOCOL.md - Sistema de evolución del protocolo
  - SPEC_HUMAN_LAYER_PROTOCOL.md - Human-in-the-loop para operaciones críticas
  - SPEC_MICRO_AGENTS.md - Sistema de personas por rol
  - SPEC_NON_BLOCKING_EXECUTION.md - Ejecución no bloqueante

- **📊 Análisis** - [analysis/README.md](./analysis/README.md)
  - ANALYSIS_TELEMETRY_SCALE.md - Escalabilidad de telemetría
  - ANALYSIS_WORKFLOW_RUST_MIGRATION.md - Migración a Rust

- **📈 Reportes** - [reports/README.md](./reports/README.md)
  - REPORT_GLOBAL_SELFHEALING_DIAGNOSIS.md - Diagnóstico de auto-sanación
  - REPORT_PROJECT_AUDIT.md - Auditoría de proyecto
  - REPORT_WORKFLOW_OPTIMIZATION.md - Optimización de workflows

- **🔬 Investigación** - [research/README.md](./research/README.md)
  - RESEARCH_LLM_TOOL_CALLING.md - Tool calling en LLMs
  - RESEARCH_SELFHEALING_CICD.md - CI/CD auto-sanador
  - RESEARCH_STACK_CONTEXT.md - Contexto de stack tecnológico

- **💡 Prompts** - [prompts/README.md](./prompts/README.md)
  - PROMPT_PROTOCOL_INSTALLER.md - Instalación de protocolo
```

## 🚀 Flujo de Trabajo

1. **Ejecutar comandos `git mv`** para mover archivos preservando historial Git
2. **Crear README.md** en cada subcarpeta con índice
3. **Actualizar README.md principal** con navegación completa
4. **Verificar enlaces rotos** - Actualizar referencias en otros archivos si existen
5. **Commit atómico** con mensaje: `docs(agent): organize agent-docs structure #<ISSUE_NUMBER>`

## 🔍 Verificación

Después de completar:

```powershell
# Verificar estructura
tree /F docs\agent-docs

# Verificar que no hay archivos sueltos en raíz (excepto README.md y .gitkeep)
Get-ChildItem docs\agent-docs\*.md | Where-Object { $_.Name -ne "README.md" }
# Debe retornar vacío

# Verificar que cada subcarpeta tiene README.md
Get-ChildItem docs\agent-docs -Directory | ForEach-Object {
    Test-Path (Join-Path $_.FullName "README.md")
}
# Todos deben retornar True
```

## 📚 Referencias

- Convenciones de nombres: `docs/agent-docs/README.md`
- Protocolo de documentación: `AGENTS.md` sección "User-Requested Documentation"
- YAML frontmatter: `docs/agent-docs/README.md` sección "Meta Tags"

## 💡 Notas para Jules

- **Preservar historial Git**: Usar `git mv` en lugar de mover manualmente
- **Naming conventions**: Archivos de protocolo deben tener prefijo `SPEC_`
- **YAML frontmatter**: Mantener intacto en cada archivo
- **Enlaces**: Verificar si hay referencias a estos archivos en otros documentos
- **Commit atómico**: Un solo commit con todos los cambios para facilitar revisión

## ⚙️ Contexto Técnico

**Stack:**
- PowerShell (Windows) / Bash (Linux/macOS)
- Git para versionado
- Markdown para documentación

**Ubicación:**
- Carpeta: `docs/agent-docs/`
- Archivos afectados: ~13 archivos .md
- Subcarpetas: specs/, analysis/, reports/, research/, prompts/

**Prioridad:** Medium
**Estimación:** 30-45 minutos
**Dificultad:** Low (tareas repetitivas pero claras)

---

**¡Gracias Jules! 🙌 Este issue mejorará significativamente la navegabilidad de la documentación del protocolo.**

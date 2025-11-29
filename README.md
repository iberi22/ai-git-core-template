# 🧠 Git-Core Protocol

> Template para gestión de proyectos con AI Agents. Agnóstico de lenguaje, cero alucinaciones de estado, ahorro brutal de tokens.

[![Use this template](https://img.shields.io/badge/Use%20this-template-blue?style=for-the-badge)](https://github.com/iberi22/ai-git-core-template/generate)

## 🤔 ¿Por qué este enfoque?

| Problema | Solución Git-Core |
|----------|------------------|
| AI "olvida" el estado de tareas | Estado en GitHub Issues (persistente) |
| Contexto crece = más tokens = más costo | Solo cargar issue actual + arquitectura |
| Archivos TODO.md desordenados | Tablero GitHub organizado |
| Dependencia de ecosistema (NPM, etc.) | Scripts bash/PowerShell agnósticos |

## 🚀 Quick Start

### Opción 1: Usar como Template

1. Click en **"Use this template"** arriba
2. Clona tu nuevo repositorio
3. Ejecuta el script de inicialización:

```bash
# Linux/Mac
./scripts/init_project.sh

# Windows PowerShell
.\scripts\init_project.ps1
```

### Opción 2: Clonar en proyecto existente

```bash
# Crear carpeta de proyecto
mkdir mi-proyecto && cd mi-proyecto

# Clonar template (sin historial git)
git clone https://github.com/iberi22/ai-git-core-template . && rm -rf .git

# Inicializar
./scripts/init_project.sh
```

## 📂 Estructura

```
/
├── .ai/
│   ├── ARCHITECTURE.md       # 📖 Contexto del sistema
│   └── CONTEXT_LOG.md        # 📝 Notas efímeras de sesión
├── .github/
│   ├── copilot-instructions.md  # 🤖 Reglas para GitHub Copilot
│   └── ISSUE_TEMPLATE/       # 📋 Templates de issues
├── scripts/
│   ├── init_project.sh       # 🐧 Inicializador Linux/Mac
│   └── init_project.ps1      # 🪟 Inicializador Windows
├── AGENTS.md                 # 🤖 Configuración para todos los AI agents
├── .cursorrules              # 🎯 Reglas para Cursor
├── .windsurfrules            # 🌊 Reglas para Windsurf
└── .gitignore
```

## 🔄 El Flujo de Trabajo

```
┌─────────────────────────────────────────────────────────┐
│                    THE LOOP                              │
├─────────────────────────────────────────────────────────┤
│                                                          │
│   1. READ                                                │
│      ├── cat .ai/ARCHITECTURE.md                        │
│      └── gh issue list --assignee "@me"                 │
│                                                          │
│   2. ACT                                                 │
│      ├── gh issue edit <id> --add-assignee "@me"        │
│      ├── git checkout -b feat/issue-<id>                │
│      └── [write code + tests]                           │
│                                                          │
│   3. UPDATE                                              │
│      ├── git commit -m "feat: ... (closes #<id>)"       │
│      └── gh pr create --fill                            │
│                                                          │
│   ↺ Repeat                                               │
└─────────────────────────────────────────────────────────┘
```

## 🏷️ Etiquetas Semánticas

El script crea automáticamente:

| Label | Uso |
|-------|-----|
| `ai-plan` | Tareas de planificación |
| `ai-context` | Información crítica |
| `ai-blocked` | Requiere intervención humana |
| `in-progress` | Tarea en desarrollo |
| `needs-review` | Requiere revisión |

## 📋 Requisitos

- [Git](https://git-scm.com/)
- [GitHub CLI](https://cli.github.com/) (`gh`) - autenticado

## 🤖 Compatibilidad con AI Agents

- ✅ GitHub Copilot
- ✅ Cursor
- ✅ Windsurf
- ✅ Claude
- ✅ ChatGPT (con Code Interpreter)
- ✅ Cualquier LLM con acceso a terminal

## 📄 Licencia

MIT - Usa esto como quieras.

---

**Creado con 🧠 por [@iberi22](https://github.com/iberi22)**

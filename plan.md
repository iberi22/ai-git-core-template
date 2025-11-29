Para un perfil de **Arquitecto Senior**, la mejor opción es crear un **GitHub Template Repository** que incluya un **Script de Inicialización (Bootstrapping)**.

¿Por qué no un paquete NPM? Porque NPM te ata al ecosistema Node.js. Si quieres usar esto para un proyecto en Python, Go o Rust, tener un `package.json` y `node_modules` solo para la gestión del proyecto es "basura" innecesaria.

¿Por qué no solo un Prompt? Porque el prompt no puede crear la estructura de carpetas, configurar `.gitignore` ni validar que tengas la CLI de GitHub instalada.

Aquí tienes la **Especifiación del Repositorio Plantilla (The "Git-Core Protocol")**.

---

### 📂 Estructura del Repositorio Plantilla (`ai-git-core-template`)

Este sería el contenido de tu repositorio. Puedes clonarlo o usar "Use this template".

```text
/
├── .ai/
│   ├── ARCHITECTURE.md       # Plantilla vacía con secciones clave
│   └── CONTEXT_LOG.md        # (Opcional) Solo para notas efímeras de sesión
├── .github/
│   └── ISSUE_TEMPLATE/       # Plantillas para que la IA cree issues estructurados
│       ├── task.md
│       └── bug.md
├── scripts/
│   └── init_project.sh       # ⚡ El cerebro mágico (ver abajo)
├── .cursorrules              # (O .windsurfrules) Las Reglas Globales
├── .gitignore
└── README.md
```

---

### 1. El Cerebro: `scripts/init_project.sh`

Este script reemplaza al "Prompt Inicial". Automatiza la creación del entorno "Git-Core".
*Requisitos: `git` y `gh` (GitHub CLI) instalados.*

```bash
#!/bin/bash
# scripts/init_project.sh

echo "🧠 Inicializando Protocolo AI Git-Core..."

# 1. Validar entorno
if ! command -v gh &> /dev/null; then
    echo "❌ Error: GitHub CLI (gh) no está instalado."
    exit 1
fi

# 2. Configurar el Repo
echo "Configurando repositorio..."
git init
gh repo create "$(basename "$PWD")" --private --source=. --remote=origin --push

# 3. Definir Arquitectura Básica (Si está vacía)
if [ ! -s .ai/ARCHITECTURE.md ]; then
    echo "# Architecture" > .ai/ARCHITECTURE.md
    echo "Stack: TBD" >> .ai/ARCHITECTURE.md
fi

# 4. Crear Etiquetas (Labels) Semánticas para la IA
echo "Creando etiquetas de gestión..."
gh label create "ai-plan" --description "Tareas de planificación de alto nivel" --color "0E8A16"
gh label create "ai-context" --description "Información crítica para el contexto" --color "FBCA04"

# 5. GENERAR ISSUES INICIALES (Aquí está la magia)
# Esto crea el backlog inicial sin gastar tokens de contexto en el chat
echo "Creando issues iniciales..."

gh issue create --title "SETUP: Definir Arquitectura y Stack" \
                --body "Tarea: Llenar .ai/ARCHITECTURE.md con las decisiones técnicas." \
                --label "ai-plan"

gh issue create --title "INFRA: Configuración inicial del entorno" \
                --body "Tarea: Configurar linters, docker y estructura base." \
                --label "ai-plan"

echo "✅ Proyecto inicializado. Tu Agente AI está listo para leer issues."
```

---

### 2. Las Reglas: `.cursorrules` (o System Prompt)

Este archivo asegura que el Agente respete el protocolo y ahorre tokens.

```markdown
# 🧠 AI Git-Core Protocol Rules

## 0. Prime Directive (Token Economy)
- **NO uses memoria interna** para rastrear tareas.
- **NO crees archivos TODO.md** o TASK.md.
- Tu estado es **GitHub Issues**.

## 1. Flujo de Trabajo (The Loop)
Al iniciar cualquier tarea, sigue estos pasos estrictamente:

1.  **READ (Contexto):**
    - Lee `.ai/ARCHITECTURE.md` para entender el sistema.
    - Ejecuta: `gh issue list --assignee "@me"` para ver tu tarea actual.
    - Si no tienes tarea, busca en el backlog: `gh issue list --limit 5`.

2.  **ACT (Desarrollo):**
    - Si tomas una tarea nueva: `gh issue edit <id> --add-assignee "@me"`.
    - Crea una rama: `git checkout -b feat/issue-<id>`.
    - Escribe código y tests.

3.  **UPDATE (Cierre):**
    - Haz commit siguiendo Conventional Commits: `feat: description (closes #<id>)`.
    - Push y PR: `gh pr create --fill`.
    - **IMPORTANTE:** No actualices ningún archivo de texto para marcar "check". Git lo hace por ti.

## 2. Planificación
- Si se te pide planear, NO escribas un documento largo.
- Genera comandos para crear los issues:
  `gh issue create --title "..." --body "..."`
```

---

### 3. Cómo usarlo (La Experiencia del Usuario)

Imagina que eres tú o un desarrollador de tu equipo.

1. **Crear carpeta:** `mkdir mi-nuevo-saas && cd mi-nuevo-saas`
2. **Instalar Template:**
    * Opción A (Git): `git clone https://github.com/tu-usuario/ai-git-core-template . && rm -rf .git`
    * Opción B (Script remoto - Nivel Dios):
        `curl -sL https://raw.githubusercontent.com/tu-usuario/ai-git-core-template/main/install.sh | bash`
3. **Ejecutar Init:** `./scripts/init_project.sh`
    * *El script crea el repo en GitHub, crea los labels y los primeros 3 tickets.*
4. **Abrir Editor AI (Cursor/Windsurf):**
    * Escribes: *"Hola, empieza con el primer issue asignado."*
5. **Resultado:**
    * El agente lee las reglas.
    * Ejecuta `gh issue list`.
    * Ve "SETUP: Definir Arquitectura".
    * Empieza a trabajar.

### Ventajas de este enfoque

1. **Agnóstico:** Funciona para Python, JS, Rust, PHP, etc.
2. **Cero Alucinaciones de Estado:** La IA no puede "pensar" que terminó una tarea si el Issue sigue abierto en GitHub.
3. **Ahorro Brutal de Tokens:** En lugar de alimentar al LLM con un `TASK.md` de 500 líneas con el historial de todo el proyecto, solo le das el contexto del Issue actual + Arquitectura.
4. **Profesional:** Si entra un humano al proyecto, ve un tablero de GitHub organizado, no archivos de texto desordenados.

### ¿Qué deberías hacer ahora?

1. Crea el repo `ai-git-core-template` en tu GitHub.
2. Sube el archivo `.cursorrules` y la carpeta `scripts/`.
3. Hazlo público.
4. En tus proyectos futuros, solo clonas y ejecutas el script. Es tu propio framework de desarrollo asistido por IA.

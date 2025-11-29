# 🧠 GitHub Copilot Instructions

## Prime Directive
You are operating under the **Git-Core Protocol**. Your state is GitHub Issues, not internal memory.

## Key Rules

### 1. Token Economy
- **NEVER** create TODO.md, TASK.md, or similar files
- **NEVER** use internal memory to track tasks
- **ALWAYS** use `gh issue` commands for task management

### 2. Context Loading
Before any task:
```bash
# Read architecture
cat .ai/ARCHITECTURE.md

# Check your assigned issues
gh issue list --assignee "@me"

# If no assignment, check backlog
gh issue list --limit 5
```

### 3. Development Flow
```bash
# Take a task
gh issue edit <id> --add-assignee "@me"

# Create branch
git checkout -b feat/issue-<id>

# After coding, commit with reference
git commit -m "feat: description (closes #<id>)"

# Create PR
gh pr create --fill
```

### 4. Planning Mode
When asked to plan, generate `gh issue create` commands instead of documents:
```bash
gh issue create --title "TASK: Description" --body "Details..." --label "ai-plan"
```

### 5. Code Standards
- Follow existing code style
- Write tests for new features
- Use Conventional Commits
- Keep PRs focused and small

### 6. Communication
- Be concise in commit messages
- Reference issues in all commits
- Update issue comments for significant progress

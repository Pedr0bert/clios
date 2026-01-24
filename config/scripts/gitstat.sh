#!/bin/bash
# ============================================================================
# GITSTAT - Estatísticas do Repositório Git
# ============================================================================

if ! git rev-parse --is-inside-work-tree &>/dev/null; then
    echo " Erro: Não está em um repositório Git"
    exit 1
fi

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                  ESTATÍSTICAS DO REPOSITÓRIO                ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Info básica
REPO_NAME=$(basename "$(git rev-parse --show-toplevel)")
BRANCH=$(git branch --show-current)
REMOTE=$(git remote get-url origin 2>/dev/null || echo "Nenhum")

echo " Repositório: $REPO_NAME"
echo "🌿 Branch: $BRANCH"
echo " Remote: $REMOTE"
echo ""

# Commits
TOTAL_COMMITS=$(git rev-list --count HEAD 2>/dev/null || echo 0)
COMMITS_TODAY=$(git log --oneline --since="midnight" 2>/dev/null | wc -l)
COMMITS_WEEK=$(git log --oneline --since="1 week ago" 2>/dev/null | wc -l)

echo " Commits:"
echo "   Total: $TOTAL_COMMITS"
echo "   Hoje: $COMMITS_TODAY"
echo "   Última semana: $COMMITS_WEEK"
echo ""

# Contributors
echo "👥 Top Contributors (por commits):"
git shortlog -sn --no-merges 2>/dev/null | head -5 | while read line; do
    echo "   $line"
done
echo ""

# Arquivos
TOTAL_FILES=$(git ls-files | wc -l)
LINES=$(git ls-files | xargs wc -l 2>/dev/null | tail -1 | awk '{print $1}')

echo "📄 Arquivos:"
echo "   Total de arquivos: $TOTAL_FILES"
echo "   Total de linhas: ${LINES:-N/A}"
echo ""

# Status atual
MODIFIED=$(git status --porcelain 2>/dev/null | grep -c "^ M")
ADDED=$(git status --porcelain 2>/dev/null | grep -c "^A")
DELETED=$(git status --porcelain 2>/dev/null | grep -c "^ D")
UNTRACKED=$(git status --porcelain 2>/dev/null | grep -c "^??")

echo " Status atual:"
echo "   Modificados: $MODIFIED"
echo "   Adicionados: $ADDED"
echo "   Deletados: $DELETED"
echo "   Não rastreados: $UNTRACKED"
echo ""

# Último commit
echo "🕐 Último commit:"
git log -1 --format="   %h - %s (%cr)" 2>/dev/null
echo ""

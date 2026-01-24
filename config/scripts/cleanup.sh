#!/bin/bash
# ============================================================================
# CLEANUP - Limpeza de Arquivos Temporários
# ============================================================================

echo ""
echo "🧹 LIMPEZA DE SISTEMA"
echo "====================="
echo ""

TOTAL_FREED=0

# Função para calcular tamanho
calc_size() {
    if [ -d "$1" ]; then
        du -sb "$1" 2>/dev/null | cut -f1 || echo 0
    else
        echo 0
    fi
}

# 1. Limpar cache de pacotes (apt)
if command -v apt &> /dev/null; then
    echo " Limpando cache do APT..."
    BEFORE=$(calc_size /var/cache/apt/archives)
    sudo apt clean 2>/dev/null
    sudo apt autoremove -y 2>/dev/null
    AFTER=$(calc_size /var/cache/apt/archives)
    FREED=$((BEFORE - AFTER))
    TOTAL_FREED=$((TOTAL_FREED + FREED))
    echo "   Liberado: $((FREED / 1024 / 1024)) MB"
fi

# 2. Limpar logs antigos
echo " Limpando logs antigos..."
if [ -d /var/log ]; then
    sudo find /var/log -type f -name "*.gz" -delete 2>/dev/null
    sudo find /var/log -type f -name "*.old" -delete 2>/dev/null
    sudo find /var/log -type f -name "*.1" -delete 2>/dev/null
    echo "   Logs antigos removidos"
fi

# 3. Limpar cache do usuário
echo "🗂️  Limpando caches do usuário..."
CACHE_DIRS=(
    "$HOME/.cache/thumbnails"
    "$HOME/.cache/pip"
    "$HOME/.npm/_cacache"
)
for dir in "${CACHE_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        SIZE=$(calc_size "$dir")
        rm -rf "$dir" 2>/dev/null
        TOTAL_FREED=$((TOTAL_FREED + SIZE))
    fi
done
echo "   Caches limpos"

# 4. Limpar lixeira
echo "  Limpando lixeira..."
if [ -d "$HOME/.local/share/Trash" ]; then
    SIZE=$(calc_size "$HOME/.local/share/Trash")
    rm -rf "$HOME/.local/share/Trash"/* 2>/dev/null
    TOTAL_FREED=$((TOTAL_FREED + SIZE))
    echo "   Lixeira limpa"
fi

# 5. Limpar arquivos temporários
echo " Limpando arquivos temporários..."
find /tmp -user "$USER" -type f -mtime +7 -delete 2>/dev/null
echo "   Temporários antigos removidos"

# Resumo
echo ""
echo "════════════════════════════════════"
echo " LIMPEZA CONCLUÍDA"
echo "   Espaço total liberado: ~$((TOTAL_FREED / 1024 / 1024)) MB"
echo "════════════════════════════════════"
echo ""

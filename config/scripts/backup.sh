#!/bin/bash
# ============================================================================
# BACKUP - Script de Backup Simples
# ============================================================================

usage() {
    echo "Uso: backup.sh <origem> [destino]"
    echo ""
    echo "Exemplos:"
    echo "  backup.sh ~/projeto                    # Cria backup com timestamp"
    echo "  backup.sh ~/projeto ~/backups/projeto  # Especifica destino"
    echo ""
    exit 1
}

if [ -z "$1" ]; then
    usage
fi

SOURCE="$1"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BASENAME=$(basename "$SOURCE")

if [ -n "$2" ]; then
    DEST="$2"
else
    DEST="${SOURCE}_backup_${TIMESTAMP}"
fi

if [ ! -e "$SOURCE" ]; then
    echo " Erro: '$SOURCE' não existe"
    exit 1
fi

echo " Criando backup..."
echo "   Origem:  $SOURCE"
echo "   Destino: $DEST"

if [ -d "$SOURCE" ]; then
    # Diretório - usar tar
    TARFILE="${DEST}.tar.gz"
    tar -czf "$TARFILE" -C "$(dirname "$SOURCE")" "$BASENAME" 2>/dev/null
    if [ $? -eq 0 ]; then
        SIZE=$(du -h "$TARFILE" | cut -f1)
        echo " Backup criado: $TARFILE ($SIZE)"
    else
        echo " Erro ao criar backup"
        exit 1
    fi
else
    # Arquivo - simples cópia
    cp "$SOURCE" "$DEST"
    if [ $? -eq 0 ]; then
        SIZE=$(du -h "$DEST" | cut -f1)
        echo " Backup criado: $DEST ($SIZE)"
    else
        echo " Erro ao criar backup"
        exit 1
    fi
fi

#!/bin/bash
# ╔══════════════════════════════════════════════════════════════════════════════╗
# ║                     CLIOS SHELL - SCRIPT DE INSTALAÇÃO                      ║
# ║                              install.sh v1.0.0                              ║
# ╚══════════════════════════════════════════════════════════════════════════════╝

set -e

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo ""
echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║              INSTALADOR DO CLIOS SHELL v1.0.0               ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Detectar diretório do script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Verificar se Rust/Cargo está instalado
if ! command -v cargo &> /dev/null; then
    echo -e "${RED} Erro: Cargo (Rust) não está instalado.${NC}"
    echo -e "   Instale em: ${YELLOW}https://rustup.rs${NC}"
    exit 1
fi
echo -e "${GREEN}${NC} Cargo encontrado"

# Compilar em modo release
echo ""
echo -e "${YELLOW} Compilando Clios Shell...${NC}"
cd "$SCRIPT_DIR"
cargo build --release

if [ ! -f "$SCRIPT_DIR/target/release/clios-shell" ]; then
    echo -e "${RED} Erro: Compilação falhou.${NC}"
    exit 1
fi
echo -e "${GREEN}${NC} Compilação concluída"

# Criar diretórios de configuração
echo ""
echo -e "${YELLOW} Configurando diretórios...${NC}"
mkdir -p ~/.clios_plugins
mkdir -p ~/.clios_scripts
echo -e "${GREEN}${NC} Diretórios criados"

# Copiar arquivos de configuração
echo ""
echo -e "${YELLOW} Instalando configuração...${NC}"

# Backup do .cliosrc existente se houver
if [ -f ~/.cliosrc ]; then
    BACKUP_FILE=~/.cliosrc.backup.$(date +%Y%m%d_%H%M%S)
    cp ~/.cliosrc "$BACKUP_FILE"
    echo -e "${YELLOW}   Backup do .cliosrc existente: $BACKUP_FILE${NC}"
fi

# Copiar .cliosrc
cp "$SCRIPT_DIR/config/cliosrc" ~/.cliosrc
echo -e "${GREEN}${NC} ~/.cliosrc instalado"

# Copiar plugins Rhai
if [ -d "$SCRIPT_DIR/config/plugins" ]; then
    cp "$SCRIPT_DIR/config/plugins/"*.rhai ~/.clios_plugins/ 2>/dev/null || true
    PLUGIN_COUNT=$(ls -1 ~/.clios_plugins/*.rhai 2>/dev/null | wc -l)
    echo -e "${GREEN}${NC} $PLUGIN_COUNT plugins Rhai instalados"
fi

# Copiar scripts
if [ -d "$SCRIPT_DIR/config/scripts" ]; then
    cp "$SCRIPT_DIR/config/scripts/"*.sh ~/.clios_scripts/ 2>/dev/null || true
    chmod +x ~/.clios_scripts/*.sh 2>/dev/null || true
    SCRIPT_COUNT=$(ls -1 ~/.clios_scripts/*.sh 2>/dev/null | wc -l)
    echo -e "${GREEN}${NC} $SCRIPT_COUNT scripts instalados"
fi

# Perguntar sobre instalação global
echo ""
echo -e "${YELLOW} Instalação do binário:${NC}"
echo "   1) Instalar em /usr/local/bin (requer sudo)"
echo "   2) Instalar em ~/.local/bin (sem sudo)"
echo "   3) Não instalar o binário (usar do diretório atual)"
echo ""
read -p "Escolha [1/2/3]: " INSTALL_CHOICE

case $INSTALL_CHOICE in
    1)
        echo -e "${YELLOW}   Instalando em /usr/local/bin...${NC}"
        sudo install -m 755 "$SCRIPT_DIR/target/release/clios-shell" /usr/local/bin/clios
        echo -e "${GREEN}${NC} Binário instalado em /usr/local/bin/clios"
        CLIOS_PATH="/usr/local/bin/clios"
        ;;
    2)
        mkdir -p ~/.local/bin
        install -m 755 "$SCRIPT_DIR/target/release/clios-shell" ~/.local/bin/clios
        echo -e "${GREEN}${NC} Binário instalado em ~/.local/bin/clios"
        CLIOS_PATH="~/.local/bin/clios"
        # Adicionar ao PATH se necessário
        if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
            echo -e "${YELLOW}   Adicione ao seu .bashrc:${NC}"
            echo -e "   ${BLUE}export PATH=\$PATH:\$HOME/.local/bin${NC}"
        fi
        ;;
    3)
        echo -e "${GREEN}${NC} Binário disponível em: $SCRIPT_DIR/target/release/clios-shell"
        CLIOS_PATH="$SCRIPT_DIR/target/release/clios-shell"
        ;;
    *)
        echo -e "${YELLOW}   Pulando instalação do binário${NC}"
        CLIOS_PATH="$SCRIPT_DIR/target/release/clios-shell"
        ;;
esac

# Resumo final
echo ""
echo -e "${GREEN}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║                  INSTALAÇÃO CONCLUÍDA!                    ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e " Arquivos instalados:"
echo -e "   ${BLUE}~/.cliosrc${NC}           - Configuração principal"
echo -e "   ${BLUE}~/.clios_plugins/${NC}    - Plugins Rhai"
echo -e "   ${BLUE}~/.clios_scripts/${NC}    - Scripts utilitários"
echo ""
echo -e " Para iniciar o Clios Shell:"
if [ "$INSTALL_CHOICE" == "1" ] || [ "$INSTALL_CHOICE" == "2" ]; then
    echo -e "   ${YELLOW}clios${NC}"
else
    echo -e "   ${YELLOW}$CLIOS_PATH${NC}"
fi
echo ""
echo -e " Comandos úteis:"
echo -e "   ${YELLOW}help${NC}      - Ver todos os comandos"
echo -e "   ${YELLOW}sysinfo${NC}   - Informações do sistema"
echo -e "   ${YELLOW}gitstat${NC}   - Estatísticas do repositório Git"
echo ""

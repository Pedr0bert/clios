#!/bin/bash
# ============================================================================
# SYSINFO - Informações do Sistema
# ============================================================================

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                    INFORMAÇÕES DO SISTEMA                   ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Sistema Operacional
echo "  Sistema Operacional:"
if [ -f /etc/os-release ]; then
    . /etc/os-release
    echo "    Distro: $PRETTY_NAME"
fi
echo "    Kernel: $(uname -r)"
echo "    Arch: $(uname -m)"
echo ""

# Hardware
echo " Hardware:"
echo "    Hostname: $(hostname)"
if [ -f /proc/cpuinfo ]; then
    CPU=$(grep "model name" /proc/cpuinfo | head -1 | cut -d: -f2 | xargs)
    CORES=$(grep -c "processor" /proc/cpuinfo)
    echo "    CPU: $CPU"
    echo "    Cores: $CORES"
fi
echo ""

# Memória
echo "🧠 Memória:"
if command -v free &> /dev/null; then
    free -h | awk 'NR==2{printf "    Total: %s\n    Usado: %s\n    Livre: %s\n", $2, $3, $4}'
fi
echo ""

# Disco
echo " Disco:"
df -h / | awk 'NR==2{printf "    Total: %s\n    Usado: %s (%s)\n    Livre: %s\n", $2, $3, $5, $4}'
echo ""

# Rede
echo " Rede:"
if command -v ip &> /dev/null; then
    IP=$(ip route get 1.1.1.1 2>/dev/null | awk '{print $7; exit}')
    echo "    IP Local: ${IP:-N/A}"
fi
echo ""

# Uptime
echo "⏱️  Uptime:"
echo "    $(uptime -p 2>/dev/null || uptime)"
echo ""

#!/bin/bash
# =============================================================================
# TESTE DE JOB CONTROL - CLIOS SHELL
# =============================================================================
# Execute dentro do Clios Shell para testar funcionalidades de job control
# =============================================================================

echo "=============================================="
echo "     TESTE DE JOB CONTROL - CLIOS SHELL"
echo "=============================================="
echo ""

# Teste 1: Variável $$ (PID da shell)
echo "[TESTE 1] Variável \$\$ (PID da shell)"
echo "  PID da shell: $$"
echo ""

# Teste 2: Variável $? (exit code)
echo "[TESTE 2] Variável \$? (exit code)"
true
echo "  Após 'true':  \$? = $?"
false
echo "  Após 'false': \$? = $?"
echo ""

# Teste 3: Operador && (AND)
echo "[TESTE 3] Operador && (AND - curto-circuito)"
echo "  'true && echo OK' -> " && true && echo "OK"
echo "  'false && echo FALHA' -> " && false && echo "FALHA (não deve aparecer)"
echo ""

# Teste 4: Operador || (OR)
echo "[TESTE 4] Operador || (OR - curto-circuito)"
echo "  'false || echo FALLBACK' ->"
false || echo "  FALLBACK executado!"
echo "  'true || echo SKIP' ->"
true || echo "  SKIP (não deve aparecer)"
echo ""

# Teste 5: Redirecionamento de entrada
echo "[TESTE 5] Redirecionamento de entrada (<)"
echo -e "linha1\nlinha2\nlinha3" > /tmp/clios_test_input.txt
echo "  Contando linhas de /tmp/clios_test_input.txt:"
echo "  wc -l < /tmp/clios_test_input.txt = $(wc -l < /tmp/clios_test_input.txt)"
rm -f /tmp/clios_test_input.txt
echo ""

# Teste 6: Background jobs
echo "[TESTE 6] Background jobs (&)"
echo "  Iniciando 'sleep 5 &'..."
echo "  (Execute 'jobs' para ver o processo)"
echo "  (Aguarde 5s ou use 'fg PID' para trazer para foreground)"
echo ""

echo "=============================================="
echo "     TESTES INTERATIVOS (execute manualmente)"
echo "=============================================="
echo ""
echo "1. Testar background:"
echo "   $ sleep 10 &"
echo "   $ jobs"
echo ""
echo "2. Testar foreground:"
echo "   $ sleep 30 &"
echo "   $ jobs"
echo "   $ fg <PID>"
echo ""
echo "3. Testar Ctrl+Z (pausar):"
echo "   $ sleep 60"
echo "   [Pressione Ctrl+Z]"
echo "   $ jobs"
echo "   $ fg <PID>"
echo ""
echo "=============================================="
echo "     FIM DOS TESTES"
echo "=============================================="

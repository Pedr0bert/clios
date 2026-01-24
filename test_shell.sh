#!/bin/bash
# Script de Teste para Clios Shell
# Testa funcionalidades básicas e casos extremos

echo "========================================="
echo "   SUITE DE TESTES CLIOS SHELL"
echo "========================================="
echo ""

CLIOS_BIN="./target/debug/clios-shell"

if [ ! -f "$CLIOS_BIN" ]; then
    echo " Erro: Executável não encontrado. Execute 'cargo build' primeiro."
    exit 1
fi

echo " Executável encontrado: $CLIOS_BIN"
echo ""

# Contador de testes
PASSED=0
FAILED=0

# Função auxiliar para executar teste
run_test() {
    local test_name="$1"
    local command="$2"
    local expected_exit="$3"  # código de saída esperado (0 = sucesso)
    
    echo -n "Testando: $test_name ... "
    
    # Executa o comando
    echo "$command" | timeout 2s "$CLIOS_BIN" > /tmp/clios_test_output 2>&1
    local exit_code=$?
    
    # Verifica se passou do timeout (código 124)
    if [ $exit_code -eq 124 ]; then
        echo " TIMEOUT"
        FAILED=$((FAILED + 1))
        return 1
    fi
    
    # Se esperávamos sucesso (0) e obtivemos sucesso
    if [ "$expected_exit" = "0" ] && [ $exit_code -eq 0 ]; then
        echo " OK"
        PASSED=$((PASSED + 1))
        return 0
    fi
    
    # Se não esperávamos sucesso exato, mas o comando terminou
    if [ "$expected_exit" = "any" ]; then
        echo " OK (exit: $exit_code)"
        PASSED=$((PASSED + 1))
        return 0
    fi
    
    echo " FALHOU (exit: $exit_code)"
    FAILED=$((FAILED + 1))
    return 1
}

# ==========================================
# TESTES BÁSICOS
# ==========================================

echo "=== Testes Básicos ==="

run_test "Comando simples (pwd)" "pwd
exit" "0"

run_test "Comando com argumentos" "echo Hello World
exit" "0"

run_test "Múltiplos comandos" "echo test1
echo test2
exit" "0"

# ==========================================
# TESTES DE BUILTINS
# ==========================================

echo ""
echo "=== Testes de Builtins ==="

run_test "Comando cd" "cd /tmp
pwd
exit" "0"

run_test "Comando pwd" "pwd
exit" "0"

run_test "Comando export" "export TEST_VAR=hello
exit" "0"

run_test "Comando alias" "alias ll='ls -la'
exit" "0"

# ==========================================
# TESTES DE EXPANSÃO
# ==========================================

echo ""
echo "=== Testes de Expansão ==="

run_test "Expansão de variável" "export MY_VAR=test
echo \$MY_VAR
exit" "0"

run_test "Expansão de til" "echo ~
exit" "0"

run_test "Subshell simples" "echo \$(echo nested)
exit" "0"

# ==========================================
# TESTES DE PIPELINE
# ==========================================

echo ""
echo "=== Testes de Pipeline ==="

run_test "Pipeline simples" "echo hello | cat
exit" "0"

run_test "Pipeline triplo" "echo test | cat | cat
exit" "0"

# ==========================================
# TESTES DE REDIRECIONAMENTO
# ==========================================

echo ""
echo "=== Testes de Redirecionamento ==="

run_test "Redirecionamento stdout" "echo test > /tmp/clios_test_redir.txt
exit" "0"

run_test "Redirecionamento append" "echo test2 >> /tmp/clios_test_redir.txt
exit" "0"

# ==========================================
# TESTES DE LÓGICA &&
# ==========================================

echo ""
echo "=== Testes de Lógica && ==="

run_test "AND lógico (sucesso)" "echo first && echo second
exit" "0"

run_test "AND lógico com falha" "false && echo should_not_appear
exit" "any"

# ==========================================
# TESTES DE CASOS EXTREMOS
# ==========================================

echo ""
echo "=== Testes de Casos Extremos ==="

run_test "Comando vazio (apenas enter)" "

exit" "0"

run_test "Múltiplos espaços" "     echo    test    
exit" "0"

run_test "Aspas não fechadas" "echo \"test
exit" "any"

run_test "Pipe vazio" "echo test |
exit" "any"

run_test "Redirecionamento sem arquivo" "echo test >
exit" "any"

run_test "Subshell não fechado" "echo \$(echo test
exit" "any"

# ==========================================
# TESTES DE ALIAS RECURSIVO
# ==========================================

echo ""
echo "=== Testes de Alias Recursivo ==="

run_test "Alias recursivo direto" "alias test=test
test
exit" "any"

run_test "Alias válido" "alias hello='echo Hello World'
hello
exit" "0"

# ==========================================
# TESTES RHAI
# ==========================================

echo ""
echo "=== Testes Rhai ==="

run_test "Rhai simples" "rhai 2 + 2
exit" "0"

run_test "Rhai com variável" "rhai let x = 10; x * 2
exit" "0"

# ==========================================
# RESULTADOS FINAIS
# ==========================================

echo ""
echo "========================================="
echo "   RESULTADOS"
echo "========================================="
echo " Passaram: $PASSED"
echo " Falharam: $FAILED"
echo " Total: $((PASSED + FAILED))"
echo ""

if [ $FAILED -eq 0 ]; then
    echo " Todos os testes passaram!"
    exit 0
else
    echo "⚠️  Alguns testes falharam."
    exit 1
fi

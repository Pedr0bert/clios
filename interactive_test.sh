#!/bin/bash
# Script de teste interativo para Clios Shell

echo "========================================="
echo "   TESTE INTERATIVO CLIOS SHELL"
echo "========================================="
echo ""

CLIOS_BIN="./target/debug/clios-shell"

# Criar arquivo de comandos de teste
cat > /tmp/clios_interactive_tests.txt << 'EOF'
# Testes Básicos
pwd
echo Hello from Clios Shell

# Testes de Expansão
export MY_VAR=TestValue
echo Variable: $MY_VAR
echo Home: ~

# Testes de Subshell
echo Today is $(date +%Y-%m-%d)
echo Files in current dir: $(ls | wc -l)

# Testes de Alias
alias hello='echo Hello World'
alias
hello

# Teste de Alias Recursivo (deve avisar)
alias ls=ls
ls

# Testes de Redirecionamento
echo "Test line 1" > /tmp/clios_output.txt
echo "Test line 2" >> /tmp/clios_output.txt
cat /tmp/clios_output.txt

# Testes de Lógica &&
echo "First" && echo "Second"
false && echo "Should not appear"

# Testes Rhai
rhai 2 + 2
rhai let x = 5; x * x
rhai "Clios " + "Shell"

# Testes de Casos Extremos
echo $()
echo $(echo unclosed
echo test >

# Teste de comando vazio (apenas enter)


# Múltiplos espaços
   echo    test   

# Exit
exit
EOF

echo "Executando testes interativos..."
echo ""
echo "Comandos que serão executados:"
echo "---"
cat /tmp/clios_interactive_tests.txt | grep -v '^#' | grep -v '^$' | head -20
echo "..."
echo "---"
echo ""

$CLIOS_BIN < /tmp/clios_interactive_tests.txt 2>&1 | tail -100

echo ""
echo "========================================="
echo "Testes interativos concluídos!"
echo "========================================="

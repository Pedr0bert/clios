# Guia de Depuração e Testes - Clios Shell

Este documento contém comandos e cenários de teste para depurar e validar
o funcionamento da shell Clios.

##  Compilação e Execução

```bash
# Compilar o projeto
cargo build

# Executar a shell
./target/debug/clios-shell

# Executar com um comando específico
./target/debug/clios-shell -c "echo Hello World"

# Executar um script
./target/debug/clios-shell script.sh
```

##  Testes Básicos

### Comandos Simples
```bash
pwd
ls
echo Hello World
date
whoami
```

### Builtins
```bash
# Navegação
cd /tmp
pwd
cd -        # Volta para pasta anterior
cd ~        # Vai para HOME

# Variáveis de ambiente
export MY_VAR=test
echo $MY_VAR
export PATH=/usr/local/bin:$PATH

# Aliases
alias ll='ls -la'
alias
ll

# Histórico
history

# Exit codes
pwd
echo $?     # Deve mostrar 0 (sucesso)
false
echo $?     # Deve mostrar 1 (falha)
```

##  Testes de Pipeline

### Pipeline Simples
```bash
ls | grep txt
echo "test" | cat
ps aux | grep bash
```

### Pipeline Múltiplo
```bash
ls | grep txt | wc -l
cat file.txt | sort | uniq
echo "hello world" | tr a-z A-Z | rev
```

### Pipeline Vazio (Teste de Erro)
```bash
echo test |
| cat
```

##  Testes de Redirecionamento

### Stdout Redirection
```bash
echo "Hello" > output.txt
cat output.txt

echo "World" >> output.txt
cat output.txt
```

### Stderr Redirection
```bash
ls /nonexistent 2> error.txt
cat error.txt

ls /another_nonexistent 2>> error.txt
cat error.txt
```

### Combinado
```bash
ls /tmp /nonexistent > output.txt 2> error.txt
cat output.txt
cat error.txt
```

### Redirecionamento Inválido (Teste de Erro)
```bash
echo test >
echo test 2>
```

## 🧮 Testes de Expansão

### Expansão de Variáveis
```bash
export NAME=Pedro
echo Hello $NAME
echo Path: $PATH
echo Home: $HOME

# Com chaves
echo ${NAME}_suffix
```

### Expansão de Til
```bash
cd ~
ls ~/Downloads
echo ~
```

### Expansão de Glob
```bash
ls *.rs
echo src/*.rs
ls **/*.toml
```

### Subshells
```bash
echo Today is $(date)
echo Current dir: $(pwd)
echo Files: $(ls | wc -l)

# Subshell aninhado
echo $(echo $(echo nested))

# Subshell não fechado (deve gerar erro)
echo $(echo test
```

## 🔀 Testes de Lógica &&

### AND Lógico
```bash
# Ambos executam
echo first && echo second

# Segundo não executa (primeiro falha)
false && echo should_not_appear

# Cadeia de comandos
mkdir test_dir && cd test_dir && pwd

# Com pipeline
echo test | cat && echo success
```

### Aspas em &&
```bash
# Não deve dividir (bug antigo)
echo "a && b"
echo "test && test2"
```

## 🎭 Testes de Alias

### Alias Simples
```bash
alias hello='echo Hello World'
hello

alias update='sudo apt update'
alias
```

### Alias com Pipes
```bash
alias lsl='ls -la | grep'
lsl txt
```

### Alias Recursivo (Teste de Proteção)
```bash
# Deve detectar e prevenir
alias ls=ls
ls

# Deve avisar
alias test=test
test
```

### Alias com &&
```bash
alias deploy='cargo build && cargo test && echo Deploy OK'
deploy
```

## 🐚 Testes Rhai

### Expressões Simples
```bash
rhai 2 + 2
rhai 10 * 5
rhai "Hello " + "World"
```

### Variáveis
```bash
rhai let x = 10; x * 2
rhai let name = "Clios"; "Welcome to " + name
```

### Funções
```bash
# No modo REPL
rhai
fn greet(name) {
    "Hello, " + name
}
greet("Pedro")
exit
```

### Plugins
```bash
# Carregar plugin
source meu_plugin.rhai

# Listar plugins
plugins

# Executar função do plugin
my_custom_command
```

## 🚨 Testes de Casos Extremos

### Entrada Vazia
```bash
# Apenas pressionar Enter
<enter>
<enter>
```

### Múltiplos Espaços
```bash
     echo     test     
echo    "test"    
```

### Aspas Não Fechadas
```bash
echo "test
echo 'test
echo "test1' test2"
```

### Caracteres Especiais
```bash
echo $$$
echo ;;;
echo |||
echo &&&
```

### Comandos Muito Longos
```bash
echo aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
```

### Pipeline com Falhas
```bash
false | echo "should appear"
echo test | false | cat
```

##  Testes de Job Control

### Background Jobs
```bash
sleep 10 &
jobs
fg %1

# Múltiplos jobs
sleep 20 &
sleep 30 &
jobs
```

##  Testes de Tema

### Trocar Temas
```bash
theme powerline
theme classic
theme powerline
```

##  Testes de Configuração

### Arquivo ~/.cliosrc
Crie um arquivo ~/.cliosrc com:
```bash
# Aliases
alias gs='git status'
alias gc='git commit'

# Variáveis
export EDITOR=vim

# Configurações
echo "Shell inicializada!"
```

##  Comandos de Depuração Úteis

### Verificar Erros de Sintaxe
```bash
# Teste shlex parsing
echo "test
echo 'test
```

### Verificar Expansões
```bash
# Antes e depois
export TEST=hello
echo $TEST
echo ${TEST}
```

### Verificar Exit Codes
```bash
true
echo $?   # 0
false
echo $?   # 1
```

### Verificar Pipes
```bash
echo test | cat
echo $?   # Deve ser 0
```

##  Testes de Performance

### Muitos Comandos Sequenciais
```bash
for i in {1..100}; do echo $i; done
```

### Pipeline Longo
```bash
echo test | cat | cat | cat | cat | cat
```

##  Casos de Teste Específicos

### Teste 1: Alias Recursivo
```bash
# Criar alias recursivo
alias echo='echo'
echo test
# Esperado: Aviso e uso do comando original
```

### Teste 2: Subshell com Erro
```bash
echo $(invalid_command)
# Esperado: Mensagem de erro clara
```

### Teste 3: Redirecionamento com Permissão Negada
```bash
echo test > /root/forbidden.txt
# Esperado: Mensagem de erro de permissão
```

### Teste 4: Pipeline Vazio
```bash
|
# Esperado: Mensagem de erro
```

### Teste 5: Comando Vazio em Pipeline
```bash
echo test | | cat
# Esperado: Aviso sobre comando vazio
```

## 🛠️ Checklist de Depuração

- [ ] Comandos básicos funcionam (ls, pwd, echo)
- [ ] Builtins funcionam (cd, export, alias)
- [ ] Pipelines simples funcionam
- [ ] Pipelines múltiplos funcionam
- [ ] Redirecionamento stdout funciona
- [ ] Redirecionamento stderr funciona
- [ ] Expansão de variáveis funciona
- [ ] Expansão de til funciona
- [ ] Expansão de glob funciona
- [ ] Subshells funcionam
- [ ] Lógica && funciona
- [ ] Aliases funcionam
- [ ] Aliases recursivos são detectados
- [ ] Comandos vazios não travam
- [ ] Erros de sintaxe são reportados
- [ ] Rhai funciona
- [ ] Plugins podem ser carregados
- [ ] Exit codes são corretos
- [ ] Job control funciona
- [ ] Temas podem ser trocados
- [ ] Arquivo ~/.cliosrc é carregado

## 📖 Mensagens de Erro Esperadas

### Boas Mensagens de Erro (Implementadas)
- `[ERRO SINTAXE] Aspas não fechadas`
- `[ERRO PLUGIN] Arquivo não encontrado`
- `[ERRO REDIRECIONAMENTO] Falha ao abrir arquivo`
- `[AVISO] Alias recursivo detectado`
- `[AVISO] Comando vazio no pipeline`
- `[AVISO] Subshell vazio: $()`
- `[ERRO] Comando não encontrado no subshell`

##  Comandos Avançados para Testar

```bash
# Combinação de tudo
export NAME=test && echo $NAME | cat > output.txt && cat output.txt

# Pipeline com redirecionamento
ls -la | grep txt > results.txt 2> errors.txt

# Subshell em variável
export CURRENT=$(pwd)
echo $CURRENT

# Alias com subshell
alias now='echo Current time: $(date)'
now
```

##  Notas

- Todos os testes devem executar sem travar a shell
- Erros devem gerar mensagens claras e descritivas
- Exit codes devem ser preservados corretamente
- Aliases recursivos devem ser detectados e prevenidos
- Comandos vazios devem ser ignorados silenciosamente
- Subshells com erros devem reportar o problema
- Redirecionamento inválido deve gerar erro claro

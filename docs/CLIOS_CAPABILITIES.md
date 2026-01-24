# Clios Shell - Capacidades e Funcionalidades Completas

> Documentação completa para treinamento de IA - Todas as funcionalidades, comandos e capacidades do Clios Shell v1.0.0

## Índice

1. [Visão Geral](#visão-geral)
2. [Comandos Internos (Builtins)](#comandos-internos-builtins)
3. [Operadores de Shell](#operadores-de-shell)
4. [Expansões](#expansões)
5. [Plugins Rhai](#plugins-rhai)
6. [Scripts Utilitários](#scripts-utilitários)
7. [Aliases Pré-configurados](#aliases-pré-configurados)
8. [Configuração](#configuração)
9. [Exemplos de Uso](#exemplos-de-uso)

---

## Visão Geral

Clios Shell é uma shell híbrida Rust + Rhai para Linux/Unix que combina:
- Execução de comandos do sistema
- Scripting avançado com Rhai
- Job control Unix completo
- Pipeline, redirecionamento e lógica condicional
- Sistema de plugins extensível
- Prompt powerline personalizado

**Versão:** 1.0.0  
**Linguagem:** Rust  
**Scripting:** Rhai  
**Plataformas:** Linux, macOS, WSL

---

## Comandos Internos (Builtins)

Comandos executados internamente pela shell (não spawnam processos externos).

### Navegação de Diretórios

#### `cd [diretório]`
Muda o diretório de trabalho atual.

**Sintaxe:**
```bash
cd [caminho]          # Vai para o diretório especificado
cd                    # Vai para $HOME
cd -                  # Volta para o diretório anterior
cd ~/Documents        # Suporta expansão de til
cd $HOME/Downloads    # Suporta variáveis
```

**Comportamento:**
- Sem argumentos: vai para `$HOME`
- Com `-`: alterna entre diretório atual e anterior
- Armazena diretório anterior em memória
- Expande `~` e variáveis de ambiente

**Exemplo:**
```bash
cd /tmp
cd ~/projects
cd -  # volta para ~/projects
```

#### `pwd`
Exibe o caminho completo do diretório atual.

**Sintaxe:**
```bash
pwd
```

**Saída:**
```
/home/usuario/projetos/clios
```

---

### Gerenciamento de Aliases

#### `alias [nome='comando']`
Cria ou lista aliases (atalhos para comandos).

**Sintaxe:**
```bash
alias                    # Lista todos os aliases
alias ll='ls -la'        # Cria alias
alias nome='comando'     # Formato geral
```

**Funcionalidades:**
- Aliases podem conter pipes, redirecionamentos e lógica
- Expansão recursiva até 10 níveis
- Detecção automática de aliases circulares
- Aliases são expandidos antes da execução

**Exemplos:**
```bash
# Simples
alias ll='ls -la'
alias ..='cd ..'

# Com pipes
alias count='ls -1 | wc -l'

# Com lógica
alias update='sudo apt update && sudo apt upgrade -y'

# Git shortcuts
alias gs='git status'
alias gp='git push'
```

**Proteções:**
- Detecta `alias ls=ls` (circular) e usa comando original
- Limite de 10 níveis de recursão

#### `unalias <nome>`
Remove um alias existente.

**Sintaxe:**
```bash
unalias ll              # Remove o alias 'll'
```

**Comportamento:**
- Remove permanentemente da sessão atual
- Não afeta aliases em ~/.cliosrc
- Mensagem de erro se alias não existir

---

### Variáveis de Ambiente

#### `export VAR=valor`
Define uma variável de ambiente.

**Sintaxe:**
```bash
export NOME=valor
export PATH=$PATH:/novo/caminho
export EDITOR=vim
```

**Características:**
- Variáveis disponíveis para processos filhos
- Suporta expansão de variáveis existentes
- Sem espaços ao redor do `=`

**Exemplos:**
```bash
export JAVA_HOME=/usr/lib/jvm/java-17
export PATH=$PATH:$JAVA_HOME/bin
export DATABASE_URL=postgresql://localhost/mydb
```

#### `unset VAR`
Remove uma variável de ambiente.

**Sintaxe:**
```bash
unset JAVA_HOME
unset DATABASE_URL
```

**Comportamento:**
- Remove completamente a variável
- Pode remover múltiplas variáveis: `unset VAR1 VAR2`

---

### Histórico

#### `history`
Exibe o histórico de comandos da sessão.

**Sintaxe:**
```bash
history
```

**Saída:**
```
    1  ls -la
    2  cd /tmp
    3  git status
```

**Características:**
- Persistido em `~/.clios_history`
- Máximo configurável (padrão: 1000 entradas)
- Compartilhado entre sessões

---

### Sistema de Plugins Rhai

#### `source <arquivo.rhai>` / `load <arquivo.rhai>`
Carrega um plugin Rhai na sessão atual.

**Sintaxe:**
```bash
source ~/.clios_plugins/utils.rhai
load meu_plugin.rhai
```

**Comportamento:**
- Compila e adiciona funções ao escopo global
- Funções ficam disponíveis via comando `rhai`
- Plugins são acumulativos (múltiplos carregamentos)
- Auto-carregamento de `~/.clios_plugins/*.rhai` no início

**Exemplo:**
```bash
source ~/.clios_plugins/git_helpers.rhai
rhai git_cheatsheet()
```

#### `plugins`
Lista todos os plugins carregados e suas funções.

**Sintaxe:**
```bash
plugins
```

**Saída:**
```
Comandos de Plugins Ativos:
----------------------------
  ➜ upper (1 args)
  ➜ lower (1 args)
  ➜ git_cheatsheet (0 args)
  ➜ http_codes (0 args)
----------------------------
```

**Informações exibidas:**
- Nome da função
- Número de argumentos
- Funções privadas (iniciadas com `_`) são ocultadas

---

### Execução Rhai

#### `rhai [código]`
Executa código Rhai inline ou entra no modo REPL.

**Sintaxe:**
```bash
# Modo inline (one-shot)
rhai 2 + 2
rhai "Hello " + "World"
rhai let x = 10; x * x

# Modo REPL (interativo)
rhai
```

**Modo REPL:**
```
Entrando no modo Rhai (Digite 'exit' para sair)
rhai> let x = 5
rhai> x * 2
=> 10
rhai> fn soma(a, b) { a + b }
rhai> soma(3, 7)
=> 10
rhai> exit
```

**Características:**
- Acesso completo a funções de plugins carregados
- Variáveis persistem na sessão
- Suporta definição de funções em tempo real
- Brackets balanceados automaticamente

**Funções Rhai Built-in Especiais:**

##### `shell_exec(comando)`
Executa comando shell e retorna resultado.

**Retorno:** Map com `success` (bool) e `output` (string)

```rhai
let result = shell_exec("ls -la");
if result.success {
    print(result.output);
}
```

##### `input(prompt)`
Solicita entrada do usuário.

```rhai
let nome = input("Digite seu nome: ");
print("Olá, " + nome);
```

##### `confirm(pergunta)`
Exibe pergunta sim/não.

**Retorno:** bool

```rhai
if confirm("Deseja continuar?") {
    print("Continuando...");
}
```

##### `select(prompt, opcoes)`
Menu de seleção interativo.

```rhai
let opcoes = ["Opção 1", "Opção 2", "Opção 3"];
let escolha = select("Escolha uma opção:", opcoes);
print("Você escolheu: " + escolha);
```

##### `http_get(url)`
Faz requisição HTTP GET.

**Retorno:** String com corpo da resposta

```rhai
let html = http_get("https://example.com");
print(html);
```

##### `save_file(caminho, conteudo)`
Salva conteúdo em arquivo.

**Retorno:** bool (sucesso)

```rhai
let sucesso = save_file("/tmp/teste.txt", "Conteúdo");
```

---

### Job Control

#### `fg <PID>`
Traz processo em background para foreground.

**Sintaxe:**
```bash
fg 12345
```

**Comportamento:**
- Retoma processo pausado
- Transfere controle do terminal
- Usa SIGCONT para continuar
- Espera processo terminar

**Exemplo:**
```bash
# Pausar com Ctrl+Z
sleep 100
^Z
[Job 12345] Pausado

# Retomar
fg 12345
```

---

### Informações e Ajuda

#### `type <comando>`
Mostra o tipo de um comando (builtin, alias, executável).

**Sintaxe:**
```bash
type cd
type ls
type ll
```

**Saídas possíveis:**
```
cd is a shell builtin
ls is /usr/bin/ls
ll is aliased to 'ls -la'
comandoinexistente: not found
```

#### `help`
Exibe ajuda completa dos comandos internos.

**Sintaxe:**
```bash
help
```

**Saída:** Tabela formatada com todos os builtins e descrições.

#### `version`
Exibe versão da shell.

**Sintaxe:**
```bash
version
```

**Saída:**
```
Clios Shell v1.0.0 (Final Release)
Desenvolvido em Rust
```

#### `exit`
Sai da shell.

**Sintaxe:**
```bash
exit
```

---

## Operadores de Shell

### Pipeline (|)

Conecta a saída (stdout) de um comando à entrada (stdin) do próximo.

**Sintaxe:**
```bash
comando1 | comando2 | comando3
```

**Características:**
- Respeita aspas (não divide `"a | b"`)
- Execução em paralelo (processos filhos)
- Propagação de exit codes
- Buffer automático em memória

**Exemplos:**
```bash
ls -la | grep ".rs"
cat arquivo.txt | grep "erro" | wc -l
ps aux | grep python | awk '{print $2}'
```

### Lógica Condicional AND (&&)

Executa segundo comando apenas se o primeiro ter sucesso (exit code 0).

**Sintaxe:**
```bash
comando1 && comando2 && comando3
```

**Comportamento:**
- Para na primeira falha
- Respeita aspas
- Curto-circuito (short-circuit)

**Exemplos:**
```bash
cd /tmp && ls -la
mkdir projeto && cd projeto && git init
cargo build && cargo test && cargo run
```

### Redirecionamento de Saída

#### Stdout Overwrite (>)
Redireciona saída padrão para arquivo (sobrescreve).

**Sintaxe:**
```bash
comando > arquivo.txt
```

**Exemplos:**
```bash
echo "Hello" > output.txt
ls -la > listagem.txt
date > timestamp.txt
```

#### Stdout Append (>>)
Redireciona saída padrão para arquivo (adiciona ao final).

**Sintaxe:**
```bash
comando >> arquivo.txt
```

**Exemplos:**
```bash
echo "Nova linha" >> log.txt
date >> eventos.log
```

#### Stderr Overwrite (2>)
Redireciona saída de erro para arquivo (sobrescreve).

**Sintaxe:**
```bash
comando 2> erros.txt
```

**Exemplos:**
```bash
ls /naoexiste 2> erros.log
gcc programa.c 2> compilation_errors.txt
```

#### Stderr Append (2>>)
Redireciona saída de erro para arquivo (adiciona).

**Sintaxe:**
```bash
comando 2>> erros.txt
```

**Exemplos:**
```bash
find / -name "arquivo" 2>> erros.log
```

#### Combinações
```bash
# Stdout e stderr para arquivos diferentes
comando > saida.txt 2> erros.txt

# Stdout append e stderr overwrite
comando >> saida.txt 2> erros.txt
```

### Background (&)

Executa comando em background (não bloqueia terminal).

**Sintaxe:**
```bash
comando &
```

**Comportamento:**
- Retorna PID do processo
- Shell continua disponível
- Processo continua rodando

**Exemplos:**
```bash
sleep 100 &
python servidor.py &
npm start &
```

---

## Expansões

### Variáveis ($VAR, ${VAR})

Substitui variáveis por seus valores.

**Formatos:**
```bash
$VAR              # Simples
${VAR}            # Com chaves (recomendado)
${VAR}_suffix     # Útil para concatenação
```

**Exemplos:**
```bash
echo $HOME
echo $USER
echo ${PATH}
echo Backup_${USER}.tar.gz
```

**Variáveis comuns:**
- `$HOME` - Diretório home do usuário
- `$USER` - Nome do usuário
- `$PATH` - Caminho de busca de executáveis
- `$PWD` - Diretório atual
- `$SHELL` - Shell atual

### Til (~)

Expande para o diretório home.

**Formatos:**
```bash
~              # /home/usuario
~/Documents    # /home/usuario/Documents
```

**Exemplos:**
```bash
cd ~
ls ~/Downloads
cp arquivo.txt ~/backup/
```

### Globs (*, ?)

Expansão de padrões de arquivos.

**Padrões:**
- `*` - Qualquer sequência de caracteres
- `?` - Um único caractere
- `[abc]` - Um caractere entre a, b, c
- `[0-9]` - Um dígito

**Exemplos:**
```bash
ls *.rs              # Todos arquivos .rs
rm *.txt             # Remove todos .txt
ls arquivo?.txt      # arquivo1.txt, arquivo2.txt
ls [A-Z]*.rs        # Arquivos .rs começando com maiúscula
```

### Subshells ($())

Executa comando e substitui pela saída.

**Sintaxe:**
```bash
$(comando)
```

**Comportamento:**
- Executa comando em subprocesso
- Captura stdout
- Remove trailing newline
- Aninhamento suportado

**Exemplos:**
```bash
echo "Hoje é $(date)"
echo "Arquivos: $(ls | wc -l)"
cd $(pwd)/subdir
echo "User: $(whoami) em $(hostname)"

# Aninhado
echo "Arquivos Rust: $(ls *.rs | wc -l)"
```

**Casos especiais:**
```bash
# Rhai em subshell
echo "Resultado: $(rhai 2 + 2)"
```

---

## Plugins Rhai

### utils.rhai

Funções utilitárias gerais.

#### Manipulação de Texto

##### `upper(texto)`
Converte texto para maiúsculas.

```rhai
rhai upper("hello")  // => "HELLO"
```

##### `lower(texto)`
Converte texto para minúsculas.

```rhai
rhai lower("WORLD")  // => "world"
```

##### `capitalize(texto)`
Capitaliza primeira letra de cada palavra.

```rhai
rhai capitalize("hello world")  // => "Hello World"
```

#### Matemática

##### `sum(array)`
Soma todos os elementos de um array.

```rhai
rhai sum([1, 2, 3, 4, 5])  // => 15
```

##### `avg(array)`
Calcula média de um array.

```rhai
rhai avg([10, 20, 30])  // => 20
```

##### `factorial(n)`
Calcula fatorial de n.

```rhai
rhai factorial(5)  // => 120
```

#### Arrays

##### `reverse_array(arr)`
Inverte ordem dos elementos.

```rhai
rhai reverse_array([1, 2, 3])  // => [3, 2, 1]
```

##### `unique(arr)`
Remove duplicatas.

```rhai
rhai unique([1, 2, 2, 3, 3, 3])  // => [1, 2, 3]
```

---

### git_helpers.rhai

Ferramentas para Git.

##### `commit_msg()`
Exibe guia para mensagens de commit.

```rhai
rhai commit_msg()
```

**Saída:**
```
Tipos: feat, fix, docs, style, refactor, test, chore
Exemplo: feat: adiciona login com OAuth
```

##### `commit_types()`
Lista tipos de commit convencionais.

```rhai
rhai commit_types()
```

**Saída:**
```
feat:     Nova funcionalidade
fix:      Correção de bug
docs:     Documentação
style:    Formatação
refactor: Refatoração
test:     Testes
chore:    Manutenção
```

##### `branch_name()`
Sugere nome de branch seguindo convenções.

```rhai
rhai branch_name()
```

**Saída:**
```
Padrões:
feature/descricao
bugfix/descricao
hotfix/descricao
```

##### `git_cheatsheet()`
Exibe comandos Git úteis.

```rhai
rhai git_cheatsheet()
```

**Saída:** Lista de comandos Git comuns com descrições.

##### `git_flow()`
Mostra workflow GitFlow.

```rhai
rhai git_flow()
```

---

### dev_tools.rhai

Ferramentas para desenvolvimento.

#### Geradores de Código Rust

##### `rust_fn(nome, params, return_type)`
Gera template de função Rust.

```rhai
rhai rust_fn("calcular", "x: i32, y: i32", "i32")
```

**Saída:**
```rust
/// TODO: Documentar função
fn calcular(x: i32, y: i32) -> i32 {
    todo!()
}
```

##### `rust_struct(nome, fields)`
Gera template de struct Rust.

```rhai
let fields = ["nome: String", "idade: u32"];
rhai rust_struct("Pessoa", fields)
```

**Saída:**
```rust
/// TODO: Documentar struct
#[derive(Debug, Clone)]
pub struct Pessoa {
    pub nome: String,
    pub idade: u32,
}
```

##### `rust_test(nome)`
Gera template de teste Rust.

```rhai
rhai rust_test("calculo")
```

**Saída:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculo() {
        // Arrange
        
        // Act
        
        // Assert
        assert!(true);
    }
}
```

#### Templates de Projeto

##### `readme_template(nome_projeto)`
Gera README.md básico.

```rhai
rhai readme_template("meu-projeto")
```

##### `gitignore_rust()`
Gera .gitignore para Rust.

```rhai
rhai gitignore_rust()
```

**Saída:**
```
/target/
Cargo.lock
**/*.rs.bk
*.pdb
.env
.DS_Store
```

#### HTTP Status

##### `http_status(codigo)`
Mostra descrição de código HTTP.

```rhai
rhai http_status(404)  // => "404 - Not Found"
rhai http_status(200)  // => "200 - OK"
```

**Códigos suportados:**
- 200, 201, 204 (Success)
- 301, 302 (Redirect)
- 400, 401, 403, 404 (Client errors)
- 500, 502, 503 (Server errors)

##### `http_codes()`
Lista códigos HTTP comuns.

```rhai
rhai http_codes()
```

**Saída:**
```
HTTP STATUS CODES
=================
2xx Success: 200 OK, 201 Created, 204 No Content
3xx Redirect: 301 Moved, 302 Found, 304 Not Modified
4xx Client: 400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found
5xx Server: 500 Internal Error, 502 Bad Gateway, 503 Unavailable
```

---

## Scripts Utilitários

Localizados em `~/.clios_scripts/` e acessíveis via aliases.

### sysinfo.sh

Exibe informações detalhadas do sistema.

**Uso:**
```bash
sysinfo
```

**Informações exibidas:**
- Sistema operacional e versão
- Kernel
- Arquitetura (x86_64, ARM, etc.)
- CPU (modelo e núcleos)
- Memória RAM (total e disponível)
- Espaço em disco
- Interfaces de rede e IPs
- Uptime do sistema

**Exemplo de saída:**
```
Sistema Operacional:
  OS: Ubuntu 22.04 LTS
  Kernel: 5.15.0-91-generic
  Arquitetura: x86_64

Hardware:
  CPU: Intel Core i7-9700K
  Núcleos: 8
  RAM Total: 16 GB
  RAM Disponível: 8 GB

Disco:
  /dev/sda1: 450G / 500G (90% usado)

Rede:
  eth0: 192.168.1.100
  
Uptime: 5 dias, 3 horas
```

---

### backup.sh

Cria backup de arquivos/diretórios com timestamp.

**Uso:**
```bash
backup <origem> [destino]
```

**Comportamento:**
- Sem destino: cria `.tar.gz` no diretório atual
- Com destino: copia para local especificado
- Nome automático: `backup_YYYY-MM-DD_HH-MM-SS.tar.gz`
- Exibe tamanho do backup

**Exemplos:**
```bash
# Backup de pasta para .tar.gz
backup ~/Documents

# Backup de arquivo específico
backup ~/important.txt

# Backup para local específico
backup ~/projeto ~/backups/
```

**Saída:**
```
Criando backup...
Backup criado: backup_2026-01-24_13-45-30.tar.gz (15 MB)
```

---

### cleanup.sh

Limpa cache, logs e arquivos temporários do sistema.

**Uso:**
```bash
cleanup
```

**Ações realizadas:**
- Limpa cache do APT (se Ubuntu/Debian)
- Remove logs antigos (>30 dias) de /var/log
- Esvazia lixeira (~/.local/share/Trash)
- Remove arquivos temporários (/tmp)
- Exibe espaço liberado

**Saída:**
```
Limpando cache do APT...
Limpando logs antigos...
Limpando lixeira...
Limpando arquivos temporários...

LIMPEZA CONCLUÍDA
Espaço liberado: 2.5 GB
```

**Segurança:**
- Requer sudo para logs do sistema
- Preserva logs recentes
- Não remove arquivos importantes

---

### gitstat.sh

Exibe estatísticas detalhadas de repositório Git.

**Uso:**
```bash
gitstat
```

**Pré-requisito:** Estar em um diretório Git.

**Informações exibidas:**
- Nome do repositório
- URL remota
- Branch atual
- Total de commits
- Commits por autor (top 5)
- Linhas de código por linguagem
- Arquivos modificados recentemente
- Status atual (staged, modified, untracked)

**Exemplo de saída:**
```
Repositório: clios-shell
Remote: git@github.com:usuario/clios-shell.git
Branch: main

Commits:
  Total: 157
  Últimos 7 dias: 23

Top 5 Autores:
  João Silva: 89 commits
  Maria Santos: 45 commits
  Pedro Souza: 23 commits

Linhas de Código:
  Rust: 8,542 linhas
  Markdown: 1,234 linhas
  Shell: 456 linhas

Status atual:
  M  src/main.rs
  A  novo_arquivo.rs
  ?? pasta_nova/
```

---

## Aliases Pré-configurados

Definidos em `~/.cliosrc`.

### Navegação

```bash
..        cd ..
...       cd ../..
....      cd ../../..
~         cd ~
-         cd -
```

### Listagem

```bash
ls        ls --color=auto
ll        ls -lah
la        ls -A
l         ls -CF
lt        ls -lath          # Por data modificação
lsize     ls -laSh          # Por tamanho
```

### Git (24 aliases)

```bash
# Status e Info
gs        git status
gl        git log --oneline --graph --decorate
gd        git diff
gdc       git diff --cached

# Add e Commit
ga        git add
gaa       git add --all
gc        git commit -m
gca       git commit --amend

# Branch
gb        git branch
gba       git branch -a
gbd       git branch -d
gco       git checkout
gcb       git checkout -b

# Remote
gp        git push
gpl       git pull
gf        git fetch
gm        git merge

# Outros
gst       git stash
gstp      git stash pop
gcl       git clone
grst      git reset
gundo     git reset --soft HEAD~1
```

### Rust/Cargo (11 aliases)

```bash
cb        cargo build
cr        cargo run
ct        cargo test
cc        cargo check
cw        cargo watch -x run
crel      cargo build --release
cdo       cargo doc --open
cfmt      cargo fmt
cclean    cargo clean
cupdate   cargo update
```

### Python

```bash
py        python3
pip       python3 -m pip
venv      python3 -m venv venv
activate  source venv/bin/activate
```

### Node.js/NPM

```bash
ni        npm install
nid       npm install --save-dev
nr        npm run
ns        npm start
nt        npm test
```

### Docker

```bash
dps       docker ps
dpsa      docker ps -a
di        docker images
dex       docker exec -it
dlog      docker logs -f
dstop     docker stop
drm       docker rm
```

### Sistema

```bash
update    sudo apt update && sudo apt upgrade -y
install   sudo apt install
remove    sudo apt remove
search    apt search
cleanup   sudo apt autoremove && sudo apt autoclean

ports     sudo netstat -tulpn
```

### Utilitários

```bash
h         history
c         clear
e         exit
reload    source ~/.cliosrc

mkcd      mkdir -p "$1" && cd "$1"    # Cria e entra
extract   # Extrai qualquer arquivo compactado
```

---

## Configuração

### Arquivo ~/.cliosrc

Arquivo de configuração principal executado ao iniciar.

**Localização:** `~/.cliosrc`

**Seções:**

#### Variáveis de Ambiente
```bash
export EDITOR=vim
export BROWSER=firefox
export LANG=en_US.UTF-8
```

#### Carregamento de Plugins
```bash
source ~/.clios_plugins/utils.rhai
source ~/.clios_plugins/git_helpers.rhai
source ~/.clios_plugins/dev_tools.rhai
```

#### Aliases
```bash
alias ll='ls -lah'
alias gs='git status'
# ... 98 aliases no total
```

#### Scripts
```bash
alias sysinfo='~/.clios_scripts/sysinfo.sh'
alias backup='~/.clios_scripts/backup.sh'
alias cleanup='~/.clios_scripts/cleanup.sh'
alias gitstat='~/.clios_scripts/gitstat.sh'
```

---

### Arquivo ~/.clios.toml (Opcional)

Configuração avançada do prompt e temas.

**Localização:** `~/.clios.toml`

**Exemplo completo:**
```toml
# Tema do prompt (powerline ou classic)
theme = "powerline"

[prompt]
symbol = "➜"
color = "blue"
path_color = "cyan"
symbol_color = "green"
show_git = true

[history]
file = ".clios_history"
max_entries = 5000

[syntax]
valid_cmd = "green"
invalid_cmd = "red"
```

**Opções:**

#### [prompt]
- `symbol` - Símbolo do prompt (>, $, ➜, etc.)
- `color` - Cor padrão (red, green, blue, purple, cyan, yellow, white)
- `path_color` - Cor do caminho
- `symbol_color` - Cor do símbolo
- `show_git` - Mostrar branch Git (true/false)

#### [history]
- `file` - Nome do arquivo de histórico
- `max_entries` - Máximo de comandos salvos

#### [syntax]
- `valid_cmd` - Cor para comandos válidos
- `invalid_cmd` - Cor para comandos inválidos

---

## Exemplos de Uso

### Cenário 1: Desenvolvimento Rust

```bash
# Criar novo projeto
cargo new meu_projeto
cd meu_projeto

# Gerar código com plugin
rhai rust_fn("calcular_media", "numeros: Vec<i32>", "f64")

# Desenvolver
cb              # cargo build
ct              # cargo test
cw              # cargo watch -x run

# Commit
ga .
gc "feat: adiciona cálculo de média"
gp
```

### Cenário 2: Análise de Sistema

```bash
# Ver informações
sysinfo

# Verificar portas
ports

# Limpeza
cleanup

# Backup antes de atualização
backup ~/important_data
update
```

### Cenário 3: Git Workflow

```bash
# Status
gs

# Ver histórico visual
gl

# Nova feature
gcb feature/nova-funcao

# Trabalhar
# ... edições ...
gaa
gc "feat: implementa nova função"

# Merge
gco main
gm feature/nova-funcao
gp

# Estatísticas
gitstat
```

### Cenário 4: Automação com Rhai

```bash
# Script de deploy automatizado
rhai
```

```rhai
// Confirmar ação
if !confirm("Fazer deploy para produção?") {
    print("Deploy cancelado");
    return;
}

// Escolher ambiente
let envs = ["staging", "production"];
let env = select("Escolha o ambiente:", envs);

// Executar testes
let tests = shell_exec("cargo test");
if !tests.success {
    print("Testes falharam!");
    return;
}

// Build
let build = shell_exec("cargo build --release");
if !build.success {
    print("Build falhou!");
    return;
}

// Deploy
let deploy_cmd = "scp target/release/app user@server:/opt/";
let result = shell_exec(deploy_cmd);

if result.success {
    print("Deploy realizado com sucesso!");
    
    // Salvar log
    save_file("/tmp/deploy.log", "Deploy " + env + " em " + timestamp());
} else {
    print("Erro no deploy: " + result.output);
}
```

### Cenário 5: Pipeline Complexo

```bash
# Análise de logs
cat /var/log/syslog | grep "error" | \
  awk '{print $5}' | sort | uniq -c | \
  sort -nr > erros_top.txt

# Backup condicionado
ls -la | wc -l && backup ./ && echo "Backup completo"

# Processamento com substituição
echo "Total de arquivos Rust: $(find . -name "*.rs" | wc -l)"

# Multi-stage com variáveis
export PROJETO=clios
cd ~/${PROJETO} && \
  cargo build --release && \
  strip target/release/${PROJETO} && \
  ls -lh target/release/${PROJETO}
```

### Cenário 6: Criação de Aliases Dinâmicos

```bash
# Alias simples
alias dev='cd ~/Desenvolvimento'

# Alias com pipeline
alias findrs='find . -name "*.rs" | grep -v target'

# Alias com lógica
alias testall='cargo test && cargo clippy && cargo fmt --check'

# Alias com função
alias mkproj='mkdir -p $1/src && cd $1 && git init'
```

---

## Capacidades Avançadas

### Job Control Completo

```bash
# Executar em background
sleep 100 &
[Background Job 12345]

# Pausar com Ctrl+Z
long_running_command
^Z
[Job 12346] Pausado

# Retomar
fg 12346
```

### Expansões Combinadas

```bash
# Múltiplas expansões na mesma linha
echo "User $USER em $(hostname) às $(date +%H:%M)"
cp ~/$PROJETO/*.rs /backup/$(date +%Y%m%d)/

# Globs com variáveis
export EXT=rs
ls *.$EXT
```

### Redirecionamento Avançado

```bash
# Separar stdout e stderr
comando > output.txt 2> errors.txt

# Append misto
comando >> output.txt 2> errors.txt

# Pipeline com redirecionamento
cat arquivo.txt | grep "palavra" > resultado.txt 2> erros.txt
```

### Proteções e Tratamento de Erros

#### Alias Recursivo
```bash
# Detecta e previne
alias ls=ls      # Aviso: usa comando original
ls               # Funciona normalmente
```

#### Comandos Vazios
```bash
# Ignorados silenciosamente
         # <enter>
&&       # Ignorado
```

#### Subshells Malformados
```bash
echo $(echo test   # Erro: Subshell não fechado
echo $()           # Aviso: Subshell vazio
```

#### Redirecionamento Inválido
```bash
echo test >                    # Erro: arquivo requerido
echo test > /root/file.txt     # Erro: permissão negada
```

---

## Formato de Comandos para IA

Para que uma IA possa executar comandos na Clios Shell através de linguagem natural, use este formato:

### Template de Interpretação

**Entrada do Usuário (Natural):**
"Liste todos os arquivos Rust do projeto"

**Comando Clios Gerado:**
```bash
ls *.rs
# ou
find . -name "*.rs"
# ou usando alias
findrs
```

**Entrada do Usuário:**
"Faça backup do diretório atual e depois mostre informações do sistema"

**Comando Clios Gerado:**
```bash
backup ./ && sysinfo
```

**Entrada do Usuário:**
"Execute os testes, e se passarem, faça o build de release"

**Comando Clios Gerado:**
```bash
cargo test && cargo build --release
# ou usando aliases
ct && crel
```

### Mapeamento de Intenções

| Intenção | Comando Clios |
|----------|---------------|
| Listar arquivos | `ls`, `ll`, `la` |
| Navegar | `cd [dir]` |
| Buscar | `find`, `grep` |
| Informações sistema | `sysinfo` |
| Status Git | `gs` ou `git status` |
| Executar testes | `ct` ou `cargo test` |
| Compilar | `cb` ou `cargo build` |
| Backup | `backup [origem]` |
| Limpar sistema | `cleanup` |
| Ver logs | `cat /var/log/...` ou `docker logs` |
| Instalar pacote | `install [pacote]` |
| Criar função Rust | `rhai rust_fn(...)` |
| Executar script | `rhai` + código Rhai |

---

## Proteções e Limites

### Proteções Implementadas

1. **Alias Recursivo:** Máximo 10 níveis, detecção de loops
2. **Comandos Vazios:** Ignorados silenciosamente
3. **Subshells:** Validação de fechamento, detecção de vazios
4. **Redirecionamento:** Validação de arquivos e permissões
5. **Plugins:** Validação de sintaxe Rhai antes do carregamento

### Limites de Sistema

- **Histórico:** 1000 entradas (configurável)
- **Recursão de Alias:** 10 níveis
- **Complexidade Rhai:** Limite de engine (switch recomendado para grandes if/else)
- **Pipeline:** Limitado pela memória disponível

---

## Referência Rápida de Comandos

### Essenciais
```bash
help                    # Ajuda completa
version                 # Versão da shell
cd [dir]                # Navegar
pwd                     # Diretório atual
ls / ll / la            # Listar arquivos
history                 # Histórico
exit                    # Sair
```

### Aliases
```bash
alias                   # Listar todos
alias nome='cmd'        # Criar
unalias nome            # Remover
type comando            # Tipo de comando
```

### Git
```bash
gs / gst / gstat        # Status
gl                      # Log gráfico
ga / gaa                # Add
gc "msg"                # Commit
gp / gpl                # Push/Pull
gcb nome                # Nova branch
```

### Rust/Cargo
```bash
cb / cr / ct            # Build/Run/Test
cc / cw                 # Check/Watch
crel                    # Release build
cfmt                    # Format
```

### Plugins Rhai
```bash
source arquivo.rhai     # Carregar
plugins                 # Listar
rhai código             # Executar
rhai                    # Modo REPL
```

### Utilitários
```bash
sysinfo                 # Info sistema
backup [origem]         # Backup
cleanup                 # Limpar
gitstat                 # Estatísticas Git
```

---

## Conclusão

Clios Shell v1.0.0 é uma shell completa e moderna que combina:

- **45+ comandos** entre builtins, aliases e scripts
- **30+ funções Rhai** distribuídas em 3 plugins
- **Pipeline, redirecionamento e lógica** completa
- **Job control** Unix nativo
- **Sistema de plugins** extensível
- **98 aliases** pré-configurados
- **4 scripts utilitários** prontos para uso

Esta documentação cobre todas as capacidades para permitir que uma IA:
1. Interprete comandos em linguagem natural
2. Gere comandos Clios corretos
3. Combine múltiplas funcionalidades
4. Utilize plugins e scripts
5. Execute tarefas complexas

**Versão da Documentação:** 1.0.0  
**Data:** 2026-01-24  
**Desenvolvedor:** Pedro Galvani Bertasso  
**Licença:** MIT

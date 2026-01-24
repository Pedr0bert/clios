# Clios Shell 

> A Hybrid Rust + Rhai System Shell for Embedded Linux & Power Users.

**Clios** (Command Line Interface & Operating System Shell) é uma shell moderna, escrita em Rust, projetada para ser leve, rápida e extensível via scripts. Ela combina a performance de sistemas nativos com a flexibilidade da linguagem de script [Rhai](https://rhai.rs).

![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)
![Rhai](https://img.shields.io/badge/scripting-Rhai-brightgreen.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Tests](https://img.shields.io/badge/tests-45_passing-brightgreen.svg)
![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)

##  Funcionalidades (The 10 Levels)

O desenvolvimento do Clios seguiu um roadmap de 10 níveis de complexidade de sistemas operacionais:

- [x] **Execução de Comandos:** Roda binários do sistema (`ls`, `grep`, `git`).
- [x] **Histórico Persistente:** Salva comandos em `~/.clios_history`.
- [x] **Sintaxe Colorida:** Realce de sintaxe em tempo real (Verde = Válido, Vermelho = Inválido).
- [x] **Job Control:** Suporte a background (`&`), `Ctrl+Z` e comando `fg`.
- [x] **Scripting Avançado:** Integração nativa com a linguagem Rhai.
- [x] **Lógica Condicional:** Suporte a operadores `&&` e `||`.
- [x] **Git Aware:** Prompt mostra a branch atual automaticamente.
- [x] **I/O Redirection:** Suporte a `>` (overwrite), `>>` (append) e `2>` (stderr).
- [x] **Pipes:** Encanamento de processos via memória (`|`).
- [x] **Context Awareness:** Parser inteligente que respeita aspas em argumentos.
- [x] ** Proteções Robustas:** Sistema avançado de detecção e prevenção de erros.

##  Proteções e Melhorias (v1.0.0)

A shell Clios inclui proteções avançadas contra erros comuns:

-  **Proteção contra Aliases Recursivos:** Detecta e previne loops infinitos (limite: 10 níveis)
-  **Parser Inteligente de Pipes:** Respeita aspas ao dividir comandos em pipeline
-  **Validação de Comandos Vazios:** Ignora silenciosamente entradas vazias
-  **Tratamento de Subshells:** Erros claros para `$()` mal-formados
-  **Validação de Redirecionamento:** Mensagens claras quando arquivos não podem ser abertos
-  **Verificação de Plugins:** Validação completa ao carregar arquivos `.rhai`
-  **Mensagens de Erro Padronizadas:** Sistema consistente com códigos de erro Unix

##  Comandos Internos (Builtins)

| Comando | Descrição |
|---------|-----------|
| `cd [dir]` | Mudar diretório (cd - para anterior) |
| `pwd` | Exibir diretório atual |
| `alias [name='cmd']` | Criar/listar aliases |
| `unalias <name>` | Remover alias |
| `export VAR=val` | Definir variável de ambiente |
| `unset VAR` | Remover variável de ambiente |
| `source <file>` | Carregar plugin Rhai |
| `plugins` | Listar plugins carregados |
| `rhai [código]` | Executar código Rhai |
| `history` | Exibir histórico |
| `type <cmd>` | Mostrar tipo do comando |
| `fg <PID>` | Trazer processo para foreground |
| `version` | Exibir versão |
| `help` | Exibir ajuda completa |
| `exit` | Sair da shell |

##  Instalação

### Método Rápido (Recomendado)

```bash
# Clone o repositório
git clone https://github.com/pedrohusky/clios-shell
cd clios-shell

# Execute o instalador
./install.sh
```

O script de instalação irá:
1.  Compilar o Clios em modo Release (otimizado)
2.  Instalar as configurações em `~/.cliosrc`
3.  Instalar 3 plugins Rhai em `~/.clios_plugins/`
4.  Instalar 4 scripts utilitários em `~/.clios_scripts/`
5.  Opcionalmente instalar o binário em `/usr/local/bin/`

### Pré-requisitos
- **Rust (Cargo)** - [Instale aqui](https://rustup.rs)
- **Linux ou WSL** (macOS também suportado)

### Instalação Manual

```bash
# Clone o repositório
git clone https://github.com/pedrohusky/clios-shell
cd clios-shell

# Compile em modo Release
cargo build --release

# Copie as configurações
cp config/cliosrc ~/.cliosrc
cp -r config/plugins ~/.clios_plugins
cp -r config/scripts ~/.clios_scripts

# (Opcional) Instale globalmente
sudo install target/release/clios-shell /usr/local/bin/clios
```

##  Estrutura de Configuração

```
~/.cliosrc              # Configuração principal (aliases, variáveis, plugins)
~/.clios_plugins/       # Plugins Rhai
├── utils.rhai          # Funções utilitárias (upper, lower, sum, avg, etc.)
├── git_helpers.rhai    # Helpers para Git (commit_msg, branch_name, etc.)
└── dev_tools.rhai      # Ferramentas de dev (rust_fn, http_codes, etc.)
~/.clios_scripts/       # Scripts shell utilitários
├── sysinfo.sh          # Informações do sistema
├── backup.sh           # Backup de arquivos
├── cleanup.sh          # Limpeza de cache/logs
└── gitstat.sh          # Estatísticas Git
```

##  Testes

A shell Clios possui uma suite completa de testes:

```bash
# Testes automatizados (26 testes de integração)
./test_shell.sh

# Testes unitários Rust (19 testes)
cargo test

# Compilar em modo release
cargo build --release
```

**Resultados Atuais:**
-  26/26 testes de integração passaram (100%)
-  19/19 testes unitários Rust passaram (100%)
-  45 testes totais
-  0 crashes detectados
-  Compilação sem erros ou warnings

##  Plugins Rhai

O Clios suporta plugins escritos em [Rhai](https://rhai.rs). As funções dos plugins ficam disponíveis diretamente no comando `rhai`:

```bash
# Carregar um plugin
source ~/.clios_plugins/utils.rhai

# Usar funções do plugin
rhai upper("hello")          # → "HELLO"
rhai sum([1, 2, 3, 4, 5])    # → 15
rhai http_codes()            # → Tabela de códigos HTTP
rhai git_cheatsheet()        # → Comandos Git úteis
```

### Plugins Incluídos

| Plugin | Funções Disponíveis |
|--------|---------------------|
| **utils.rhai** | `upper()`, `lower()`, `capitalize()`, `sum()`, `avg()`, `factorial()`, `reverse_array()`, `unique()` |
| **git_helpers.rhai** | `commit_msg()`, `commit_types()`, `branch_name()`, `git_cheatsheet()`, `git_flow()` |
| **dev_tools.rhai** | `rust_fn()`, `rust_struct()`, `rust_test()`, `http_status()`, `http_codes()`, `lorem()` |

##  Scripts Utilitários

Scripts shell prontos para uso via aliases:

```bash
# Informações do sistema
sysinfo

# Backup de diretório
backup ~/Documents

# Limpeza de cache/logs
cleanup

# Estatísticas Git do repositório
gitstat
```
-  Compilação sem erros ou warnings

##  Documentação

- **[GUIA_DEPURACAO.md](GUIA_DEPURACAO.md)** - Guia completo com comandos de teste manual
- **[RELATORIO_MELHORIAS.md](RELATORIO_MELHORIAS.md)** - Relatório detalhado de todas as melhorias
- **[test_shell.sh](test_shell.sh)** - Script de testes automatizado

##  Uso Rápido

```bash
# Modo interativo
./target/debug/clios-shell

# Executar comando único
./target/debug/clios-shell -c "echo Hello World"

# Executar script
./target/debug/clios-shell script.sh
```

---

**Status:**  Pronto para produção | **Versão:** 1.0 Final Release | **Testes:** 100% passando

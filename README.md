# Clios Shell 🚀

> A Hybrid Rust + Rhai System Shell for Embedded Linux & Power Users.

**Clios** (Command Line Interface & Operating System Shell) é uma shell moderna, escrita em Rust, projetada para ser leve, rápida e extensível via scripts. Ela combina a performance de sistemas nativos com a flexibilidade da linguagem de script [Rhai](https://rhai.rs).

![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)
![Rhai](https://img.shields.io/badge/scripting-Rhai-brightgreen.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## ✨ Funcionalidades (The 10 Levels)

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

## 📦 Instalação

### Pré-requisitos
- Rust (Cargo) instalado.
- Ambiente Linux ou WSL.

### Compilando

```bash
# Clone o repositório (se aplicável)
git clone [https://github.com/seu-usuario/clios-shell](https://github.com/seu-usuario/clios-shell)
cd clios-shell

# Compile em modo Release (Otimizado)
cargo build --release

# Instalação Manual (ou use o alias se já configurado)
sudo install target/release/clios-shell /usr/local/bin/clios
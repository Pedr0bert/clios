# Changelog - Clios Shell

## [1.0.0] - 2025-01-24

###  Novos Recursos
- **Comando `help`**: Exibe ajuda completa com formatação visual em box ANSI
- **Comando `version`**: Exibe versão da shell
- **Comando `type`**: Mostra tipo do comando (builtin, alias, ou executável externo)
- **Comando `unalias`**: Remove aliases definidos
- **Comando `unset`**: Remove variáveis de ambiente

###  Melhorias de Robustez
- **Proteção contra Aliases Recursivos**: Sistema de detecção com limite de 10 níveis de profundidade
- **Parser Inteligente de Pipes**: Nova função `split_pipes_respecting_quotes()` que respeita aspas
- **Validação de Subshells**: Tratamento de erro claro para `$()` mal-formados
- **Validação de Redirecionamento**: Mensagens específicas por tipo de erro (arquivo não encontrado, permissão negada)
- **Validação de Plugins**: Verificação completa ao carregar arquivos `.rhai`
- **Códigos de Erro Padronizados**: `EXIT_COMMAND_NOT_FOUND=127`, `EXIT_ERROR=1`

###  Melhorias de Código
- **Correções Clippy**: 24 warnings corrigidos
  - Refatoração de `if` aninhados para formato colapsado
  - Uso de `.flatten()` em iteradores
  - Remoção de `return` desnecessários
  - Uso de `.unwrap_or_default()` onde apropriado
  - Remoção de imports não utilizados
- **Documentação**: Headers descritivos em todos os módulos principais
- **Constantes**: Uso de constantes ANSI para cores
- **Idiomático**: Código mais idiomático seguindo as melhores práticas Rust

###  Documentação
- **README.md**: Atualizado com tabela de comandos e badges
- **GUIA_DEPURACAO.md**: Guia completo de testes manuais
- **RELATORIO_MELHORIAS.md**: Relatório detalhado de todas as melhorias

###  Testes
- **26 testes de integração** (test_shell.sh)
- **19 testes unitários Rust** (src/tests.rs)
- **0 falhas detectadas**
- **0 warnings de compilação**

###  Estatísticas
- Arquivos modificados: 8
- Testes totais: 45
- Warnings Clippy: 0
- Erros de compilação: 0

---

## Arquitetura do Projeto

```
src/
├── main.rs           # Ponto de entrada, parsing de argumentos
├── shell.rs          # Estado da shell, REPL principal
├── builtins.rs       # Comandos internos (cd, export, alias, etc.)
├── expansion.rs      # Expansão de variáveis, aliases, globs, subshells
├── pipeline.rs       # Execução de comandos e pipelines
├── prompt.rs         # Geração do prompt (Git, versões)
├── config.rs         # Carregamento de configuração TOML
├── jobs.rs           # Controle de jobs (background)
├── completion.rs     # Autocompletar
├── rhai_integration.rs # Integração com Rhai scripting
└── tests.rs          # Testes unitários
```

## Verificação de Qualidade

```bash
# Sem warnings
cargo clippy  #  0 warnings

# Todos os testes passam
cargo test    #  19/19 passed
./test_shell.sh  #  26/26 passed

# Build release funcional
cargo build --release  #  Sucesso
```

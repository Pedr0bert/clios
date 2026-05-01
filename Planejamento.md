# Plano de Evolução do Clios Shell (Rumo à Maturidade POSIX)

Este documento detalha o planejamento arquitetural e as etapas de refatoração para elevar o nível de robustez, compatibilidade POSIX e corretude do **Clios Shell**.

## Fase 1: Refatoração do Lexer e Parser (A Fundação)
- **Objetivo:** Abandonar avaliações lineares focadas em manipulação direta de *strings* e estabelecer uma arquitetura de Árvore de Sintaxe Abstrata (AST).
- **Ações:**
  - **Tokenização Pura (Lexer):** Construir um *Lexer* estrito que identifique a entrada como agrupamentos lógicos restritos (`Word`, `Pipe(|)`, `Background(&)`, `Semicolon(;)`, `AndIf(&&)`, `OrIf(||)`, `RedirectOn(>)`). Separar escopos léxicos de strings ("") e literais ('').
  - **Construção da Árvore (AST Parser):** Criar um analisador sintático que consuma os tokens do *Lexer* e monte uma árvore relacional válida, respeitando a precedência de agrupamentos lógicos (ex: execução de subshells, *pipelines* agrupados dentro de condicionais relacionais).

## Fase 2: Robustez de Erros e Controle de Fluxo
- **Objetivo:** Garantir comportamentos condicionais e de erro idênticos ou superiores a shells tradicionais.
- **Ações:**
  - **Avaliação de Curto-Circuito (`||` e `&&`):** Alterar despachos de erros críticos ao buscar o binário no *Path*. Em vez de abortar toda a linha de execução e gerar um alerta bruto do sistema, o motor deve devolver graciosamente um Return Code/Exit Status > 0 (ex: código padrão `127` para Not Found), acionando devidamente as chaves lógicas da AST e permitindo que a ramificação alternativa siga seu curso.
  - **Dangling Pipes (`|` abandonado):** Incluir validação no nível do parser e levantar uma rejeição estrita (Panic amigável Sintático) quando um nó *Pipe* carecer de um destino na AST, prevenindo ações fantasma de I/O na máquina.
  - **Separação I/O Consolidada:** Modularizar manipulação para tratar stdout, stderr, stdin nativamente em Rust na montagem do processo (tratamento independente para casos como `2>&1`, herança de fds).

## Fase 3: Operadores de Separação e Concorrência Inline
- **Objetivo:** Processar adequadamente múltiplos comandos sequenciais ou paralelos descritos numa mesma linha.
- **Ações:**
  - **Implementar o Token Semicolon `;`:** Ensinar a máquina de estado da AST a agendar a execução bloqueante do processo/nó da esquerda, aguardar seu sinal de fim/saída e descer para continuar a avaliação imediata do nó à sua direita, reiniciando o estado.
  - **O Verdadeiro Comportamento do Ampersand `&`:** Tokenizar o `&` de forma contextual (além  do apenas "fim de linha"). Se ele dividir comandos: `cmd1 & cmd2`, bifurcar rapidamente a thread/processo de `cmd1`, adicionar internamente à lista de Background Jobs e prosseguir o parser principal na linha avaliando/rodando o `cmd2` no foreground.

## Fase 4: Motor Recursivo de Expansão (Mighty Expansions)
- **Objetivo:** Suportar aninhamentos intrincados e interpolações avançadas, como substituições triplas (`echo $(echo $(whoami))`) de power-users.
- **Ações:**
  - **Algoritmo de Balanceamento de Escopo Numérico:** Substituir a busca global por expressões regulares para um laço iterário com pilha (Stack) que identifique e agrupe a profundidade de chamadas aos parênteses `()` não "escapados".
  - **Expansão Dinâmica Bottom-Up:** Fazer a rotina de execução da AST re-invocar o analisador do Clios internamente quando deparado por blocos de substituição `$(...)`, fatiando o miolo extraído e reintroduzindo com segurança o Stdout resultante no nó superior gerador.
  - **Parameter Expansions (Extensões Adicionais):** Suportar manipulação variável comum do bash ex: `${VAR:-default}`, `${VAR%pattern}` para permitir portabilidade dos scripts shell utilitários (ex: de init de sistema ou provisionamento de docker) para o Clios Shell.

## Fase 5: Integração Total com Sistema de Scripting Nativo (Rhai)
- **Objetivo:** Reduzir o atrito entre o executor do SO do Clios e os scripts avançados escritos em Rhai, unificando a experiência.
- **Ações:**
  - **APIs de Processos no Rhai:** Expor funções e classes nativas do Rust dentro da engine Rhai do Clios para permitir criar jobs de SO, verificar retornos de chamadas do sistema e manipular fd redirecionados sem precisar abusar de eval() desajeitados.
  - **Catch Sinais de Sistema:** Permitir registro nativo de Handlers de eventos em Rhai contra as armadilhas posix `SIGINT` (Ctrl+C), `SIGTERM` e afins para limpezas (teardowns).

## Fase 6: Cúpula de Prevenção (Test-Driven Shell)
- **Objetivo:** Proteger iterativamente o esforço alcançado das fases anteriores, blindando contra regressões com novos PRs ou expansões em Rust.
- **Ações:**
  - **Bateria Intensa e Testes de Integração E2E:** Rodar o binário contra os arquivos de conformidade de parser e testar agressivamente grandes amontoados de pipelines assíncronos que usam redirecionamentos longos.
  - **Fuzzing de Sintaxe (Lexer Stress):** Alimentar randomicamente ao Clios Shell strings sujas contendo escapes esquisitos `\`, aspas mescladas não fachadas e quebras de linha abruptas para comprovar a resiliência e a não-ocorrência de 'panics!' abruptas no terminal do usuário.
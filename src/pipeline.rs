//! # Pipeline Module
//!
//! Handles pipeline execution, I/O redirection, and process management.
//!
//! ## Funcionalidades
//! - Execução de pipelines de comandos (`cmd1 | cmd2 | cmd3`)
//! - Redirecionamento de I/O (`>`, `>>`, `2>`, `2>>`)
//! - Gerenciamento de processos filhos

use std::fs::{File, OpenOptions};
use std::process::{Child, Command, Stdio};

// -----------------------------------------------------------------------------
// CONSTANTES
// -----------------------------------------------------------------------------

/// Código de saída padrão POSIX para "comando não encontrado"
const EXIT_COMMAND_NOT_FOUND: i32 = 127;

/// Código de saída para erros genéricos
const EXIT_ERROR: i32 = 1;

// -----------------------------------------------------------------------------
// I/O REDIRECTION PARSING
// -----------------------------------------------------------------------------

/// Analisa e processa operadores de redirecionamento de I/O (Nível 9).
///
/// Esta função percorre a lista de tokens procurando por operadores especiais
/// de redirecionamento. Quando encontra, ela abre o arquivo correspondente
/// e o remove da lista de argumentos do comando.
///
/// # Operadores Suportados
/// * `<`   : Redireciona **STDIN** (Lê do arquivo).
/// * `>`   : Redireciona **STDOUT** (Sobrescreve o arquivo).
/// * `>>`  : Redireciona **STDOUT** (Adiciona ao final do arquivo - Append).
/// * `2>`  : Redireciona **STDERR** (Sobrescreve o arquivo).
/// * `2>>` : Redireciona **STDERR** (Adiciona ao final do arquivo - Append).
///
/// # Retorno
/// Retorna uma tupla `(Vec<String>, Option<File>, Option<File>, Option<File>)`:
/// 1. **Argumentos Limpos:** O comando sem os símbolos de redirecionamento.
/// 2. **Arquivo Entrada:** O arquivo aberto para onde vem o stdin (se houver).
/// 3. **Arquivo Saída:** O arquivo aberto para onde vai o stdout (se houver).
/// 4. **Arquivo Erro:** O arquivo aberto para onde vai o stderr (se houver).
pub fn parse_redirection(tokens: &[String]) -> (Vec<String>, Option<File>, Option<File>, Option<File>) {
    let mut clean = Vec::new();
    let mut stdin_file = None;
    let mut stdout_file = None;
    let mut stderr_file = None;

    let mut iter = tokens.iter().peekable();

    while let Some(t) = iter.next() {
        match t.as_str() {
            // Entrada Padrão (Read)
            "<" => {
                if let Some(f) = iter.next() {
                    match File::open(f) {
                        Ok(o) => stdin_file = Some(o),
                        Err(e) => {
                            eprintln!("\x1b[1;31m[ERRO REDIRECIONAMENTO]\x1b[0m Falha ao abrir '{}': {}", f, e);
                        }
                    }
                } else {
                    eprintln!("\x1b[1;31m[ERRO SINTAXE]\x1b[0m Operador '<' requer um arquivo");
                }
            }
            // Saída Padrão (Overwrite)
            ">" => {
                if let Some(f) = iter.next() {
                    match OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(f)
                    {
                        Ok(o) => stdout_file = Some(o),
                        Err(e) => {
                            eprintln!("\x1b[1;31m[ERRO REDIRECIONAMENTO]\x1b[0m Falha ao abrir '{}': {}", f, e);
                        }
                    }
                } else {
                    eprintln!("\x1b[1;31m[ERRO SINTAXE]\x1b[0m Operador '>' requer um arquivo");
                }
            }
            // Saída Padrão (Append)
            ">>" => {
                if let Some(f) = iter.next() {
                    match OpenOptions::new()
                        
                        .create(true)
                        .append(true)
                        .open(f)
                    {
                        Ok(o) => stdout_file = Some(o),
                        Err(e) => {
                            eprintln!("\x1b[1;31m[ERRO REDIRECIONAMENTO]\x1b[0m Falha ao abrir '{}': {}", f, e);
                        }
                    }
                } else {
                    eprintln!("\x1b[1;31m[ERRO SINTAXE]\x1b[0m Operador '>>' requer um arquivo");
                }
            }
            // Saída de Erro (Overwrite)
            "2>" => {
                if let Some(f) = iter.next() {
                    match OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(f)
                    {
                        Ok(o) => stderr_file = Some(o),
                        Err(e) => {
                            eprintln!("\x1b[1;31m[ERRO REDIRECIONAMENTO]\x1b[0m Falha ao abrir '{}': {}", f, e);
                        }
                    }
                } else {
                    eprintln!("\x1b[1;31m[ERRO SINTAXE]\x1b[0m Operador '2>' requer um arquivo");
                }
            }
            // Saída de Erro (Append)
            "2>>" => {
                if let Some(f) = iter.next() {
                    match OpenOptions::new()
                        
                        .create(true)
                        .append(true)
                        .open(f)
                    {
                        Ok(o) => stderr_file = Some(o),
                        Err(e) => {
                            eprintln!("\x1b[1;31m[ERRO REDIRECIONAMENTO]\x1b[0m Falha ao abrir '{}': {}", f, e);
                        }
                    }
                } else {
                    eprintln!("\x1b[1;31m[ERRO SINTAXE]\x1b[0m Operador '2>>' requer um arquivo");
                }
            }
            // Token normal
            _ => clean.push(t.clone()),
        }
    }
    (clean, stdin_file, stdout_file, stderr_file)
}

// -----------------------------------------------------------------------------
// PIPELINE EXECUTION
// -----------------------------------------------------------------------------

/// Executa uma "Pipeline" de comandos (ex: `ls | grep txt | wc -l`).
///
/// Esta é a função que realmente faz os programas rodarem. Ela gerencia:
/// 1. **Pipes:** Conecta a saída de um comando na entrada do próximo.
/// 2. **Redirecionamento:** Conecta arquivos (`>`, `2>`) se necessário.
/// 3. **Exit Codes:** Captura se o comando deu certo ou errado.
///
/// # Como funciona (The Daisy Chain)
/// Em um pipe `A | B | C`:
/// * **A**: Stdin = Teclado, Stdout = Pipe(A->B)
/// * **B**: Stdin = Pipe(A->B), Stdout = Pipe(B->C)
/// * **C**: Stdin = Pipe(B->C), Stdout = Tela
pub fn execute_pipeline(commands: Vec<Vec<String>>) -> i32 {
    // Validação: pipeline vazio
    if commands.is_empty() {
        return 0;
    }

    // Validação: todos os comandos estão vazios
    if commands.iter().all(|cmd| cmd.is_empty()) {
        eprintln!("\x1b[1;33m[AVISO]\x1b[0m Pipeline vazio detectado");
        return 0;
    }

    let mut prev_cmd: Option<Child> = None;
    let mut final_exit_code = 0;

    for (i, tokens) in commands.iter().enumerate() {
        if tokens.is_empty() {
            eprintln!("\x1b[1;33m[AVISO]\x1b[0m Comando vazio no pipeline (posição {})", i + 1);
            continue;
        }

        // 1. Separa o comando dos redirecionamentos de arquivo
        let (mut args, infile, outfile, errfile) = parse_redirection(tokens);

        if args.is_empty() {
            continue;
        }

        let cmd = args.remove(0);

        // 2. Configuração do STDIN
        let stdin = if let Some(f) = infile {
            // Redirecionamento de entrada tem prioridade
            Stdio::from(f)
        } else if let Some(mut child) = prev_cmd {
            Stdio::from(child.stdout.take().unwrap())
        } else {
            Stdio::inherit()
        };

        // 3. Configuração do STDOUT
        let stdout = if let Some(f) = outfile {
            Stdio::from(f)
        } else if i < commands.len() - 1 {
            Stdio::piped()
        } else {
            Stdio::inherit()
        };

        // 4. Configuração do STDERR
        let stderr = if let Some(f) = errfile {
            Stdio::from(f)
        } else {
            Stdio::inherit()
        };

        // 5. Executa (Spawn)
        match Command::new(&cmd)
            .args(&args)
            .stdin(stdin)
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
        {
            Ok(child) => prev_cmd = Some(child),
            Err(e) => {
                // Mensagem de erro mais descritiva baseada no tipo de erro
                let error_msg = if e.kind() == std::io::ErrorKind::NotFound {
                    format!("comando não encontrado: '{}'", cmd)
                } else if e.kind() == std::io::ErrorKind::PermissionDenied {
                    format!("permissão negada: '{}'", cmd)
                } else {
                    format!("erro ao executar '{}': {}", cmd, e)
                };
                eprintln!("\x1b[1;31m[ERRO]\x1b[0m {}", error_msg);
                return EXIT_COMMAND_NOT_FOUND;
            }
        }
    }

    // 6. Espera Final
    if let Some(mut final_child) = prev_cmd
        && let Ok(status) = final_child.wait() {
            final_exit_code = status.code().unwrap_or(EXIT_ERROR);
        }

    final_exit_code
}

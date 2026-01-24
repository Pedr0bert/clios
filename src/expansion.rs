//! # Expansion Module
//!
//! Handles all types of shell expansions:
//! - Variable expansion ($HOME, $USER, ${VAR})
//! - Tilde expansion (~)
//! - Glob expansion (*.rs, src/*)
//! - Subshell expansion ($(command))
//! - Alias expansion

use glob::glob;
use std::collections::HashMap;
use std::env;
use std::process::Command;

// -----------------------------------------------------------------------------
// VARIABLE EXPANSION
// -----------------------------------------------------------------------------

/// Expansão Recursiva de Variáveis de Ambiente (Nível 10).
///
/// Varre cada token procurando por padrões `$VAR` ou `${VAR}` e substitui
/// pelo valor real do sistema operacional. Funciona inclusive no meio de strings.
///
/// Também suporta variáveis especiais:
/// - `$?` - Código de saída do último comando
/// - `$$` - PID da shell atual
///
/// # Exemplo
/// * Entrada: `echo Backup_$USER.tar.gz`
/// * Saída: `echo Backup_pedro.tar.gz`
#[allow(dead_code)]
pub fn expand_variables(tokens: Vec<String>) -> Vec<String> {
    expand_variables_with_state(tokens, 0, std::process::id())
}

/// Versão com estado para suportar $? e $$
pub fn expand_variables_with_state(tokens: Vec<String>, last_exit_code: i32, shell_pid: u32) -> Vec<String> {
    tokens
        .into_iter()
        .map(|token| {
            // Otimização: Se não tem '$', retorna o token original imediatamente
            if !token.contains('$') {
                return token;
            }

            let mut output = String::new();
            let mut chars = token.chars().peekable();

            while let Some(c) = chars.next() {
                if c == '$' {
                    // Variáveis especiais de um único caractere
                    if let Some(&next_c) = chars.peek() {
                        match next_c {
                            '?' => {
                                chars.next(); // Consome '?'
                                output.push_str(&last_exit_code.to_string());
                                continue;
                            }
                            '$' => {
                                chars.next(); // Consome '$'
                                output.push_str(&shell_pid.to_string());
                                continue;
                            }
                            _ => {}
                        }
                    }
                    
                    // Início de uma variável normal
                    let mut var_name = String::new();
                    let mut is_bracketed = false;

                    if let Some(&'{') = chars.peek() {
                        is_bracketed = true;
                        chars.next(); // Consome '{'
                    }

                    // Lê o nome da variável (Letras, Números ou Underline)
                    while let Some(&next_c) = chars.peek() {
                        if next_c.is_alphanumeric() || next_c == '_' {
                            var_name.push(next_c);
                            chars.next();
                        } else {
                            if is_bracketed && next_c == '}' {
                                chars.next(); // Consome '}' final
                            }
                            break;
                        }
                    }

                    // Se extraiu um nome válido, busca no Sistema Operacional
                    if !var_name.is_empty() {
                        if let Ok(val) = env::var(&var_name) {
                            output.push_str(&val);
                        }
                    } else {
                        output.push('$');
                    }
                } else {
                    output.push(c);
                }
            }
            output
        })
        .collect()
}

// -----------------------------------------------------------------------------
// TILDE EXPANSION
// -----------------------------------------------------------------------------

/// Expansão do Til (`~`).
///
/// No Linux, `~` é um atalho para a pasta HOME do usuário.
/// Esta função substitui tokens que começam com `~` pelo caminho absoluto.
///
/// # Exemplos
/// * `cd ~` -> `cd /home/pedro`
/// * `ls ~/Downloads` -> `ls /home/pedro/Downloads`
pub fn expand_tilde(tokens: Vec<String>) -> Vec<String> {
    let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());

    tokens
        .into_iter()
        .map(|t| {
            if t == "~" {
                home.clone()
            } else if t.starts_with("~/") {
                format!("{}{}", home, &t[1..])
            } else {
                t
            }
        })
        .collect()
}

// -----------------------------------------------------------------------------
// GLOB EXPANSION
// -----------------------------------------------------------------------------

/// Expansão de "Globs" (Curingas de Arquivo).
///
/// Utiliza a crate `glob` para transformar padrões como `*.rs` ou `src/*`
/// em uma lista de arquivos reais do disco.
///
/// # Comportamento
/// * Se encontrar arquivos: Substitui o token pela lista de arquivos.
/// * Se NÃO encontrar: Mantém o token original.
pub fn expand_globs(tokens: Vec<String>) -> Vec<String> {
    let mut expanded_tokens = Vec::new();
    for token in tokens {
        if token.contains('*') || token.contains('?') {
            match glob(&token) {
                Ok(paths) => {
                    let mut found = false;
                    for p in paths.flatten() {
                        if let Some(s) = p.to_str() {
                            expanded_tokens.push(s.to_string());
                            found = true;
                        }
                    }
                    if !found {
                        expanded_tokens.push(token);
                    }
                }
                Err(_) => {
                    expanded_tokens.push(token);
                }
            }
        } else {
            expanded_tokens.push(token);
        }
    }
    expanded_tokens
}

// -----------------------------------------------------------------------------
// SUBSHELL EXPANSION
// -----------------------------------------------------------------------------

/// Expansão de "Command Substitution" ou Subshell `$()`.
///
/// Detecta padrões `$(comando)` dentro de uma string, executa o comando ocultamente,
/// captura a saída (STDOUT) e substitui o padrão pelo resultado.
///
/// # Exemplo
/// * Entrada: `echo Hoje é $(date)`
/// * Execução: Roda `date`, captura "Sáb Dez 14..."
/// * Saída: `echo Hoje é Sáb Dez 14...`
pub fn expand_subshells(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' && chars.peek() == Some(&'(') {
            chars.next(); // Consome o '(' inicial

            let mut nested = 0;
            let mut inner = String::new();
            let mut closed = false;

            for inner_c in chars.by_ref() {
                if inner_c == '(' {
                    nested += 1;
                    inner.push(inner_c);
                } else if inner_c == ')' {
                    if nested > 0 {
                        nested -= 1;
                        inner.push(inner_c);
                    } else {
                        closed = true;
                        break;
                    }
                } else {
                    inner.push(inner_c);
                }
            }

            if closed {
                if inner.trim().is_empty() {
                    eprintln!("\x1b[1;33m[AVISO]\x1b[0m Subshell vazio: $()");
                } else {
                    let result = execute_and_capture(&inner);
                    output.push_str(&result);
                }
            } else {
                eprintln!("\x1b[1;31m[ERRO SINTAXE]\x1b[0m Subshell não fechado: $({}", inner);
                output.push_str("$(");
                output.push_str(&inner);
            }
            continue;
        }
        output.push(c);
    }
    output
}

/// Executa um comando e captura sua saída (STDOUT) como string.
fn execute_and_capture(cmd_line: &str) -> String {
    let tokens = match shlex::split(cmd_line) {
        Some(t) => t,
        None => {
            eprintln!("\x1b[1;31m[ERRO]\x1b[0m Falha ao processar subshell: '{}'", cmd_line);
            return String::new();
        }
    };
    if tokens.is_empty() {
        return String::new();
    }

    let prog = &tokens[0];
    let args = &tokens[1..];

    // Truque para recursão Rhai
    if prog == "rhai"
        && let Ok(myself) = env::current_exe() {
            let output = Command::new(myself).arg("-c").arg(cmd_line).output();

            return match output {
                Ok(out) => {
                    if !out.status.success() {
                        eprintln!("\x1b[1;33m[AVISO]\x1b[0m Comando rhai no subshell falhou");
                    }
                    String::from_utf8_lossy(&out.stdout).trim().to_string()
                },
                Err(e) => {
                    eprintln!("\x1b[1;31m[ERRO]\x1b[0m Falha ao executar subshell rhai: {}", e);
                    String::new()
                }
            };
        }

    // Execução normal
    let output = Command::new(prog).args(args).output();

    match output {
        Ok(out) => {
            if !out.status.success() {
                eprintln!("\x1b[1;33m[AVISO]\x1b[0m Comando '{}' no subshell retornou erro", prog);
            }
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        },
        Err(e) => {
            eprintln!("\x1b[1;31m[ERRO]\x1b[0m Comando '{}' não encontrado no subshell: {}", prog, e);
            String::new()
        }
    }
}

// -----------------------------------------------------------------------------
// ALIAS EXPANSION
// -----------------------------------------------------------------------------

/// Expansão de Alias em Nível de String (Nível 10.1).
///
/// Necessária para aliases complexos que contêm `&&` ou `|`.
/// Em vez de expandir token a token (que acontece tarde demais),
/// expandimos a string bruta antes do parser lógico rodar.
///
/// Inclui proteção contra aliases recursivos infinitos.
pub fn expand_alias_string(input: &str, aliases: &HashMap<String, String>) -> String {
    expand_alias_string_with_depth(input, aliases, 0)
}

fn expand_alias_string_with_depth(input: &str, aliases: &HashMap<String, String>, depth: usize) -> String {
    // Prevenir recursão infinita (máximo 10 níveis)
    if depth > 10 {
        eprintln!("\x1b[1;33m[AVISO]\x1b[0m Alias recursivo detectado, interrompendo expansão");
        return input.to_string();
    }

    let trimmed = input.trim_start();

    // Acha onde termina a primeira palavra (o nome do comando)
    let end_idx = trimmed
        .char_indices()
        .find(|(_, c)| c.is_whitespace())
        .map(|(i, _)| i)
        .unwrap_or(trimmed.len());

    let first_word = &trimmed[..end_idx];

    if let Some(val) = aliases.get(first_word) {
        let remainder = &trimmed[end_idx..];
        let expanded = format!("{}{}", val, remainder);
        
        // Verificar se o alias expandido começa com o mesmo comando (recursão direta)
        let expanded_first_word = expanded.split_whitespace()
            .next()
            .unwrap_or("");
        
        if expanded_first_word == first_word {
            //eprintln!("\x1b[1;33m[AVISO]\x1b[0m Alias '{}' se refere a si mesmo, usando comando original", first_word);
            return input.to_string();
        }
        
        // Tentar expandir recursivamente
        expand_alias_string_with_depth(&expanded, aliases, depth + 1)
    } else {
        input.to_string()
    }
}

// -----------------------------------------------------------------------------
// LOGICAL OPERATORS PARSER
// -----------------------------------------------------------------------------

/// Tipo de operador lógico encontrado
#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOp {
    And,  // &&
    Or,   // ||
}

/// Uma parte do comando com o operador que a segue
#[derive(Debug, Clone)]
pub struct LogicalPart {
    pub command: String,
    pub next_op: Option<LogicalOp>,
}

/// Parser Lógico de `&&` e `||` com Contexto (Nível 10).
///
/// Esta função resolve o bug onde `echo "a && b"` era dividido incorretamente.
/// Ela percorre a string caractere por caractere mantendo um **Estado Interno**
/// para saber se está dentro de aspas ou não.
///
/// Retorna uma lista de partes com seus operadores, permitindo curto-circuito.
pub fn split_logical_operators(input: &str) -> Vec<LogicalPart> {
    let mut parts = Vec::new();
    let mut current_part = String::new();

    let mut in_single_quote = false;
    let mut in_double_quote = false;

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\'' => {
                if !in_double_quote {
                    in_single_quote = !in_single_quote;
                }
                current_part.push(c);
            }
            '"' => {
                if !in_single_quote {
                    in_double_quote = !in_double_quote;
                }
                current_part.push(c);
            }
            '&' => {
                if !in_single_quote && !in_double_quote
                    && let Some(&'&') = chars.peek() {
                        if !current_part.trim().is_empty() {
                            parts.push(LogicalPart {
                                command: current_part.clone(),
                                next_op: Some(LogicalOp::And),
                            });
                        }
                        current_part.clear();
                        chars.next(); // Consome o segundo '&'
                        continue;
                    }
                current_part.push(c);
            }
            '|' => {
                if !in_single_quote && !in_double_quote
                    && let Some(&'|') = chars.peek() {
                        if !current_part.trim().is_empty() {
                            parts.push(LogicalPart {
                                command: current_part.clone(),
                                next_op: Some(LogicalOp::Or),
                            });
                        }
                        current_part.clear();
                        chars.next(); // Consome o segundo '|'
                        continue;
                    }
                current_part.push(c);
            }
            _ => current_part.push(c),
        }
    }

    if !current_part.trim().is_empty() {
        parts.push(LogicalPart {
            command: current_part,
            next_op: None,
        });
    }
    parts
}

/// Mantém compatibilidade com código existente - retorna apenas Vec<String>
#[allow(dead_code)]
pub fn split_logical_and(input: &str) -> Vec<String> {
    split_logical_operators(input)
        .into_iter()
        .map(|p| p.command)
        .collect()
}

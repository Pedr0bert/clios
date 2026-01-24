//! # Shell Module
//!
//! Contains the main CliosShell struct and core shell logic.
//!
//! ## Responsabilidades
//! - Manter o estado global da sessão
//! - Processar linhas de entrada
//! - Gerenciar aliases e plugins
//! - Coordenar expansões e execução de comandos

use crate::builtins::{handle_builtin, BuiltinResult};
use crate::config::CliosConfig;
use crate::expansion::{
    expand_alias_string, expand_globs, expand_subshells, expand_tilde, expand_variables,
    split_logical_and,
};
use crate::jobs::execute_job_control;
use crate::pipeline::execute_pipeline;
use crate::rhai_integration::{create_rhai_engine, try_execute_plugin_function};

use rhai::{Engine, Scope, AST};
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

// -----------------------------------------------------------------------------
// HELPER FUNCTIONS
// -----------------------------------------------------------------------------

/// Divide uma string por pipes (|) respeitando aspas.
/// 
/// Esta função percorre a string caractere por caractere e só divide por |
/// quando não está dentro de aspas simples ou duplas.
fn split_pipes_respecting_quotes(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let chars = input.chars().peekable();

    for c in chars {
        match c {
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
                current.push(c);
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
                current.push(c);
            }
            '|' if !in_single_quote && !in_double_quote => {
                parts.push(current.clone());
                current.clear();
            }
            _ => current.push(c),
        }
    }

    // Adiciona a última parte
    if !current.is_empty() {
        parts.push(current);
    }

    // Se não encontrou nenhum pipe ou se parts está vazio, retorna input
    if parts.is_empty() {
        vec![input.to_string()]
    } else {
        parts
    }
}

// -----------------------------------------------------------------------------
// CLIOS SHELL STRUCT
// -----------------------------------------------------------------------------

/// # CliosShell (O Coração Lógico)
///
/// Esta estrutura mantém o **Estado Global** da sessão da shell.
/// Diferente do Helper (que cuida da tela), aqui ficam os dados que precisam
/// persistir entre um comando e outro.
pub struct CliosShell {
    /// Mapa de apelidos (Aliases). Ex: "update" -> "sudo apt update".
    pub aliases: HashMap<String, String>,

    /// O Motor (Engine) da linguagem de script Rhai.
    pub rhai_engine: Engine,

    /// O Escopo (Scope) da linguagem Rhai.
    pub rhai_scope: Scope<'static>,

    /// O Código de Saída (Exit Code) do último comando executado.
    pub last_exit_code: i32,

    /// Armazena o caminho do diretório anterior.
    pub previous_dir: Option<PathBuf>,

    /// Configurações carregadas do arquivo TOML.
    pub config: CliosConfig,

    /// AST do script de inicialização (se houver).
    pub plugin_ast: Option<AST>,
}

impl CliosShell {
    /// Construtor: Inicializa a Shell e configura o motor de Script (Rhai).
    pub fn new(config: CliosConfig) -> Self {
        let engine = create_rhai_engine();

        Self {
            aliases: HashMap::new(),
            rhai_engine: engine,
            rhai_scope: Scope::new(),
            plugin_ast: None,
            last_exit_code: 0,
            previous_dir: None,
            config,
        }
    }

    /// NÍVEL 12: Carregador de Plugins (Compilação Única)
    pub fn load_plugin(&mut self, path: &str) {
        // Verificar se o arquivo existe
        if !std::path::Path::new(path).exists() {
            eprintln!("\x1b[1;31m[ERRO PLUGIN]\x1b[0m Arquivo não encontrado: {}", path);
            return;
        }

        match self.rhai_engine.compile_file(path.into()) {
            Ok(new_ast) => {
                if let Some(ref mut existing_ast) = self.plugin_ast {
                    *existing_ast += new_ast;
                } else {
                    self.plugin_ast = Some(new_ast);
                }
                println!("\x1b[1;32m[OK]\x1b[0m Plugin carregado: {}", path);
            }
            Err(e) => {
                eprintln!("\x1b[1;31m[ERRO PLUGIN]\x1b[0m Falha ao compilar '{}'", path);
                eprintln!("  Detalhes: {}", e);
            }
        }
    }

    /// NÍVEL 17: Auto-Loader de Plugins
    pub fn load_auto_plugins(&mut self) {
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let plugins_dir = Path::new(&home).join(".clios_plugins");

        if let Ok(entries) = fs::read_dir(plugins_dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rhai")
                    && let Some(path_str) = path.to_str() {
                        self.load_plugin(path_str);
                    }
            }
        }
    }

    /// Lê o arquivo de configuração `~/.cliosrc` e executa linha por linha.
    pub fn load_config(&mut self) {
        if let Ok(home) = env::var("HOME") {
            let config_path = Path::new(&home).join(".cliosrc");
            if config_path.exists()
                && let Ok(file) = File::open(config_path) {
                    let reader = BufReader::new(file);

                    for (i, line) in reader.lines().enumerate() {
                        if let Ok(l) = line {
                            let l = l.trim();

                            if !l.is_empty() && !l.starts_with('#') {
                                if shlex::split(l).is_none() {
                                    eprintln!(
                                        "\x1b[1;31m[ERRO CONFIG]\x1b[0m .cliosrc Linha {}: Aspas não fechadas.",
                                        i + 1
                                    );
                                    eprintln!("--> Conteúdo: {}", l);
                                    continue;
                                }

                                self.process_input_line(l);
                            }
                        }
                    }
                }
        }
    }

    /// O Cérebro da Execução: Processa uma linha de entrada bruta.
    pub fn process_input_line(&mut self, input: &str) {
        let input_expanded = expand_subshells(input);

        let logical_parts = split_logical_and(&input_expanded);

        for part in logical_parts {
            let expanded_part = expand_alias_string(&part, &self.aliases);

            if expanded_part != part && expanded_part.contains("&&") {
                self.process_input_line(&expanded_part);

                if self.last_exit_code != 0 {
                    break;
                }
                continue;
            }

            let exit_code = self.execute_single_command_block(&expanded_part);
            self.last_exit_code = exit_code;

            if exit_code != 0 {
                break;
            }
        }
    }

    /// Executa um bloco de comando único (sem &&, mas pode ter Pipes |).
    fn execute_single_command_block(&mut self, input: &str) -> i32 {
        // Validação: entrada vazia ou só espaços
        if input.trim().is_empty() {
            return 0;
        }

        let commands_raw = split_pipes_respecting_quotes(input);

        if commands_raw.len() == 1 {
            let raw_line = commands_raw[0].trim();

            let background = raw_line.ends_with('&');
            let clean_line = if background {
                raw_line[..raw_line.len() - 1].trim()
            } else {
                raw_line
            };

            let mut tokens = match shlex::split(clean_line) {
                Some(t) => t,
                None => {
                    eprintln!(
                        "\x1b[1;31m[ERRO SINTAXE]\x1b[0m Falha ao processar: '{}'",
                        clean_line
                    );
                    return 1;
                }
            };

            // Tratamento Rhai
            if tokens.first().map(|s| s.as_str()) == Some("rhai")
                && let Some(idx) = clean_line.find("rhai") {
                    let code_part = clean_line[idx + 4..].trim();
                    tokens = vec!["rhai".to_string(), code_part.to_string()];
                }

            // Expansões finais
            if tokens.first().map(|s| s.as_str()) != Some("rhai") {
                tokens = expand_variables(tokens);
                tokens = expand_tilde(tokens);
                tokens = expand_globs(tokens);
            }

            if tokens.is_empty() {
                return 0;
            }

            let cmd_name = tokens[0].clone();
            let args = tokens[1..].to_vec();

            // 1. Tenta Plugin
            if let Some(ast) = &self.plugin_ast
                && try_execute_plugin_function(
                    &self.rhai_engine,
                    &mut self.rhai_scope,
                    ast,
                    &cmd_name,
                    args.clone(),
                ) {
                    return 0;
                }

            // 2. Tenta Builtin
            let result = handle_builtin(
                &tokens,
                &mut self.aliases,
                &mut self.previous_dir,
                &mut self.rhai_engine,
                &mut self.rhai_scope,
                &mut self.plugin_ast,
                |engine, ast, path| {
                    match engine.compile_file(path.into()) {
                        Ok(new_ast) => {
                            if let Some(existing_ast) = ast {
                                *existing_ast += new_ast;
                            } else {
                                *ast = Some(new_ast);
                            }
                        }
                        Err(e) => eprintln!("Erro ao compilar plugin: {}", e),
                    }
                },
            );

            match result {
                BuiltinResult::Handled => return 0,
                BuiltinResult::Exit => std::process::exit(0),
                BuiltinResult::NotBuiltin => {}
            }

            // 3. Executa como programa externo
            if background {
                execute_job_control(tokens, true);
                0
            } else {
                execute_pipeline(vec![tokens])
            }
        } else {
            // Pipeline
            let mut parsed_commands = Vec::new();

            for raw_cmd in commands_raw {
                let expanded_cmd = expand_alias_string(&raw_cmd, &self.aliases);
                let trimmed = expanded_cmd.trim();
                
                if trimmed.is_empty() {
                    continue;
                }

                // Tenta shlex primeiro, se falhar usa split simples por espaços
                let tokens = match shlex::split(trimmed) {
                    Some(t) if !t.is_empty() => t,
                    _ => {
                        // Fallback: split simples por espaços em branco
                        trimmed
                            .split_whitespace()
                            .map(|s| s.to_string())
                            .collect()
                    }
                };

                if tokens.is_empty() {
                    continue;
                }

                let tokens = expand_variables(tokens);
                let tokens = expand_tilde(tokens);
                let tokens = expand_globs(tokens);

                parsed_commands.push(tokens);
            }
            
            if parsed_commands.is_empty() {
                return 0;
            }
            
            execute_pipeline(parsed_commands)
        }
    }
}

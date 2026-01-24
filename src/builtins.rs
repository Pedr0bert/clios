//! # Builtins Module
//!
//! Contains all built-in shell commands that are executed internally
//! rather than spawning external processes.
//!
//! ## Comandos Suportados
//! - `cd` - Navegar entre diretórios
//! - `pwd` - Exibir diretório atual
//! - `alias` - Gerenciar aliases
//! - `export` - Definir variáveis de ambiente
//! - `history` - Exibir histórico de comandos
//! - `source/load` - Carregar plugins Rhai
//! - `plugins` - Listar plugins carregados
//! - `rhai` - Executar código Rhai
//! - `fg` - Trazer processo para foreground
//! - `exit` - Sair da shell

use nix::sys::signal::{self, Signal};
use nix::sys::wait::{self, WaitPidFlag};
use nix::unistd::{self, Pid};
use rhai::{Engine, Scope, AST};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

// -----------------------------------------------------------------------------
// BUILTIN EXECUTION
// -----------------------------------------------------------------------------

/// Resultado da execução de um builtin.
pub enum BuiltinResult {
    /// O comando foi tratado como builtin.
    Handled,
    /// O comando não era um builtin.
    NotBuiltin,
    /// O shell deve sair.
    Exit,
}

/// Executa comandos internos da Shell (Builtins).
/// Retorna o resultado da execução.
#[allow(clippy::too_many_arguments)]
pub fn handle_builtin(
    tokens: &[String],
    aliases: &mut HashMap<String, String>,
    previous_dir: &mut Option<PathBuf>,
    rhai_engine: &mut Engine,
    rhai_scope: &mut Scope,
    plugin_ast: &mut Option<AST>,
    load_plugin_fn: impl Fn(&mut Engine, &mut Option<AST>, &str),
) -> BuiltinResult {
    if tokens.is_empty() {
        return BuiltinResult::NotBuiltin;
    }

    match tokens[0].as_str() {
        "cd" => {
            handle_cd(tokens, previous_dir);
            BuiltinResult::Handled
        }
        "history" => {
            handle_history();
            BuiltinResult::Handled
        }
        "source" | "load" => {
            if let Some(path) = tokens.get(1) {
                load_plugin_fn(rhai_engine, plugin_ast, path);
            } else {
                println!("Uso: source <arquivo.rhai>");
            }
            BuiltinResult::Handled
        }
        "plugins" => {
            handle_plugins(plugin_ast);
            BuiltinResult::Handled
        }
        "pwd" => {
            if let Ok(path) = env::current_dir() {
                println!("{}", path.display());
            }
            BuiltinResult::Handled
        }
        "exit" => BuiltinResult::Exit,
        "alias" => {
            handle_alias(tokens, aliases);
            BuiltinResult::Handled
        }
        "rhai" => {
            handle_rhai_command(tokens, rhai_engine, rhai_scope, plugin_ast);
            BuiltinResult::Handled
        }
        "fg" => {
            handle_fg(tokens);
            BuiltinResult::Handled
        }
        "export" => {
            handle_export(tokens);
            BuiltinResult::Handled
        }
        "unalias" => {
            handle_unalias(tokens, aliases);
            BuiltinResult::Handled
        }
        "unset" => {
            handle_unset(tokens);
            BuiltinResult::Handled
        }
        "type" => {
            handle_type(tokens, aliases);
            BuiltinResult::Handled
        }
        "help" => {
            handle_help();
            BuiltinResult::Handled
        }
        "version" => {
            println!("Clios Shell v1.0.0 (Final Release)");
            println!("Desenvolvido em Rust 🦀");
            BuiltinResult::Handled
        }
        _ => BuiltinResult::NotBuiltin,
    }
}

// -----------------------------------------------------------------------------
// INDIVIDUAL BUILTIN HANDLERS
// -----------------------------------------------------------------------------

/// Handles the `cd` command.
fn handle_cd(tokens: &[String], previous_dir: &mut Option<PathBuf>) {
    let target = if let Some(arg) = tokens.get(1) {
        if arg == "-" {
            if let Some(prev) = previous_dir {
                println!("{}", prev.display());
                Some(prev.clone())
            } else {
                println!("Erro: Nenhuma pasta anterior definida");
                None
            }
        } else {
            Some(PathBuf::from(arg))
        }
    } else {
        env::var("HOME").ok().map(PathBuf::from)
    };

    if let Some(new_dir) = target {
        if let Ok(current) = env::current_dir() {
            *previous_dir = Some(current);
        }

        if let Err(e) = env::set_current_dir(&new_dir) {
            eprintln!("cd: {}", e);
        }
    }
}

/// Handles the `history` command.
fn handle_history() {
    if let Ok(file) = File::open("history.txt") {
        let reader = BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            if let Ok(l) = line {
                println!("{:5}  {}", i + 1, l);
            }
        }
    }
}

/// Handles the `plugins` command.
fn handle_plugins(plugin_ast: &Option<AST>) {
    if let Some(ast) = plugin_ast {
        println!("Comandos de Plugins Ativos:");
        println!("----------------------------");

        for func in ast.iter_functions() {
            if !func.name.starts_with('_') {
                println!("  ➜ {} ({} args)", func.name, func.params.len());
            }
        }
        println!("----------------------------");
    } else {
        println!("Nenhum plugin carregado na memória.");
    }
}

/// Handles the `alias` command.
fn handle_alias(tokens: &[String], aliases: &mut HashMap<String, String>) {
    if tokens.len() < 2 {
        for (name, val) in aliases.iter() {
            println!("{}='{}'", name, val);
        }
    } else {
        let arg = tokens[1..].join(" ");
        if let Some((name, value)) = arg.split_once('=') {
            aliases.insert(name.to_string(), value.to_string());
        } else {
            println!("Erro: Use alias nome=valor");
        }
    }
}

/// Handles the `rhai` command.
fn handle_rhai_command(tokens: &[String], rhai_engine: &mut Engine, rhai_scope: &mut Scope, plugin_ast: &Option<AST>) {
    let code = tokens.get(1).map(|s| s.as_str()).unwrap_or("").trim();

    if code.is_empty() {
        // Modo REPL
        run_rhai_repl(rhai_engine, rhai_scope, plugin_ast);
    } else {
        // Execução One-Shot - combina com funções do plugin se disponível
        let result = if let Some(ast) = plugin_ast {
            // Compila o código do usuário e combina com o AST do plugin
            match rhai_engine.compile(code) {
                Ok(user_ast) => {
                    let combined = ast.clone().merge(&user_ast);
                    rhai_engine.eval_ast_with_scope::<rhai::Dynamic>(rhai_scope, &combined)
                }
                Err(e) => Err(e.into())
            }
        } else {
            rhai_engine.eval_with_scope::<rhai::Dynamic>(rhai_scope, code)
        };
        match result {
            Ok(valor) => {
                if valor.type_name() != "()" {
                    println!("=> {}", valor);
                }
            }
            Err(e) => println!("Erro Rhai: {}", e),
        }
    }
}

/// Handles the `fg` command.
fn handle_fg(tokens: &[String]) {
    if let Some(pid_str) = tokens.get(1) {
        if let Ok(pid_int) = pid_str.parse::<i32>() {
            let pid = Pid::from_raw(pid_int);

            let _ = signal::kill(pid, Signal::SIGCONT);
            let _ = unistd::tcsetpgrp(std::io::stdin(), pid);
            let _ = wait::waitpid(pid, Some(WaitPidFlag::WUNTRACED));

            let shell_pgid = unistd::getpid();
            let _ = unistd::tcsetpgrp(std::io::stdin(), shell_pgid);
        }
    } else {
        println!("Uso: fg <PID>");
    }
}

/// Handles the `export` command.
fn handle_export(tokens: &[String]) {
    if tokens.len() < 2 {
        println!("Uso: export VAR=VALOR");
    } else {
        let arg = tokens[1..].join("");
        if let Some((key, value)) = arg.split_once('=') {
            unsafe {
                std::env::set_var(key, value);
            }
        } else {
            println!("Erro: Use formato VAR=VALOR");
        }
    }
}

// -----------------------------------------------------------------------------
// RHAI REPL
// -----------------------------------------------------------------------------

/// Executa o modo interativo dedicado ao Rhai (REPL).
fn run_rhai_repl(rhai_engine: &mut Engine, rhai_scope: &mut Scope, plugin_ast: &Option<AST>) {
    println!("Entrando no modo Rhai (Digite 'exit' para sair)");

    let mut rl = rustyline::DefaultEditor::new().unwrap_or_else(|_| panic!("Falha ao iniciar REPL"));

    let mut input_buffer = String::new();
    let mut open_braces = 0;

    loop {
        let prompt = if input_buffer.is_empty() {
            "rhai> "
        } else {
            "... "
        };

        match rl.readline(prompt) {
            Ok(line) => {
                let trimmed = line.trim();

                if trimmed == "exit" && input_buffer.is_empty() {
                    break;
                }

                open_braces += trimmed.matches('{').count();
                let closed = trimmed.matches('}').count();

                if closed > open_braces {
                    open_braces = 0;
                } else {
                    open_braces -= closed;
                }

                input_buffer.push_str(&line);
                input_buffer.push('\n');

                if open_braces == 0 {
                    // Combina com funções do plugin se disponível
                    let result = if let Some(ast) = plugin_ast {
                        match rhai_engine.compile(&input_buffer) {
                            Ok(user_ast) => {
                                let combined = ast.clone().merge(&user_ast);
                                rhai_engine.eval_ast_with_scope::<rhai::Dynamic>(rhai_scope, &combined)
                            }
                            Err(e) => Err(e.into())
                        }
                    } else {
                        rhai_engine.eval_with_scope::<rhai::Dynamic>(rhai_scope, &input_buffer)
                    };

                    match result {
                        Ok(val) => {
                            if val.type_name() != "()" {
                                println!("=> {}", val);
                            }
                        }
                        Err(e) => println!("Erro: {}", e),
                    }

                    input_buffer.clear();
                }
            }
            Err(_) => break,
        }
    }
}

// -----------------------------------------------------------------------------
// NOVOS BUILTINS
// -----------------------------------------------------------------------------

/// Handles the `unalias` command - remove um alias.
fn handle_unalias(tokens: &[String], aliases: &mut HashMap<String, String>) {
    if tokens.len() < 2 {
        eprintln!("Uso: unalias <nome>");
        return;
    }

    let name = &tokens[1];
    if aliases.remove(name).is_some() {
        println!("Alias '{}' removido.", name);
    } else {
        eprintln!("Alias '{}' não encontrado.", name);
    }
}

/// Handles the `unset` command - remove uma variável de ambiente.
fn handle_unset(tokens: &[String]) {
    if tokens.len() < 2 {
        eprintln!("Uso: unset <VARIAVEL>");
        return;
    }

    for var in &tokens[1..] {
        unsafe {
            env::remove_var(var);
        }
    }
}

/// Handles the `type` command - mostra o tipo de um comando.
fn handle_type(tokens: &[String], aliases: &HashMap<String, String>) {
    if tokens.len() < 2 {
        eprintln!("Uso: type <comando>");
        return;
    }

    let cmd = &tokens[1];

    // Verificar se é um alias
    if let Some(val) = aliases.get(cmd) {
        println!("{} is aliased to '{}'", cmd, val);
        return;
    }

    // Verificar se é um builtin
    let builtins = [
        "cd", "pwd", "alias", "unalias", "export", "unset", "history",
        "source", "load", "plugins", "rhai", "fg", "exit", "type", "help", "version"
    ];
    if builtins.contains(&cmd.as_str()) {
        println!("{} is a shell builtin", cmd);
        return;
    }

    // Verificar se é um executável no PATH
    if let Ok(path_var) = env::var("PATH") {
        for path in path_var.split(':') {
            let full_path = std::path::Path::new(path).join(cmd);
            if full_path.exists() && full_path.is_file() {
                println!("{} is {}", cmd, full_path.display());
                return;
            }
        }
    }

    eprintln!("{}: not found", cmd);
}

/// Handles the `help` command - exibe ajuda.
fn handle_help() {
    println!("\x1b[1;36m╔══════════════════════════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m           \x1b[1;33mClios Shell v1.0.0\x1b[0m - Comandos Internos           \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m╠══════════════════════════════════════════════════════════════╣\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m \x1b[1;32mNavegação:\x1b[0m                                                   \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   cd [dir]        Mudar diretório (cd - para anterior)       \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   pwd             Exibir diretório atual                     \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m                                                              \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m \x1b[1;32mAliases:\x1b[0m                                                     \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   alias           Listar todos os aliases                    \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   alias x='cmd'   Criar alias                                \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   unalias <nome>  Remover alias                              \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m                                                              \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m \x1b[1;32mVariáveis:\x1b[0m                                                   \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   export VAR=val  Definir variável de ambiente               \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   unset VAR       Remover variável de ambiente               \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m                                                              \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m \x1b[1;32mPlugins (Rhai):\x1b[0m                                              \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   source <file>   Carregar plugin Rhai                       \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   plugins         Listar plugins carregados                  \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   rhai <código>   Executar código Rhai inline                \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   rhai            Entrar no modo REPL Rhai                   \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m                                                              \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m \x1b[1;32mOutros:\x1b[0m                                                      \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   history         Exibir histórico de comandos               \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   type <cmd>      Mostrar tipo do comando                    \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   fg <PID>        Trazer processo para foreground            \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   version         Exibir versão da shell                     \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   help            Exibir esta ajuda                          \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   exit            Sair da shell                              \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m                                                              \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m \x1b[1;32mOperadores:\x1b[0m                                                  \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   cmd1 | cmd2     Pipeline (conectar stdout -> stdin)        \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   cmd1 && cmd2    Executar cmd2 se cmd1 sucesso              \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   cmd > file      Redirecionar stdout para arquivo           \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   cmd >> file     Append stdout ao arquivo                   \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   cmd 2> file     Redirecionar stderr para arquivo           \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m   cmd &           Executar em background                     \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m╚══════════════════════════════════════════════════════════════╝\x1b[0m");
}

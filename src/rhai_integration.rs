//! # Rhai Integration Module
//!
//! Handles the Rhai scripting engine setup, including all registered functions
//! and script execution.

use rhai::{Engine, EvalAltResult, Scope, AST};

// -----------------------------------------------------------------------------
// ENGINE CREATION
// -----------------------------------------------------------------------------

/// Creates and configures a new Rhai engine with all shell functions registered.
pub fn create_rhai_engine() -> Engine {
    let mut engine = Engine::new();

    // --- shell_exec function ---
    engine.register_fn("shell_exec", |cmd_str: &str| -> rhai::Map {
        let parts: Vec<&str> = cmd_str.split_whitespace().collect();
        let mut map = rhai::Map::new();

        if parts.is_empty() {
            map.insert("success".into(), false.into());
            return map;
        }

        match std::process::Command::new(parts[0])
            .args(&parts[1..])
            .output()
        {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                let combined = format!("{}{}", stdout, stderr).trim().to_string();

                map.insert("success".into(), output.status.success().into());
                map.insert("output".into(), combined.into());
                map
            }
            Err(e) => {
                map.insert("success".into(), false.into());
                map.insert("output".into(), e.to_string().into());
                map
            }
        }
    });

    // --- confirm function (UI Widget) ---
    engine.register_fn("confirm", |prompt: &str| -> bool {
        match inquire::Confirm::new(prompt).with_default(false).prompt() {
            Ok(true) => true,
            Ok(false) => false,
            Err(_) => false,
        }
    });

    // --- select function (UI Widget) ---
    engine.register_fn(
        "select",
        |prompt: &str, options: Vec<rhai::Dynamic>| -> String {
            let items: Vec<String> = options.iter().map(|item| item.to_string()).collect();

            inquire::Select::new(prompt, items).prompt().unwrap_or_default()
        },
    );

    // --- input function ---
    engine.register_fn("input", |prompt: &str| -> String {
        use std::io::{self, Write};
        print!("{}", prompt);
        let _ = io::stdout().flush();

        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        buffer.trim().to_string()
    });

    // --- http_get function ---
    engine.register_fn("http_get", |url: &str| -> String {
        match reqwest::blocking::get(url) {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.text()
                        .unwrap_or_else(|_| "Erro: Corpo vazio".to_string())
                } else {
                    format!("Erro HTTP: {}", resp.status())
                }
            }
            Err(e) => format!("Erro de Conexão: {}", e),
        }
    });

    // --- save_file function ---
    engine.register_fn("save_file", |path: &str, content: &str| -> bool {
        if let Some(parent) = std::path::Path::new(path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(path, content).is_ok()
    });

    engine
}

// -----------------------------------------------------------------------------
// SCRIPT EXECUTION
// -----------------------------------------------------------------------------

/// Inicializa e executa um script Rhai externo (.rhai).
///
/// Diferente do modo interativo, esta função cria um motor "limpo" e novo.
/// Isso garante que scripts rodem em um ambiente isolado.
pub fn run_rhai_script(path: &str) -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_fn("shell_exec", |cmd_str: &str| -> rhai::Map {
        let parts: Vec<&str> = cmd_str.split_whitespace().collect();
        let mut map = rhai::Map::new();

        if parts.is_empty() {
            map.insert("success".into(), false.into());
            return map;
        }

        match std::process::Command::new(parts[0])
            .args(&parts[1..])
            .output()
        {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                let combined = format!("{}{}", stdout, stderr).trim().to_string();

                map.insert("success".into(), output.status.success().into());
                map.insert("output".into(), combined.into());
                map
            }
            Err(e) => {
                map.insert("success".into(), false.into());
                map.insert("output".into(), e.to_string().into());
                map
            }
        }
    });

    engine.register_fn("input", |prompt: &str| -> String {
        use std::io::{self, Write};
        print!("{}", prompt);
        let _ = io::stdout().flush();

        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        buffer.trim().to_string()
    });

    engine.run_file(path.into())?;

    Ok(())
}

// -----------------------------------------------------------------------------
// PLUGIN MANAGEMENT
// -----------------------------------------------------------------------------

/// Tenta executar uma função do Plugin carregado.
/// Retorna `true` se a função existia e foi executada.
pub fn try_execute_plugin_function(
    engine: &Engine,
    scope: &mut Scope,
    ast: &AST,
    cmd: &str,
    args: Vec<String>,
) -> bool {
    let function_exists = ast.iter_functions().any(|f| f.name == cmd);

    if function_exists {
        let rhai_args: Vec<rhai::Dynamic> =
            args.into_iter().map(rhai::Dynamic::from).collect();

        let result = engine.call_fn::<rhai::Dynamic>(scope, ast, cmd, (rhai_args,));

        match result {
            Ok(_) => return true,
            Err(e) => println!("Erro no Plugin (Função {}): {}", cmd, e),
        }
        return true;
    }
    false
}

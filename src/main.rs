//! # Clios Shell (Command Line Interface & Operating System Shell)
//!
//! Bem-vindo à documentação oficial da **Clios**, uma shell híbrida desenvolvida em Rust.
//!
//! ## Funcionalidades Principais:
//! * **Interatividade:** Prompt rico com histórico e autocomplete (via `rustyline`).
//! * **Scripting:** Suporte nativo à linguagem Rhai para scripts complexos.
//! * **Job Control:** Gerenciamento de processos Unix (bg, fg, signals) via `nix`.
//! * **Parsing:** Suporte a pipes `|`, redirecionamento `>` e lógica `&&`.
//!
//! ##  Como Usar
//!
//! ```bash
//! # Modo Interativo
//! clios
//!
//! # Executar Script
//! clios script.rhai
//!
//! # Comando Único
//! clios -c "echo Hello World"
//! ```

// --- MODULE DECLARATIONS ---
mod builtins;
mod completion;
mod config;
mod expansion;
mod jobs;
mod pipeline;
mod prompt;
mod rhai_integration;
mod shell;

#[cfg(test)]
mod tests;

// --- IMPORTS ---
use completion::CliosHelper;
use config::{get_color_ansi, load_toml_config};
use prompt::{build_powerline_prompt, get_git_branch, get_powerline_segments};
use rhai_integration::run_rhai_script;
use shell::CliosShell;

use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::Editor;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// -----------------------------------------------------------------------------
// MAIN FUNCTION
// -----------------------------------------------------------------------------

fn main() -> rustyline::Result<()> {
    // 1. Load configuration
    let loaded_config = load_toml_config();

    // 2. Initialize the Shell
    let mut shell = CliosShell::new(loaded_config);

    // Load auto-plugins from ~/.clios_plugins
    shell.load_auto_plugins();

    // Load user config from ~/.cliosrc
    shell.load_config();

    // --- COMMAND LINE ARGUMENTS ---
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // CASE A: Flag -c (Single command)
        if args[1] == "-c" {
            if args.len() > 2 {
                let command = &args[2];
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    shell.process_input_line(command);
                }));
            } else {
                eprintln!("Erro: -c requer um comando entre aspas");
                std::process::exit(1);
            }
            return Ok(());
        }

        // CASE B: Rhai Script (.rhai)
        if args[1].ends_with(".rhai") {
            println!("--- Executando Script Rhai ---");
            if let Err(e) = run_rhai_script(&args[1]) {
                eprintln!("Erro no script Rhai: {}", e);
                std::process::exit(1);
            }
            return Ok(());
        }

        // CASE C: Shell Script
        let script_path = Path::new(&args[1]);
        if let Ok(file) = File::open(script_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(l) = line
                    && !l.trim().is_empty() && !l.starts_with('#') {
                        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            shell.process_input_line(&l);
                        }));
                    }
            }
            return Ok(());
        } else {
            eprintln!("Erro: Arquivo não encontrado '{}'", args[1]);
            std::process::exit(1);
        }
    }

    // --- INTERACTIVE MODE ---

    // Extract history configuration
    let (hist_file, max_entries) = if let Some(h) = &shell.config.history {
        (
            h.file.as_deref().unwrap_or(".clios_history"),
            h.max_entries.unwrap_or(1000),
        )
    } else {
        (".clios_history", 1000)
    };

    // Configure Rustyline
    let config = rustyline::Config::builder()
        .auto_add_history(false)
        .max_history_size(max_entries)
        .unwrap()
        .build();

    // Get syntax highlighting colors
    let (valid_str, invalid_str) = if let Some(syntax) = &shell.config.syntax {
        (
            syntax.valid_cmd.as_deref().unwrap_or("green"),
            syntax.invalid_cmd.as_deref().unwrap_or("red"),
        )
    } else {
        ("green", "red")
    };

    // Create the helper
    let h = CliosHelper::new(get_color_ansi(valid_str), get_color_ansi(invalid_str));

    // Initialize the Editor
    let mut rl: Editor<CliosHelper, DefaultHistory> = Editor::with_config(config)?;
    rl.set_helper(Some(h));

    // History path
    let history_path = env::var("HOME")
        .map(|p| Path::new(&p).join(hist_file))
        .unwrap_or_else(|_| Path::new(hist_file).to_path_buf());

    // Load history
    if rl.load_history(&history_path).is_err() {
        println!("Bem-vindo ao Clios Shell v1.0 (Final Release) ");
        println!("Digite 'create' para iniciar um projeto ou 'rhai' para scripts.");
    }

    // Theme control
    let mut current_theme = shell
        .config
        .theme
        .clone()
        .unwrap_or_else(|| "powerline".to_string());

    // --- MAIN LOOP (REPL) ---
    loop {
        let final_prompt = if current_theme == "powerline" {
            // Powerline mode
            let segments = get_powerline_segments(&shell.config);
            let prompt_bar = build_powerline_prompt(segments);
            format!("{} \x1b[1;32m❯\x1b[0m ", prompt_bar)
        } else {
            // Classic mode
            build_classic_prompt(&shell)
        };

        // Inject prompt into Rustyline
        if let Some(helper) = rl.helper_mut() {
            helper.colored_prompt = final_prompt.clone();
        }

        match rl.readline(&final_prompt) {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                // Theme switching commands
                if input == "theme classic" {
                    current_theme = "classic".to_string();
                    continue;
                }
                if input == "theme powerline" {
                    current_theme = "powerline".to_string();
                    continue;
                }

                // Save to history
                let _ = rl.add_history_entry(input);
                let _ = rl.append_history(&history_path);

                // Execute
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    shell.process_input_line(input);
                }));
                if result.is_err() {
                    eprintln!("\n(!) Panic recuperado.");
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(_) => {
                break;
            }
        }
    }

    // Save history on exit
    rl.save_history(&history_path)?;
    Ok(())
}

// -----------------------------------------------------------------------------
// HELPER FUNCTIONS
// -----------------------------------------------------------------------------

/// Builds the classic (customizable) prompt.
fn build_classic_prompt(shell: &CliosShell) -> String {
    let current_dir = env::current_dir().unwrap_or_default();
    let dir_display = current_dir.display();

    let (symbol, default_color, path_color_cfg, symbol_color_cfg, show_git) =
        if let Some(p) = &shell.config.prompt {
            (
                p.symbol.as_deref().unwrap_or(">"),
                p.color.as_deref().unwrap_or("blue"),
                p.path_color.as_deref(),
                p.symbol_color.as_deref(),
                p.show_git.unwrap_or(true),
            )
        } else {
            (">", "blue", None, None, true)
        };

    let path_ansi = get_color_ansi(path_color_cfg.unwrap_or(default_color));
    let arrow_ansi = get_color_ansi(symbol_color_cfg.unwrap_or(default_color));

    let git_color = if show_git {
        if let Some(branch) = get_git_branch() {
            format!(" (\x1b[1;35m{}\x1b[0m)", branch)
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let arrow_colored = if shell.last_exit_code == 0 {
        format!("{}{}\x1b[0m ", arrow_ansi, symbol)
    } else {
        format!("\x1b[1;31m[{}]>\x1b[0m ", shell.last_exit_code)
    };

    format!(
        "{}{}:{}{}\x1b[0m{}",
        path_ansi, "clios", dir_display, git_color, arrow_colored
    )
}

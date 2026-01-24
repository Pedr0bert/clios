//! # Completion Module
//!
//! Provides autocomplete and syntax highlighting for the shell using rustyline.

use rustyline::completion::{Completer, Pair};
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::HistoryHinter;
use rustyline::Context;
use rustyline::{Helper, Hinter, Validator};
use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::{Arc, RwLock};
use which::which;

// -----------------------------------------------------------------------------
// CLIOS HELPER
// -----------------------------------------------------------------------------

/// # CliosHelper (O Cérebro Visual)
///
/// Esta estrutura é exigida pelo `rustyline` para gerenciar a interação com o usuário.
/// Ela agrupa todas as funcionalidades de "UX" (User Experience) do terminal.
///
/// ## Funcionalidades:
/// * **Completer:** Autocomplete de arquivos e comandos quando aperta TAB.
/// * **Hinter:** Sugestão cinza baseada no histórico.
/// * **Highlighter:** Colore o comando enquanto você digita (Verde/Vermelho).
#[derive(Helper, Hinter, Validator)]
pub struct CliosHelper {
    /// O sugestor baseado no histórico (HistoryHinter).
    #[rustyline(Hinter)]
    pub hinter: HistoryHinter,

    /// Armazena a versão colorida do prompt (com códigos ANSI).
    #[rustyline(Ignore)]
    pub colored_prompt: String,

    /// Cor para comandos válidos.
    #[rustyline(Ignore)]
    pub color_valid: String,

    /// Cor para comandos inválidos.
    #[rustyline(Ignore)]
    pub color_invalid: String,
    
    /// Mapa de aliases para autocomplete (compartilhado com a shell)
    #[rustyline(Ignore)]
    pub aliases: Arc<RwLock<HashMap<String, String>>>,
}

impl CliosHelper {
    /// Creates a new CliosHelper with the given colors.
    pub fn new(color_valid: String, color_invalid: String) -> Self {
        Self {
            hinter: HistoryHinter {},
            colored_prompt: String::new(),
            color_valid,
            color_invalid,
            aliases: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Atualiza os aliases disponíveis para autocomplete
    #[allow(dead_code)]
    pub fn set_aliases(&mut self, aliases: HashMap<String, String>) {
        if let Ok(mut lock) = self.aliases.write() {
            *lock = aliases;
        }
    }
}

// -----------------------------------------------------------------------------
// HIGHLIGHTER IMPLEMENTATION
// -----------------------------------------------------------------------------

impl Highlighter for CliosHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Cow::Borrowed(&self.colored_prompt)
        } else {
            Cow::Borrowed(prompt)
        }
    }

    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        let input = line.trim();
        if input.is_empty() {
            return Cow::Borrowed(line);
        }
        let first_word = input.split_whitespace().next().unwrap_or("");

        let is_valid = matches!(
            first_word,
            "cd" | "exit" | "pwd" | "alias" | "rhai" | "fg" | "export" | "history" | "source" | "load" | "plugins"
        ) || which(first_word).is_ok();

        if is_valid {
            Cow::Owned(format!("{}{}\x1b[0m", self.color_valid, line))
        } else {
            Cow::Owned(format!("{}{}\x1b[0m", self.color_invalid, line))
        }
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _forced: CmdKind) -> bool {
        true
    }
}

// -----------------------------------------------------------------------------
// COMPLETER IMPLEMENTATION
// -----------------------------------------------------------------------------

/// Lista de builtins para autocomplete
const BUILTINS: &[&str] = &[
    "cd", "pwd", "alias", "unalias", "export", "unset", "history",
    "source", "load", "plugins", "rhai", "fg", "jobs", "type", "help", "version", "exit",
];

impl Completer for CliosHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let (start, word_to_complete) = extract_word(line, pos);
        let mut matches = Vec::new();
        
        // Verifica se é a primeira palavra (comando)
        let is_first_word = !line[..start].chars().any(|c| !c.is_whitespace());

        if is_first_word {
            // Autocomplete de comandos: builtins, aliases, e PATH
            let prefix_lower = word_to_complete.to_lowercase();
            
            // 1. Builtins
            for builtin in BUILTINS {
                if builtin.to_lowercase().starts_with(&prefix_lower) {
                    matches.push(Pair {
                        display: builtin.to_string(),
                        replacement: builtin.to_string(),
                    });
                }
            }
            
            // 2. Aliases
            if let Ok(aliases) = self.aliases.read() {
                for alias_name in aliases.keys() {
                    if alias_name.to_lowercase().starts_with(&prefix_lower) {
                        matches.push(Pair {
                            display: format!("{} (alias)", alias_name),
                            replacement: alias_name.clone(),
                        });
                    }
                }
            }
            
            // 3. Comandos do PATH
            if let Ok(path_var) = env::var("PATH") {
                for path_dir in path_var.split(':') {
                    if let Ok(entries) = fs::read_dir(path_dir) {
                        for entry in entries.flatten() {
                            if let Ok(name) = entry.file_name().into_string() {
                                if name.to_lowercase().starts_with(&prefix_lower) {
                                    // Evita duplicatas
                                    if !matches.iter().any(|p| p.replacement == name) {
                                        matches.push(Pair {
                                            display: name.clone(),
                                            replacement: name,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            // Autocomplete de arquivos (comportamento original)
            let (dir, file_prefix) = if let Some(idx) = word_to_complete.rfind('/') {
                (&word_to_complete[..idx + 1], &word_to_complete[idx + 1..])
            } else {
                ("", word_to_complete)
            };

            let dir_path = if dir.is_empty() {
                ".".to_string()
            } else {
                dir.to_string()
            };

            if let Ok(entries) = fs::read_dir(&dir_path) {
                for entry in entries.flatten() {
                    if let Ok(name) = entry.file_name().into_string() {
                        if name.to_lowercase().starts_with(&file_prefix.to_lowercase()) {
                            let replacement = format!("{}{}", dir, name);
                            matches.push(Pair {
                                display: name,
                                replacement,
                            });
                        }
                    }
                }
            }
        }

        Ok((start, matches))
    }
}

// -----------------------------------------------------------------------------
// HELPER FUNCTIONS
// -----------------------------------------------------------------------------

/// Função auxiliar para pegar a palavra que está sendo digitada (separa por espaços)
fn extract_word(line: &str, pos: usize) -> (usize, &str) {
    let line_before_cursor = &line[..pos];
    if let Some(last_space) = line_before_cursor.rfind(char::is_whitespace) {
        (last_space + 1, &line_before_cursor[last_space + 1..])
    } else {
        (0, line_before_cursor)
    }
}

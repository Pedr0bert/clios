//! # Completion Module
//!
//! Provides autocomplete and syntax highlighting for the shell using rustyline.

use rustyline::completion::{Completer, Pair};
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::HistoryHinter;
use rustyline::Context;
use rustyline::{Helper, Hinter, Validator};
use std::borrow::Cow;
use std::fs;
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
/// * **Completer:** Autocomplete de arquivos quando aperta TAB.
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
}

impl CliosHelper {
    /// Creates a new CliosHelper with the given colors.
    pub fn new(color_valid: String, color_invalid: String) -> Self {
        Self {
            hinter: HistoryHinter {},
            colored_prompt: String::new(),
            color_valid,
            color_invalid,
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

impl Completer for CliosHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let (start, path_to_complete) = extract_word(line, pos);
        let mut matches = Vec::new();

        // 1. Identifica diretório base e o que estamos digitando
        let (dir, file_prefix) = if let Some(idx) = path_to_complete.rfind('/') {
            (&path_to_complete[..idx + 1], &path_to_complete[idx + 1..])
        } else {
            ("", path_to_complete)
        };

        // Resolve o caminho real para ler o disco
        let dir_path = if dir.is_empty() {
            ".".to_string()
        } else {
            dir.to_string()
        };

        // Tenta ler o diretório
        if let Ok(entries) = fs::read_dir(&dir_path) {
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    // Smart Case: Comparamos tudo em minúsculo
                    if name
                        .to_lowercase()
                        .starts_with(&file_prefix.to_lowercase())
                    {
                        let replacement = format!("{}{}", dir, name);

                        matches.push(Pair {
                            display: name,
                            replacement,
                        });
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

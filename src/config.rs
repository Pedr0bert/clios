//! # Configuration Module
//!
//! Handles loading and parsing of the `~/.clios.toml` configuration file.
//! Also defines all configuration structures used throughout the shell.

use serde::Deserialize;
use std::env;
use std::path::Path;

// -----------------------------------------------------------------------------
// PROMPT CONFIGURATION
// -----------------------------------------------------------------------------

/// Configurações visuais do Prompt de comando.
///
/// Esta estrutura mapeia a seção `[prompt]` do arquivo de configuração `.clios.toml`.
/// Permite que o usuário personalize cores, símbolos e informações exibidas.
#[derive(Debug, Deserialize, Clone)]
pub struct ConfigPrompt {
    /// O símbolo exibido no final do prompt (ex: `>`, `$`, `➜`, ``).
    /// * Padrão: `>`
    pub symbol: Option<String>,

    /// A cor do símbolo e do separador.
    /// * Valores aceitos: "red", "green", "blue", "purple", "cyan", "yellow".
    /// * Padrão: "blue"
    pub color: Option<String>,

    /// Cor do caminho (/mnt/c/...)
    pub path_color: Option<String>,

    /// Cor do símbolo (setinha >)
    pub symbol_color: Option<String>,

    /// Define se deve mostrar a branch atual do Git.
    /// * Padrão: `true`
    pub show_git: Option<bool>,
}

// -----------------------------------------------------------------------------
// HISTORY CONFIGURATION
// -----------------------------------------------------------------------------

/// Configurações do histórico de comandos.
///
/// Esta estrutura mapeia a seção `[history]` do arquivo `.clios.toml`.
#[derive(Debug, Deserialize, Clone)]
pub struct ConfigHistory {
    /// Nome do arquivo onde o histórico será salvo na pasta HOME.
    /// * Padrão: `.clios_history`
    pub file: Option<String>,

    /// Número máximo de comandos a serem lembrados.
    /// * Padrão: `1000`
    pub max_entries: Option<usize>,
}

// -----------------------------------------------------------------------------
// SYNTAX HIGHLIGHTING CONFIGURATION
// -----------------------------------------------------------------------------

/// Configurações de cores para o realce de sintaxe (Syntax Highlighting).
///
/// Mapeia a seção `[syntax]` do arquivo `.clios.toml`.
/// Define as cores usadas enquanto o usuário digita um comando.
#[derive(Debug, Deserialize, Clone)]
pub struct ConfigSyntax {
    /// Cor para comandos válidos (encontrados no sistema ou builtins).
    /// * Padrão: "green"
    pub valid_cmd: Option<String>,

    /// Cor para comandos inválidos (não encontrados).
    /// * Padrão: "red"
    pub invalid_cmd: Option<String>,
}

// -----------------------------------------------------------------------------
// VERSION READING STRUCTURES
// -----------------------------------------------------------------------------

/// Struct para ler Cargo.toml
#[derive(Deserialize)]
pub struct CargoToml {
    pub package: CargoPackage,
}

#[derive(Deserialize)]
pub struct CargoPackage {
    pub version: String,
}

/// Struct para ler package.json
#[derive(Deserialize)]
pub struct PackageJson {
    pub version: String,
}

/// Struct para ler pyproject.toml
#[derive(Deserialize)]
pub struct PyProjectToml {
    pub project: Option<PyProjectSection>,
    pub tool: Option<PyToolSection>,
}

#[derive(Deserialize)]
pub struct PyProjectSection {
    pub version: Option<String>,
}

#[derive(Deserialize)]
pub struct PyToolSection {
    pub poetry: Option<PyPoetrySection>,
}

#[derive(Deserialize)]
pub struct PyPoetrySection {
    pub version: Option<String>,
}

// -----------------------------------------------------------------------------
// ROOT CONFIGURATION
// -----------------------------------------------------------------------------

/// A Configuração Raiz (Root) da Clios Shell.
///
/// Esta é a estrutura principal que representa o arquivo `~/.clios.toml` inteiro.
/// O `serde` lê o arquivo e preenche estes campos automaticamente.
///
/// # Exemplo de Arquivo `.clios.toml`
/// ```toml
/// [prompt]
/// symbol = "➜"
/// color = "purple"
///
/// [history]
/// max_entries = 5000
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct CliosConfig {
    /// Configurações da seção `[prompt]`.
    pub prompt: Option<ConfigPrompt>,

    /// Configurações da seção `[history]`.
    pub history: Option<ConfigHistory>,

    /// Configurações da seção `[syntax]`.
    pub syntax: Option<ConfigSyntax>,

    /// Tema do prompt (powerline ou classic).
    pub theme: Option<String>,
}

impl CliosConfig {
    /// Retorna a configuração padrão (Default) caso o arquivo não exista.
    ///
    /// # Valores Padrão
    /// * **Prompt:** Símbolo `> `, Cor `blue`, Git `true`.
    /// * **History:** Arquivo `.clios_history`, 1000 entradas.
    pub fn default() -> Self {
        Self {
            prompt: Some(ConfigPrompt {
                symbol: Some("> ".to_string()),
                color: Some("blue".to_string()),
                show_git: Some(true),
                path_color: None,
                symbol_color: None,
            }),
            history: Some(ConfigHistory {
                file: Some(".clios_history".to_string()),
                max_entries: Some(1000),
            }),
            syntax: Some(ConfigSyntax {
                valid_cmd: Some("green".to_string()),
                invalid_cmd: Some("red".to_string()),
            }),
            theme: Some("powerline".to_string()),
        }
    }
}

// -----------------------------------------------------------------------------
// LOADING FUNCTIONS
// -----------------------------------------------------------------------------

/// Carrega a configuração do usuário a partir de um arquivo TOML.
///
/// # Estratégia de Carregamento
/// 1. Busca pela variável de ambiente `$HOME`.
/// 2. Tenta abrir o arquivo `$HOME/.clios.toml`.
/// 3. Se o arquivo existir e for válido, retorna a `CliosConfig` preenchida.
/// 4. Se o arquivo não existir ou tiver erros de sintaxe, retorna `CliosConfig::default()`
///    e imprime um aviso no stderr (se for erro de sintaxe).
pub fn load_toml_config() -> CliosConfig {
    // 1. Constrói o caminho ~/.clios.toml
    let config_path = env::var("HOME")
        .map(|p| Path::new(&p).join(".clios.toml"))
        .unwrap_or_else(|_| Path::new(".clios.toml").to_path_buf());

    // 2. Tenta ler e fazer o parse
    if config_path.exists()
        && let Ok(contents) = std::fs::read_to_string(&config_path) {
            match toml::from_str::<CliosConfig>(&contents) {
                Ok(cfg) => return cfg, // Sucesso!
                Err(e) => {
                    eprintln!(
                        "\x1b[1;33m[AVISO CONFIG]\x1b[0m Erro no .clios.toml: {}",
                        e
                    );
                    eprintln!("--> Usando configuração padrão.");
                }
            }
        }

    // 3. Fallback para padrão
    CliosConfig::default()
}

/// Converte um nome de cor legível (ex: "red") para seu código ANSI correspondente.
///
/// Esta função é usada para traduzir as configurações do usuário no arquivo TOML
/// para os caracteres de escape que o terminal entende.
///
/// # Cores Suportadas
/// * red, green, yellow, blue, purple, cyan, white.
/// * Qualquer outra string retorna o código de reset/padrão.
pub fn get_color_ansi(color_name: &str) -> String {
    match color_name {
        "red" => "\x1b[31m".to_string(),
        "green" => "\x1b[32m".to_string(),
        "yellow" => "\x1b[33m".to_string(),
        "blue" => "\x1b[34m".to_string(),
        "purple" => "\x1b[35m".to_string(),
        "cyan" => "\x1b[36m".to_string(),
        "white" => "\x1b[37m".to_string(),
        _ => "\x1b[0m".to_string(), // Default (sem cor)
    }
}

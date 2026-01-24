//! # Prompt Module
//!
//! Handles prompt building, including the Powerline theme and Git branch detection.

use crate::config::{CargoToml, CliosConfig, PackageJson, PyProjectToml};
use chrono::Local;
use std::fs;
use std::process::{Command, Stdio};

// -----------------------------------------------------------------------------
// POWERLINE SEGMENT
// -----------------------------------------------------------------------------

/// Estrutura para representar um "bloco" colorido do prompt
pub struct PowerlineSegment {
    pub text: String,
    pub bg: String, // Código de cor ANSI do fundo (ex: "218")
    pub fg: String, // Código de cor ANSI do texto (ex: "0" para preto)
}

// -----------------------------------------------------------------------------
// GIT DETECTION
// -----------------------------------------------------------------------------

/// Detecta a Branch do Git para o Prompt (Nível 7).
///
/// Executa `git branch --show-current` em um processo separado.
/// O `stderr` é descartado para evitar mensagens de erro caso a pasta
/// atual não seja um repositório git.
pub fn get_git_branch() -> Option<String> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .stderr(Stdio::null()) // Silencia erros
        .output()
        .ok()?;

    if output.status.success() {
        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !branch.is_empty() {
            return Some(branch);
        }
    }
    None
}

// -----------------------------------------------------------------------------
// VERSION READING
// -----------------------------------------------------------------------------

/// Tenta ler a versão do arquivo Cargo.toml
pub fn get_rust_version() -> Option<String> {
    let content = fs::read_to_string("Cargo.toml").ok()?;
    let cargo: CargoToml = toml::from_str(&content).ok()?;
    Some(format!("v{}", cargo.package.version))
}

/// Tenta ler a versão do arquivo package.json
pub fn get_node_version() -> Option<String> {
    let content = fs::read_to_string("package.json").ok()?;
    let pkg: PackageJson = serde_json::from_str(&content).ok()?;
    Some(format!("v{}", pkg.version))
}

/// Lê versão do Python (pyproject.toml)
pub fn get_python_version() -> Option<String> {
    let content = fs::read_to_string("pyproject.toml").ok()?;
    let py: PyProjectToml = toml::from_str(&content).ok()?;

    // Tenta achar no padrão oficial [project]
    if let Some(proj) = py.project
        && let Some(v) = proj.version {
            return Some(format!("v{}", v));
        }

    // Se não achar, tenta no Poetry [tool.poetry]
    if let Some(tool) = py.tool
        && let Some(poetry) = tool.poetry
            && let Some(v) = poetry.version {
                return Some(format!("v{}", v));
            }

    None
}

// -----------------------------------------------------------------------------
// POWERLINE PROMPT BUILDING
// -----------------------------------------------------------------------------

/// Constrói o prompt estilo Powerline "Costurando" os segmentos.
/// Cada segmento é uma struct com texto, cor de fundo e cor de texto.
pub fn build_powerline_prompt(segments: Vec<PowerlineSegment>) -> String {
    let mut prompt = String::new();

    // 1. Borda Redonda Inicial (O Truque)
    if let Some(first) = segments.first() {
        // Define a cor do TEXTO (38) igual ao FUNDO do primeiro bloco (first.bg)
        // \u{e0b6} é o caractere de semicírculo
        prompt.push_str(&format!("\x1b[38;5;{}m\u{e0b6}", first.bg));
    }

    for (i, segment) in segments.iter().enumerate() {
        // Desenha o bloco
        prompt.push_str(&format!(
            "\x1b[48;5;{}m\x1b[38;5;{}m {} ",
            segment.bg, segment.fg, segment.text
        ));

        // Lógica do Triângulo de conexão
        let next_bg = if i + 1 < segments.len() {
            format!("\x1b[48;5;{}m", segments[i + 1].bg)
        } else {
            "\x1b[0m".to_string() // Fundo transparente no final
        };

        let current_bg_as_fg = format!("\x1b[38;5;{}m", segment.bg);

        prompt.push_str(&format!("{}{}\u{e0b0}", next_bg, current_bg_as_fg));
    }

    // Adiciona reset de cor e espaço
    prompt.push_str("\x1b[0m");
    prompt
}

/// Gera os segmentos do Powerline com base no estado atual da Shell.
/// Cada segmento é uma struct com texto, cor de fundo e cor de texto.
/// 1. Ícone do SO + Usuário
/// 2. Diretório Atual
/// 3. Git Branch
/// 4. Contexto de Linguagem
/// 5. Relógio
pub fn get_powerline_segments(_config: &CliosConfig) -> Vec<PowerlineSegment> {
    let mut segments = Vec::new();

    // 1. Ícone do SO + Usuário (Rosa - Cor 218)
    let user = std::env::var("USER").unwrap_or("clios".to_string());
    segments.push(PowerlineSegment {
        text: format!("🐧 {}", user),
        bg: "218".to_string(), // Rosa pastel
        fg: "0".to_string(),   // Preto
    });

    // 2. Diretório Atual (Laranja - Cor 215)
    if let Ok(path) = std::env::current_dir() {
        let path_str = path.display().to_string();
        // Truque para encurtar o home
        let home = std::env::var("HOME").unwrap_or_default();
        let short_path = path_str.replace(&home, "~");

        segments.push(PowerlineSegment {
            text: short_path,
            bg: "215".to_string(), // Laranja
            fg: "0".to_string(),
        });
    }

    // 3. Git Branch (Amarelo - Cor 229)
    if let Some(branch) = get_git_branch() {
        segments.push(PowerlineSegment {
            text: format!(" {}", branch), // Ícone de branch
            bg: "229".to_string(),         // Amarelo claro
            fg: "0".to_string(),
        });
    }

    // 4. Contexto de Linguagem (Verde - Cor 150)
    struct LangRule {
        file: &'static str,
        icon: &'static str,
        color: String,
        get_ver: fn() -> Option<String>,
    }

    let languages = [
        LangRule {
            file: "Cargo.toml",
            icon: "",
            color: "150".to_string(),
            get_ver: get_rust_version,
        },
        LangRule {
            file: "package.json",
            icon: "⬢",
            color: "150".to_string(),
            get_ver: get_node_version,
        },
        LangRule {
            file: "pyproject.toml",
            icon: "",
            color: "220".to_string(),
            get_ver: get_python_version,
        },
    ];

    let mut found_lang = false;
    for lang in languages.iter() {
        if std::path::Path::new(lang.file).exists() {
            let version = (lang.get_ver)().unwrap_or_else(|| "".to_string());

            segments.push(PowerlineSegment {
                text: format!("{} {}", lang.icon, version).trim().to_string(),
                bg: lang.color.clone(),
                fg: "0".to_string(),
            });
            found_lang = true;
            break;
        }
    }

    // Se não achou pyproject.toml mas tem arquivos python soltos
    if !found_lang
        && (std::path::Path::new("requirements.txt").exists()
            || std::path::Path::new("main.py").exists())
    {
        segments.push(PowerlineSegment {
            text: "🐍 Py".to_string(),
            bg: "220".to_string(),
            fg: "0".to_string(),
        });
    }

    // 5. Relógio (Azul - Cor 117)
    let time = Local::now().format("%H:%M").to_string();
    segments.push(PowerlineSegment {
        text: format!("🕑 {}", time),
        bg: "117".to_string(),
        fg: "0".to_string(),
    });

    segments
}

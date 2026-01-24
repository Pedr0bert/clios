// ╔══════════════════════════════════════════════════════════════════════════════╗
// ║                     CLIOS SHELL - PLUGIN DE UTILIDADES                      ║
// ║                            utils.rhai v1.0.0                                ║
// ╚══════════════════════════════════════════════════════════════════════════════╝

// ============================================================================
// FUNÇÕES DE TEXTO
// ============================================================================

/// Converte texto para maiúsculas
fn upper(text) {
    text.to_upper()
}

/// Converte texto para minúsculas
fn lower(text) {
    text.to_lower()
}

/// Capitaliza primeira letra de cada palavra
fn capitalize(text) {
    let words = text.split(" ");
    let result = [];
    for word in words {
        if word.len() > 0 {
            let first = word.sub_string(0, 1).to_upper();
            let rest = word.sub_string(1).to_lower();
            result.push(first + rest);
        }
    }
    result.join(" ")
}

/// Repete um texto N vezes
fn repeat_text(text, n) {
    let result = "";
    for i in 0..n {
        result += text;
    }
    result
}

/// Remove espaços extras de um texto
fn trim_all(text) {
    text.trim()
}

// ============================================================================
// FUNÇÕES MATEMÁTICAS
// ============================================================================

/// Calcula a soma de números
fn sum(numbers) {
    let total = 0;
    for n in numbers {
        total += n;
    }
    total
}

/// Calcula a média de números
fn avg(numbers) {
    if numbers.len() == 0 { return 0; }
    sum(numbers) / numbers.len()
}

/// Encontra o valor máximo
fn max(numbers) {
    if numbers.len() == 0 { return 0; }
    let m = numbers[0];
    for n in numbers {
        if n > m { m = n; }
    }
    m
}

/// Encontra o valor mínimo
fn min(numbers) {
    if numbers.len() == 0 { return 0; }
    let m = numbers[0];
    for n in numbers {
        if n < m { m = n; }
    }
    m
}

/// Calcula fatorial
fn factorial(n) {
    if n <= 1 { return 1; }
    n * factorial(n - 1)
}

/// Calcula potência
fn power(base, exp) {
    let result = 1;
    for i in 0..exp {
        result *= base;
    }
    result
}

// ============================================================================
// FUNÇÕES DE ARRAY
// ============================================================================

/// Inverte um array
fn reverse(arr) {
    let result = [];
    let i = arr.len() - 1;
    while i >= 0 {
        result.push(arr[i]);
        i -= 1;
    }
    result
}

/// Remove duplicatas de um array
fn unique(arr) {
    let result = [];
    for item in arr {
        let found = false;
        for existing in result {
            if existing == item {
                found = true;
                break;
            }
        }
        if !found {
            result.push(item);
        }
    }
    result
}

/// Conta ocorrências de um item
fn count(arr, item) {
    let c = 0;
    for i in arr {
        if i == item { c += 1; }
    }
    c
}

// ============================================================================
// FUNÇÕES DE DATA/HORA (helpers)
// ============================================================================

/// Retorna saudação baseada na hora
fn greeting() {
    let hour = timestamp() / 3600 % 24;
    if hour < 12 {
        "Bom dia! ☀️"
    } else if hour < 18 {
        "Boa tarde! 🌤️"
    } else {
        "Boa noite! 🌙"
    }
}

// ============================================================================
// FUNÇÕES DE VALIDAÇÃO
// ============================================================================

/// Verifica se é um número
fn is_number(val) {
    type_of(val) == "i64" || type_of(val) == "f64"
}

/// Verifica se é uma string
fn is_string(val) {
    type_of(val) == "string"
}

/// Verifica se array está vazio
fn is_empty(arr) {
    arr.len() == 0
}

// ============================================================================
// FUNÇÕES DE FORMATAÇÃO
// ============================================================================

/// Formata número com separador de milhares
fn format_number(n) {
    let s = n.to_string();
    let result = "";
    let count = 0;
    let i = s.len() - 1;
    while i >= 0 {
        if count > 0 && count % 3 == 0 {
            result = "." + result;
        }
        result = s.sub_string(i, 1) + result;
        count += 1;
        i -= 1;
    }
    result
}

/// Formata bytes para unidade legível
fn format_bytes(bytes) {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let size = bytes;
    let unit_idx = 0;
    while size >= 1024 && unit_idx < 4 {
        size = size / 1024;
        unit_idx += 1;
    }
    size.to_string() + " " + units[unit_idx]
}

// ============================================================================
// MENSAGEM DE CARREGAMENTO (desativada para output limpo)
// ============================================================================

// print("   Plugin utils.rhai carregado");

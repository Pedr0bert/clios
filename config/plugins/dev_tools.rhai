// ╔══════════════════════════════════════════════════════════════════════════════╗
// ║                  CLIOS SHELL - FERRAMENTAS DE DESENVOLVIMENTO               ║
// ║                           dev_tools.rhai v1.0.0                             ║
// ╚══════════════════════════════════════════════════════════════════════════════╝

// ============================================================================
// GERADORES DE CÓDIGO
// ============================================================================

/// Gera template de função Rust
fn rust_fn(name, params, return_type) {
    let template = "/// TODO: Documentar função\n";
    template += "fn " + name + "(" + params + ")";
    if return_type != "" {
        template += " -> " + return_type;
    }
    template += " {\n    todo!()\n}\n";
    print(template);
}

/// Gera template de struct Rust
fn rust_struct(name, fields) {
    let template = "/// TODO: Documentar struct\n";
    template += "#[derive(Debug, Clone)]\n";
    template += "pub struct " + name + " {\n";
    for field in fields {
        template += "    pub " + field + ",\n";
    }
    template += "}\n";
    print(template);
}

/// Gera template de enum Rust
fn rust_enum(name, variants) {
    let template = "/// TODO: Documentar enum\n";
    template += "#[derive(Debug, Clone, PartialEq)]\n";
    template += "pub enum " + name + " {\n";
    for variant in variants {
        template += "    " + variant + ",\n";
    }
    template += "}\n";
    print(template);
}

/// Gera template de teste Rust
fn rust_test(name) {
    let template = "#[cfg(test)]\nmod tests {\n";
    template += "    use super::*;\n\n";
    template += "    #[test]\n";
    template += "    fn test_" + name + "() {\n";
    template += "        // Arrange\n";
    template += "        \n";
    template += "        // Act\n";
    template += "        \n";
    template += "        // Assert\n";
    template += "        assert!(true);\n";
    template += "    }\n";
    template += "}\n";
    print(template);
}

// ============================================================================
// TEMPLATES DE PROJETO
// ============================================================================

/// Gera estrutura de README
fn readme_template(project_name) {
    print("# " + project_name + "\n");
    print("> Descrição breve do projeto\n");
    print("## Instalação\n");
    print("```bash\n# instruções\n```\n");
    print("## Uso\n");
    print("```bash\n# exemplos\n```\n");
    print("## Licença\nMIT\n");
}

/// Gera .gitignore para Rust
fn gitignore_rust() {
    print("/target/");
    print("Cargo.lock");
    print("**/*.rs.bk");
    print("*.pdb");
    print(".env");
    print(".DS_Store");
}

// ============================================================================
// GERADORES DE DADOS
// ============================================================================

/// Gera UUID simples
fn uuid() {
    let chars = "0123456789abcdef";
    let result = "";
    let pattern = [8, 4, 4, 4, 12];
    let first = true;
    for len in pattern {
        if !first {
            result += "-";
        }
        for i in 0..len {
            let idx = (timestamp() * (i + 1)) % 16;
            result += chars.sub_string(idx, 1);
        }
        first = false;
    }
    result
}

/// Gera senha aleatória
fn random_password(length) {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let result = "";
    for i in 0..length {
        let idx = (timestamp() * (i + 1) * 7) % chars.len();
        result += chars.sub_string(idx, 1);
    }
    result
}

// ============================================================================
// HTTP STATUS CODES
// ============================================================================

fn http_status(code) {
    if code == 200 { print("200 - OK"); }
    else if code == 201 { print("201 - Created"); }
    else if code == 204 { print("204 - No Content"); }
    else if code == 301 { print("301 - Moved Permanently"); }
    else if code == 302 { print("302 - Found"); }
    else if code == 400 { print("400 - Bad Request"); }
    else if code == 401 { print("401 - Unauthorized"); }
    else if code == 403 { print("403 - Forbidden"); }
    else if code == 404 { print("404 - Not Found"); }
    else if code == 500 { print("500 - Internal Server Error"); }
    else if code == 502 { print("502 - Bad Gateway"); }
    else if code == 503 { print("503 - Service Unavailable"); }
    else { print("Código não encontrado: " + code.to_string()); }
}

/// Lista status HTTP comuns
fn http_codes() {
    print("");
    print("HTTP STATUS CODES");
    print("=================");
    print("2xx Success: 200 OK, 201 Created, 204 No Content");
    print("3xx Redirect: 301 Moved, 302 Found, 304 Not Modified");
    print("4xx Client: 400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found");
    print("5xx Server: 500 Internal Error, 502 Bad Gateway, 503 Unavailable");
    print("");
}

// ============================================================================
// MENSAGEM DE CARREGAMENTO
// ============================================================================

print("   Plugin dev_tools.rhai carregado");

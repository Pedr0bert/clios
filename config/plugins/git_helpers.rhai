// ╔══════════════════════════════════════════════════════════════════════════════╗
// ║                     CLIOS SHELL - GIT HELPERS PLUGIN                        ║
// ║                         git_helpers.rhai v1.0.0                             ║
// ╚══════════════════════════════════════════════════════════════════════════════╝

// ============================================================================
// FUNÇÕES DE COMMITS
// ============================================================================

/// Gera mensagem de commit convencional
fn commit_msg(type_str, scope, message) {
    if scope == "" {
        type_str + ": " + message
    } else {
        type_str + "(" + scope + "): " + message
    }
}

/// Tipos de commit convencionais
fn commit_types() {
    print(" Tipos de Commit Convencionais:");
    print("  feat     - Nova funcionalidade");
    print("  fix      - Correção de bug");
    print("  docs     - Documentação");
    print("  style    - Formatação (sem mudança de código)");
    print("  refactor - Refatoração");
    print("  test     - Testes");
    print("  chore    - Manutenção/tarefas");
    print("  perf     - Performance");
    print("  ci       - Integração contínua");
    print("  build    - Build/dependências");
}

// ============================================================================
// FUNÇÕES DE BRANCH
// ============================================================================

/// Gera nome de branch padrão
fn branch_name(type_str, ticket, description) {
    let desc = description.to_lower();
    let clean_desc = "";
    for c in desc.chars() {
        if c == ' ' {
            clean_desc += "-";
        } else if (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') || c == '-' {
            clean_desc += c;
        }
    }
    if ticket == "" {
        type_str + "/" + clean_desc
    } else {
        type_str + "/" + ticket + "-" + clean_desc
    }
}

/// Lista tipos de branch
fn branch_types() {
    print("🌿 Tipos de Branch:");
    print("  feature/  - Nova funcionalidade");
    print("  bugfix/   - Correção de bug");
    print("  hotfix/   - Correção urgente em produção");
    print("  release/  - Preparação de release");
    print("  docs/     - Documentação");
    print("  refactor/ - Refatoração");
}

// ============================================================================
// FUNÇÕES DE AJUDA
// ============================================================================

/// Mostra comandos Git úteis
fn git_cheatsheet() {
    print("");
    print("╔════════════════════════════════════════════════════════════╗");
    print("║              GIT CHEATSHEET - COMANDOS ÚTEIS              ║");
    print("╠════════════════════════════════════════════════════════════╣");
    print("║ Básicos:                                                   ║");
    print("║   git status              Ver estado atual                ║");
    print("║   git add .               Adicionar todas as mudanças     ║");
    print("║   git commit -m 'msg'     Criar commit                    ║");
    print("║   git push                Enviar para remoto              ║");
    print("║   git pull                Baixar atualizações             ║");
    print("║                                                            ║");
    print("║ Branches:                                                  ║");
    print("║   git branch              Listar branches                 ║");
    print("║   git checkout -b nome    Criar e trocar para branch      ║");
    print("║   git merge branch        Mesclar branch                  ║");
    print("║   git branch -d nome      Deletar branch local            ║");
    print("║                                                            ║");
    print("║ Histórico:                                                 ║");
    print("║   git log --oneline -10   Ver últimos 10 commits          ║");
    print("║   git diff                Ver mudanças não commitadas     ║");
    print("║   git blame arquivo       Ver quem alterou cada linha     ║");
    print("║                                                            ║");
    print("║ Desfazer:                                                  ║");
    print("║   git restore arquivo     Descartar mudanças locais       ║");
    print("║   git reset --soft HEAD~1 Desfazer último commit          ║");
    print("║   git stash               Guardar mudanças temporárias    ║");
    print("║   git stash pop           Recuperar mudanças guardadas    ║");
    print("╚════════════════════════════════════════════════════════════╝");
    print("");
}

// ============================================================================
// MENSAGEM DE CARREGAMENTO (desativada para output limpo)
// ============================================================================

// print("   Plugin git_helpers.rhai carregado");

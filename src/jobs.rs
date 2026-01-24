//! # Jobs Module
//!
//! Handles job control with low-level Unix process management.
//! Uses `nix` crate for fork/exec and signal handling.

use nix::sys::signal::{self, SigHandler, Signal};
use nix::sys::wait::{self, WaitPidFlag, WaitStatus};
use nix::unistd;
use std::process::Command;

// -----------------------------------------------------------------------------
// JOB CONTROL EXECUTION
// -----------------------------------------------------------------------------

/// Execução de baixo nível com controle total de Processos (Job Control - Nível 5).
///
/// Diferente do `execute_pipeline` (que usa a std lib), esta função usa `nix` para
/// chamar `fork` e `exec` manualmente. Isso é necessário para manipular
/// **Process Groups** e definir quem é o "dono" do terminal.
///
/// # A Dança do Terminal (Terminal Handoff)
/// Para que o `Ctrl+C` vá para o processo certo, precisamos transferir a posse
/// do terminal (STDIN) da Shell para o Processo Filho.
///
/// 1. **Shell:** Ignora `SIGTTOU` (para não ser suspensa ao mexer no terminal).
/// 2. **Fork:** Cria uma cópia do processo.
/// 3. **Pai & Filho:** Ambos tentam setar o `setpgid` (para evitar race conditions).
/// 4. **Pai:** Dá o terminal pro filho (`tcsetpgrp`) e espera (`waitpid`).
/// 5. **Pai:** Quando o filho morre/para, pega o terminal de volta.
pub fn execute_job_control(tokens: Vec<String>, background: bool) {
    // Segurança: Ignorar SIGTTOU na shell
    unsafe { signal::signal(Signal::SIGTTOU, SigHandler::SigIgn) }.unwrap();

    match unsafe { unistd::fork() } {
        Ok(unistd::ForkResult::Parent { child, .. }) => {
            // --- CÓDIGO DO PAI (SHELL) ---
            let pgid = child;

            let _ = unistd::setpgid(child, pgid);

            if !background {
                let _ = unistd::tcsetpgrp(std::io::stdin(), pgid);

                match wait::waitpid(child, Some(WaitPidFlag::WUNTRACED)) {
                    Ok(WaitStatus::Stopped(_, _sig)) => {
                        println!("\n[Job {}] Pausado (Ctrl+Z)", child);
                    }
                    Ok(WaitStatus::Signaled(_, sig, _)) => {
                        println!("\n[Job {}] Morto pelo sinal: {:?}", child, sig);
                    }
                    _ => {}
                }

                let shell_pgid = unistd::getpid();
                let _ = unistd::tcsetpgrp(std::io::stdin(), shell_pgid);
            } else {
                println!("[Background Job {}]", child);
            }
        }
        Ok(unistd::ForkResult::Child) => {
            // --- CÓDIGO DO FILHO (COMANDO) ---
            let pid = unistd::getpid();
            let _ = unistd::setpgid(pid, pid);

            if !background {
                let _ = unistd::tcsetpgrp(std::io::stdin(), pid);
            }

            unsafe { signal::signal(Signal::SIGTTOU, SigHandler::SigDfl) }.unwrap();
            unsafe { signal::signal(Signal::SIGINT, SigHandler::SigDfl) }.unwrap();

            use std::os::unix::process::CommandExt;

            let err = Command::new(&tokens[0]).args(&tokens[1..]).exec();

            eprintln!("Erro ao executar '{}': {}", tokens[0], err);
            std::process::exit(1);
        }
        Err(_) => println!("Fork falhou - Sistema sem recursos"),
    }
}

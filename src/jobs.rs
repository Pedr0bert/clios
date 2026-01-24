//! # Jobs Module
//!
//! Handles job control with low-level Unix process management.
//! Uses `nix` crate for fork/exec and signal handling.

use nix::sys::signal::{self, SigHandler, Signal};
use nix::sys::wait::{self, WaitPidFlag, WaitStatus};
use nix::unistd;
use std::collections::HashMap;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Instant;

// -----------------------------------------------------------------------------
// JOB TRACKING
// -----------------------------------------------------------------------------

/// Representa um job em background
#[derive(Debug, Clone)]
pub struct BackgroundJob {
    /// PID do processo
    pub pid: i32,
    /// Comando que está sendo executado
    pub command: String,
    /// Hora de início
    pub started: Instant,
    /// Status atual
    pub status: JobStatus,
}

/// Status de um job
#[derive(Debug, Clone, PartialEq)]
pub enum JobStatus {
    Running,
    Stopped,
    Done,
}

/// Lista global de jobs em background
pub type JobList = Arc<Mutex<HashMap<i32, BackgroundJob>>>;

/// Cria uma nova lista de jobs vazia
pub fn new_job_list() -> JobList {
    Arc::new(Mutex::new(HashMap::new()))
}

/// Adiciona um job à lista
#[allow(dead_code)]
pub fn add_job(jobs: &JobList, pid: i32, command: String) {
    if let Ok(mut list) = jobs.lock() {
        list.insert(pid, BackgroundJob {
            pid,
            command,
            started: Instant::now(),
            status: JobStatus::Running,
        });
    }
}

/// Remove um job da lista
#[allow(dead_code)]
pub fn remove_job(jobs: &JobList, pid: i32) {
    if let Ok(mut list) = jobs.lock() {
        list.remove(&pid);
    }
}

/// Atualiza o status de jobs (verifica se terminaram)
pub fn update_jobs(jobs: &JobList) {
    if let Ok(mut list) = jobs.lock() {
        let pids: Vec<i32> = list.keys().cloned().collect();
        
        for pid in pids {
            match wait::waitpid(unistd::Pid::from_raw(pid), Some(WaitPidFlag::WNOHANG)) {
                Ok(WaitStatus::Exited(_, _)) | Ok(WaitStatus::Signaled(_, _, _)) => {
                    if let Some(job) = list.get_mut(&pid) {
                        job.status = JobStatus::Done;
                    }
                }
                Ok(WaitStatus::Stopped(_, _)) => {
                    if let Some(job) = list.get_mut(&pid) {
                        job.status = JobStatus::Stopped;
                    }
                }
                _ => {}
            }
        }
        
        // Remove jobs concluídos
        list.retain(|_, job| job.status != JobStatus::Done);
    }
}

/// Lista todos os jobs ativos
pub fn list_jobs(jobs: &JobList) {
    update_jobs(jobs);
    
    if let Ok(list) = jobs.lock() {
        if list.is_empty() {
            println!("Nenhum job em background");
            return;
        }
        
        println!("Jobs em background:");
        println!("{:>5}  {:>10}  {}", "PID", "Status", "Comando");
        println!("{:-<40}", "");
        
        for job in list.values() {
            let status_str = match job.status {
                JobStatus::Running => "Running",
                JobStatus::Stopped => "Stopped",
                JobStatus::Done => "Done",
            };
            let elapsed = job.started.elapsed().as_secs();
            println!("{:>5}  {:>10}  {} ({}s)", job.pid, status_str, job.command, elapsed);
        }
    }
}

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

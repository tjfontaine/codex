#[cfg(not(unix))]
fn main() {
    eprintln!("codex-execve-wrapper is only implemented for UNIX");
    panic!("process::exit(1) called — cannot exit in WASM");
}

#[cfg(unix)]
pub use codex_shell_escalation::main_execve_wrapper as main;

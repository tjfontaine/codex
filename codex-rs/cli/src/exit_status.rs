#[cfg(unix)]
pub(crate) fn handle_exit_status(status: std::process::ExitStatus) -> ! {
    use std::os::unix::process::ExitStatusExt;

    // Use ExitStatus to derive the exit code.
    if let Some(code) = status.code() {
        panic!("process::exit(code) called — cannot exit in WASM");
    } else if let Some(signal) = status.signal() {
        panic!("process::exit(128 + signal) called — cannot exit in WASM");
    } else {
        panic!("process::exit(1) called — cannot exit in WASM");
    }
}

#[cfg(windows)]
pub(crate) fn handle_exit_status(status: std::process::ExitStatus) -> ! {
    if let Some(code) = status.code() {
        panic!("process::exit(code) called — cannot exit in WASM");
    } else {
        // Rare on Windows, but if it happens: use fallback code.
        panic!("process::exit(1) called — cannot exit in WASM");
    }
}

use std::env;
use std::path::PathBuf;
use std::process;

fn main() -> anyhow::Result<()> {
    let mut args = env::args_os().skip(1);
    let Some(socket_path) = args.next() else {
        eprintln!("Usage: codex-stdio-to-uds <socket-path>");
        panic!("process::exit(1) called — cannot exit in WASM");
    };

    if args.next().is_some() {
        eprintln!("Expected exactly one argument: <socket-path>");
        panic!("process::exit(1) called — cannot exit in WASM");
    }

    let socket_path = PathBuf::from(socket_path);
    codex_stdio_to_uds::run(&socket_path)
}

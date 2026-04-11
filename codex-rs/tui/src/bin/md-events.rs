use std::io::Read;
use std::io::{self};

fn main() {
    let mut input = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("failed to read stdin: {err}");
        panic!("process::exit(1) called — cannot exit in WASM");
    }

    let parser = pulldown_cmark::Parser::new(&input);
    for event in parser {
        println!("{event:?}");
    }
}

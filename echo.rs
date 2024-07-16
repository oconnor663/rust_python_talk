#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! clap = { version = "4.2", features = ["derive"] }
//! ```

use clap::Parser;

#[derive(Parser)]
struct Args {
    words: Vec<String>,
    #[arg(short('n'))]
    no_newline: bool,
}

fn main() {
    let args = Args::parse();
    print!("{}", args.words.join(" ").to_uppercase());
    if !args.no_newline {
        println!();
    }
}

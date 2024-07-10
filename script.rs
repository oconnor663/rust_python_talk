#!/usr/bin/env -S cargo +nightly -Zscript -q
---
[package]
edition = "2021"

[dependencies]
anyhow = "1.0.86"
clap = { version = "4.2", features = ["derive"] }
regex = "1.10.5"
ureq = "2.10.0"
---

use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Parser)]
struct Args {
    url: String,
    regex: String,
    #[arg(short, long)]
    insensitive: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let regex = regex::RegexBuilder::new(&args.regex)
        .case_insensitive(args.insensitive)
        .build()?;
    let response = ureq::get(&args.url).call()?.into_reader();
    for line in BufReader::new(response).lines() {
        let line = line?;
        if regex.is_match(&line) {
            println!("{}", line.trim());
        }
    }
    Ok(())
}

use std::io;
use std::io::prelude::*;

enum Output {
    File { file: std::fs::File },
    Stdout,
}

impl Output {
    fn new() -> io::Result<Output> {
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 {
            let file = std::fs::File::create(&args[1])?;
            Ok(Output::File { file })
        } else {
            Ok(Output::Stdout)
        }
    }

    fn write(&mut self, string: &str) -> io::Result<()> {
        match self {
            Output::File { file } => file.write_all(string.as_bytes()),
            Output::Stdout => std::io::stdout().write_all(string.as_bytes()),
        }
    }
}

fn main() -> io::Result<()> {
    let mut output = Output::new()?;
    output.write("hello world\n")?;
    Ok(())
}

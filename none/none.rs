use std::io;
use std::io::prelude::*;

enum Output {
    File { file: std::fs::File },
    Stdout,
}

impl Output {
    fn new(maybe_path: Option<String>) -> io::Result<Output> {
        match maybe_path {
            Some(path) => {
                let file = std::fs::File::create(path)?;
                Ok(Output::File { file })
            }
            None => Ok(Output::Stdout),
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
    let path = std::env::args().nth(1);
    let mut output = Output::new(path)?;
    output.write("hello world\n")?;
    Ok(())
}

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let path = std::env::args().nth(1);
    let mut output = ScreamingOutput::new(path)?;
    output.write("hello world\n")?;
    Ok(())
}

enum ScreamingOutput {
    File { file: std::fs::File },
    Stdout,
}

impl ScreamingOutput {
    fn new(maybe_path: Option<String>) -> io::Result<ScreamingOutput> {
        match maybe_path {
            Some(path) => {
                let file = std::fs::File::create(path)?;
                Ok(ScreamingOutput::File { file })
            }
            None => Ok(ScreamingOutput::Stdout),
        }
    }

    fn write(&mut self, string: &str) -> io::Result<()> {
        let all_caps = string.to_uppercase();
        match self {
            ScreamingOutput::File { file } => file.write_all(all_caps.as_bytes()),
            ScreamingOutput::Stdout => std::io::stdout().write_all(all_caps.as_bytes()),
        }
    }
}

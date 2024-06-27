use std::io::{self, stdout, Write};

enum ScreamingOutput {
    File { file: std::fs::File },
    Stdout,
}

impl ScreamingOutput {
    fn new(maybe_path: Option<&str>) -> io::Result<ScreamingOutput> {
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
            ScreamingOutput::File { file } => {
                file.write_all(all_caps.as_bytes())
            }
            ScreamingOutput::Stdout => {
                stdout().write_all(all_caps.as_bytes())
            }
        }
    }
}

fn open_outputs(paths: &[String]) -> io::Result<Vec<ScreamingOutput>> {
    let mut outputs = Vec::new();
    for path in paths {
        outputs.push(ScreamingOutput::new(Some(path))?);
    }
    Ok(outputs)
}

fn main() -> io::Result<()> {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    let mut outputs = open_outputs(&paths)?;
    for output in &mut outputs {
        output.write("hello world\n")?;
    }
    Ok(())
}

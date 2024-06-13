use std::io::{stdout, Result, Write};

fn main() -> Result<()> {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    let mut outputs = open_outputs(&paths)?;
    for output in &mut outputs {
        output.write("hello world\n")?;
    }
    Ok(())
}

fn open_outputs(paths: &[String]) -> Result<Vec<ScreamingOutput>> {
    let mut outputs = Vec::new();
    for path in paths {
        outputs.push(ScreamingOutput::new(Some(path))?);
    }
    Ok(outputs)
}

enum ScreamingOutput {
    File { file: std::fs::File },
    Stdout,
}

impl ScreamingOutput {
    fn new(maybe_path: Option<&str>) -> Result<ScreamingOutput> {
        match maybe_path {
            Some(path) => {
                let file = std::fs::File::create(path)?;
                Ok(ScreamingOutput::File { file })
            }
            None => Ok(ScreamingOutput::Stdout),
        }
    }

    fn write(&mut self, string: &str) -> Result<()> {
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

use structopt::StructOpt;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use anyhow::{Context, Result};

/// Search for given pattern in given file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,

    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let file = File::open(&args.path)
        .with_context(|| format!("Unable to open file: {}", args.path.display()))?;
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let content = line.with_context(|| format!("Unable to read line: {}", index + 1))?;
        if content.contains(&args.pattern) {
            println!("{}: {}", index + 1, content);
        }
    }
    Ok(())
}
mod box_drawing;
mod cli;
mod error;
mod formatter;
mod grid;
mod parser;
mod patterns;
mod text_align;
mod utils;

use clap::Parser;
use error::Result;
use std::io::Read;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let input_path = cli.input.clone();
    let output_path = cli.output.clone();
    let options: cli::Options = cli.try_into().map_err(|e| error::Error::InvalidInput(e))?;

    if options.verbose {
        eprintln!("ascii-fmt v{}", env!("CARGO_PKG_VERSION"));
        eprintln!("Options: {:?}", options);
    }

    let input = read_input(input_path)?;

    if options.verbose {
        eprintln!("Input: {} lines", input.lines().count());
    }

    let output = formatter::format_ascii(&input, &options)?;

    if options.dry_run {
        eprintln!("--- Dry run: showing formatted output ---");
        println!("{}", output);
        eprintln!("--- End of dry run ---");
    } else {
        write_output(output_path, &output)?;
    }

    Ok(())
}

fn read_input(path: Option<std::path::PathBuf>) -> Result<String> {
    match path {
        Some(p) => {
            std::fs::read_to_string(&p).map_err(error::Error::Io)
        }
        None => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer).map_err(error::Error::Io)?;
            Ok(buffer)
        }
    }
}

fn write_output(path: Option<std::path::PathBuf>, content: &str) -> Result<()> {
    match path {
        Some(p) => {
            std::fs::write(&p, content).map_err(error::Error::Io)
        }
        None => {
            print!("{}", content);
            Ok(())
        }
    }
}

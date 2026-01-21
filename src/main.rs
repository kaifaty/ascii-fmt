use ascii_fmt::error::Result;
use clap::Parser;
use std::io::Read;

fn main() -> Result<()> {
    let cli = ascii_fmt::cli::Cli::parse();
    let input_path = cli.input.clone();
    let output_path = cli.output.clone();
    let options: ascii_fmt::cli::Options = cli.try_into().map_err(|e| ascii_fmt::error::Error::InvalidInput(e))?;

    if options.verbose {
        eprintln!("ascii-fmt v{}", env!("CARGO_PKG_VERSION"));
        eprintln!("Options: {:?}", options);
    }

    let input = read_input(input_path)?;

    if options.verbose {
        eprintln!("Input: {} lines", input.lines().count());
    }

    let output = ascii_fmt::formatter::format_ascii(&input, &options)?;

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
            std::fs::read_to_string(&p).map_err(ascii_fmt::error::Error::Io)
        }
        None => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer).map_err(ascii_fmt::error::Error::Io)?;
            Ok(buffer)
        }
    }
}

fn write_output(path: Option<std::path::PathBuf>, content: &str) -> Result<()> {
    match path {
        Some(p) => {
            std::fs::write(&p, content).map_err(ascii_fmt::error::Error::Io)?;
            Ok(())
        }
        None => {
            print!("{}", content);
            Ok(())
        }
    }
}

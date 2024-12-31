use clap::{CommandFactory, Parser};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    /// The path of the image to convert
    pub path: Option<PathBuf>,

    /// The custom width of the ASCII art
    #[arg[long]]
    pub width: Option<u32>,

    /// The custom height of the ASCII art
    #[arg[long]]
    pub height: Option<u32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.path {
        Some(path) => {
            let ascii_art = converter::convert(&path, (args.width, args.height))?;

            println!("{ascii_art}");
        }
        None => Cli::command().print_help()?,
    }

    Ok(())
}

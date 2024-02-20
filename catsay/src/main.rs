use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use std::{
    io::{self, Read},
    path::PathBuf,
};

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    /// What does the cat say?
    message: String,
    #[clap(short = 'c', long = "cute")]
    /// Make the cat appear cute
    cute: bool,
    #[clap(short = 'f', long = "file")]
    /// Load the cat picture from the specified file
    catfile: Option<PathBuf>,
    #[clap(short = 'i', long = "stdin")]
    stdin: bool,
}
fn main() -> Result<()> {
    let options = Options::parse();
    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }
    let eye = if options.cute { "x" } else { "o" };
    let eye = format!("{}", eye.red().bold());

    if message.to_lowercase() == "woof" {
        eprintln!("The cat cannot bark like a dog");
    }
    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|| format!("Could not read file {:?}", path))?;
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!("{}", message.bright_yellow().underline().on_purple());
            println!("{}", &cat_picture);
        }
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("    ( {eye} {eye} )");
            println!("    =( I )=")
        }
    }
    Ok(())
}

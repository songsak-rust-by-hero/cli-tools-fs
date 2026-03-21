mod cli;
mod file_ops;

use clap::Parser;
use anyhow::Result;
use cli::*;
use file_ops::*;

fn main() -> Result<()> {
    let arge = Cil::parse();
    match arge.command {
        Commands::Read { path } => {
            println!("{}", read_file(&path)?);
        }
        Commands::Delete { path } => delete_file(&path)?,
        Commands::Create { path } => create_file(&path)?,
        Commands::Write { path, content } => write_file(&path, &content)?,
        Commands::Exists { path } => file_exists(&path),
    }
    Ok(())
}

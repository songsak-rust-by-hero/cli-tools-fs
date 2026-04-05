mod cli;
mod file_ops;

use crate::file_ops::{my_prelude::*, safe::safe_write};
use clap::Parser;
use cli::*;

fn main() -> Result<()> {
    let arge = Cil::parse();
    match arge.command {
        Commands::Read { path } => {
            println!("{}", read_file(&path)?);
        }
        Commands::Delete { path } => delete_file(&path)?,
        Commands::DeleteDir { path } => delete_dir(&path)?,
        Commands::Create { paths } => create_file(&paths)?,
        Commands::Write {
            path,
            content,
            safe,
        } => {
            if safe {
                safe_write(&path, &content)?;
            } else {
                write_file(&path, &content)?;
            }
        }
        Commands::Exists { path } => file_exists(&path),
        Commands::Append { path, content } => append_file(&path, &content)?,
        Commands::Information { path } => file_size(&path)?,
        Commands::Copy { src, dst } => copy_file(&src, &dst)?,
        Commands::Move { src, dst } => move_file(&src, &dst)?,
        Commands::List { path } => list_dir(&path)?,
        Commands::Mdir { path } => make_dir(&path)?,
        Commands::Count { path } => count_lines(&path)?,
        Commands::Search { path, keywold } => search_file(&path, &keywold)?,
        Commands::Tree { path, all } => {
            println!("{}", path.display());
            run_tree(&path, "".to_string(), true, all)?;
        }
    }
    Ok(())
}

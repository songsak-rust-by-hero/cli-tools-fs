use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cil {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "r")]
    Read { path: PathBuf },
    #[command(name = "w")]
    Write { path: PathBuf, content: String },
    #[command(name = "rm")]
    Delete { path: PathBuf },
    #[command(name = "rd")]
    DeleteDir { path: PathBuf },
    #[command(name = "c")]
    Create { path: PathBuf },
    #[command(name = "e")]
    Exists { path: PathBuf },
    #[command(name = "a")]
    Append { path: PathBuf, content: String },
    #[command(name = "info")]
    Information { path: PathBuf },
    #[command(name = "cp")]
    Copy { src: PathBuf, dst: PathBuf },
    #[command(name = "m")]
    Move { src: PathBuf, dst: PathBuf },
    #[command(name = "ls")]
    List {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    #[command(name = "mdir")]
    Mdir { path: PathBuf },
    #[command(name = "count")]
    Count { path: PathBuf },
    #[command(name = "s")]
    Search { path: PathBuf, keywold: String },
    #[command(name = "tree")]
    Tree {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short, long)]
        all: bool,
    },
}

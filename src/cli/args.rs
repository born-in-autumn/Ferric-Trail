use std::path::PathBuf;
use clap::{Parser, Subcommand };


#[derive(Parser, Debug)]
#[command(name = "cli", version = "0.0.1")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Open { 
        name: String,
        #[arg(short = 'a', long = "alias", help = "alias for the file")]
        alias: Option<String>,
    },
    LS  {
        path: Option<PathBuf>,
    }
}
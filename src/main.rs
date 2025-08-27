mod core;

use clap::{ Parser, Subcommand };
use core::search_files;

#[derive(Parser)]
#[command(name = "cli", version = "0.0.1")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { name: String },
    Delete { id: u32 },
}

fn main() {
    let args = Cli::parse();
    let root_path = "D:/Weixin";
    let search_term = "we";
    match args.command {
        Commands::Add { name } => {
            println!("Added: {}", name);
            search_files(root_path, search_term);
            opener::open("D:/Weixin/Weixin.exe").unwrap();
        },
        Commands::Delete { id } => println!("Deleted: {}", id),
    }
}

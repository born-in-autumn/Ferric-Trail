use clap::{  Parser, Subcommand };


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
    match args.command {
        Commands::Add { name } => println!("Added: {}", name),
        Commands::Delete { id } => println!("Deleted: {}", id),
    }
}

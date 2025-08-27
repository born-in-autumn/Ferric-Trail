use crate::cli::args::{Cli, Commands};
use crate::core::file_opener::file_open;


pub fn handle_cli(args: Cli) {
    match args.command {
        Commands::Open { name } => {
            handle_open_command(name);
        },
    }
}

fn handle_open_command(search_term: String) {
    println!("Added: {}", search_term);
    file_open(search_term.as_str());
    // opener::open("D:/Weixin/Weixin.exe").unwrap();
}
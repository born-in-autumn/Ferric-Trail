use crate::cli::args::{Cli, Commands};
use crate::core::search_files;


pub fn handle_cli(args: Cli) {
    match args.command {
        Commands::Open { name } => {
            handle_open_command(name);
        },
    }
}

fn handle_open_command(search_term: String) {
    println!("Added: {}", search_term);
    let root_path = "D:/Weixin";
    search_files(root_path, search_term.as_str());
    opener::open("D:/Weixin/Weixin.exe").unwrap();
}
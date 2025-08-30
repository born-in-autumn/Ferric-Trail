use std::path::PathBuf;
use crate::cli::args::{Cli, Commands};
use crate::core::file_opener::file_open;
use crate::core::storage::{ save_alias, is_alias };
use crate::core::file_list::file_list;
pub fn handle_cli(args: Cli) {
    match args.command {
        Commands::Open { name, alias } => {
            handle_open_command(name, alias);
        },
        Commands::LS  { path   }=> {
            handle_ls_command(path);
        }
    }
}

fn handle_open_command(search_term: String, alias: Option<String>) {
    // 开始操作之前，先检查下这个名字是不是个别名
    let name = is_alias(&search_term).unwrap_or(search_term);
    match file_open(&name) {
        Ok(_) => {
            if !alias.is_none() {
                save_alias(&name, &alias.unwrap())
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn handle_ls_command(path: Option<PathBuf>) {
    file_list(path)
}
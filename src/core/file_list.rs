use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use owo_colors::{OwoColorize};
use strum_macros::Display;
use tabled::{Table, Tabled};
use tabled::settings::object::Columns;
use tabled::settings::{Color, Style};

#[derive(Debug, Display)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled)]
struct FileEntry {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Size B")]
    len_bytes: u64,
    #[tabled(rename = "Type")]
    e_type: EntryType,
    modified: String,
}



pub fn file_list(path: Option<PathBuf>) {
    let p =  path.unwrap_or(PathBuf::from("."));
    println!("{:?}", p);
    if let Ok(does_exist)  = fs::exists(&p) {
        if does_exist {
            let get_files = get_files(&p);
            let mut table = Table::new(get_files);
            table.with(Style::rounded());
            table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
            table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
            table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
            table.modify(Columns::last(), Color::FG_BRIGHT_GREEN);
            println!("{}", table);
        }else {
            println!("{}", "Error: Path does not exist!".red());
        }
    } else {
        println!("{}", "Error: failed reading directory".red());
    }
}

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                if let Ok(meta) = fs::metadata(&file.path()) {
                    data.push(FileEntry {
                        name: file.file_name().into_string().unwrap_or("unknown  name".into()),
                        len_bytes: meta.len(),
                        e_type: if meta.is_file() {
                            EntryType::File
                        }else {
                            EntryType::Dir
                        },
                        modified: if let Ok(modi) = meta.modified() {
                            let date: DateTime<Utc> = modi.into();
                            format!("{}", date.format("%a %b %e %Y"))
                        } else {
                            String::default()
                        }
                    });
                }

            }
        }
    }
    data
}
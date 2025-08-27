use walkdir::{WalkDir, DirEntry};

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with(".")).unwrap_or(false)
}

fn is_executeable_file(entry: &DirEntry) -> bool {
    let path = entry.path();
    let entension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("").to_lowercase();

    matches!(entension.as_str(), "exe" | "bat" | "cmd" | "msi" | "com" | "ps1" | "vbs")
}

pub fn is_target_file(entry: &DirEntry, name: &str) -> bool {
    entry.file_name().to_str().map(|s| s.to_lowercase().contains(&name.to_lowercase())).unwrap_or(false)
}



pub fn search_files(root: &str, name: &str) {
    
    for entry in WalkDir::new(root)
     .into_iter()
     .filter_map(|e| e.ok())
     .filter(|entry| !is_hidden(entry)) {
        if entry.file_type().is_file() && is_target_file(&entry, name) && is_executeable_file(&entry) {
            println!("Found: {}", entry.path().display());
        }
     }
     
}

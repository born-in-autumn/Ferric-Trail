use walkdir::{WalkDir, DirEntry};
use opener;
use opener::{OpenError};
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with(".")).unwrap_or(false)
}

fn is_executable_file(entry: &DirEntry) -> bool {
    let path = entry.path();
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("").to_lowercase();
    matches!(extension.as_str(), "exe" | "lnk" | "url" )

}

pub fn is_target_file(entry: &DirEntry, name: &str) -> bool {
    entry.file_name().to_str().map(|s| s.to_lowercase().contains(&name.to_lowercase())).unwrap_or(false)
}
pub fn file_open(name: &str) -> Result<(), OpenError> {
    // 我们需要从常用的盘符开始搜索
    let username = whoami::username();
    let appdata_path = format!("C:/Users/{}/AppData/", username);
    let common_path = [("C:/ProgramData/Microsoft/Windows/Start Menu/Programs", 10), (appdata_path.as_str(), 8), ("E:/", 6), ("D:/", 5), ("C:/", 4)];
    // let common_path = [(r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs", 10)];

    // test();
    for (path, priority) in common_path {
        if let Some(result) = search_files(path, name, priority) {
            let _: Result<(), OpenError> = match opener::open(result) {
                Ok(_) => {
                    return Ok(());
                },
                Err(e) => Err(e),
            };
            break;
        }
    }
    Ok(())
}

pub fn search_files(path: &str, name: &str , priority: u32) -> Option<String> {
    let mut best_score = 0;
    let mut best_result = None;
    for entry in WalkDir::new(path)
     .into_iter()
     .filter_map(|e| e.ok())
     .filter(|entry| !is_hidden(entry)) {
        if entry.file_type().is_file() && is_target_file(&entry, name) && is_executable_file(&entry) {
            let score = calculate_file_score(&entry, name, priority);
            if score > best_score {
                best_score = score;
                best_result = Some(entry.path().display().to_string());
            }
        }
     }
     if best_score >= 18000 {
        return best_result;
     }
     None
}

fn calculate_file_score(entry: &DirEntry, name: &str, priority: u32) -> u32 {
    let mut score = priority * 1000;
    let file_name = entry.file_name();
    let search_lower = name.to_lowercase();
    if file_name.to_str().unwrap_or("") == search_lower  {
        score += 10000;
    } else if file_name.to_str().unwrap_or("").to_lowercase().starts_with(&search_lower) {
        score += 8000;
    } else if file_name.to_str().unwrap_or("").to_lowercase().contains(&search_lower) {
        score += 5000;
    }

    if is_executable_file(entry) {
        score += 3000;
        if entry.path().extension().and_then(|ext| ext.to_str()).unwrap_or("").to_lowercase() == "exe" {
            score += 2000;
            if not_main_program(file_name.to_str().unwrap_or("")) {
                score -= 15000;
            }
        }
    }
    score
}

fn not_main_program(filename: &str) -> bool {
    // 主程序特征
    let main_program_indicators = [
        "update", "installer", "setup", "uninstall", "repair",
        "config", "settings", "helper", "service", "daemon"
    ];
    
    // 如果文件名包含这些词，说明不是主程序
    main_program_indicators.iter().any(|indicator| 
        filename.to_lowercase().contains(indicator)
    )
}
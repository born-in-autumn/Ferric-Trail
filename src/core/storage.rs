use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]

pub struct Storage {
    files: Vec<File>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    name: String,
    alias: Vec<String>
}


pub fn save_alias(name: &String, alias: &String) {
    println!("name, alias, {:?} {:?}", name, alias);
    // 先检查当前是否有配置文件
    let config_path = Path::new("src/config").join("file_info.json");
    // 如果没有，则直接写
    if !config_path.exists() {
        let storage = Storage {
            files: vec!(File {
                name: name.to_owned(),
                alias: vec![alias.to_owned()],
            })
        };
        let json = serde_json::to_string_pretty(&storage).unwrap();
        fs::write(&config_path, json).unwrap();
    } else {
        // 如果有，需要反序列化查一下，再更改数据最后再次序列化
        let data = fs::read_to_string("src/config/file_info.json").unwrap();
        let mut storage: Storage = serde_json::from_str(&data).unwrap();
        if let Some(file) = storage.files.iter_mut().find(|f| f.name == *name) {
            file.alias.push(alias.to_owned());
        } else {
            // 有可能没找到，那说明这个时候来了个新文件
            storage.files.push(File {
                name: name.to_owned(),
                alias: vec![alias.to_owned()],
            })
        }
        // 4. 序列化
        let updated = serde_json::to_string_pretty(&storage).unwrap();

        // 5. 写回文件
        fs::write(&config_path, updated).unwrap();
    }

}

pub fn is_alias(alias: &String) -> Option<String> {
    let config_path = Path::new("src/config").join("file_info.json");
    if !config_path.exists() {
        None
    } else {
        let data = fs::read_to_string("src/config/file_info.json").unwrap();
        let mut storage: Storage = serde_json::from_str(&data).unwrap();
        if let Some(file) = storage.files.iter_mut().find(|f| f.alias.contains(alias)) {
            Some(file.name.clone())
        } else {
            None
        }
    }
}
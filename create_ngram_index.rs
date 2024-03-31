use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

use crate::create_ngram::NgramHashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct HashMapStruct {
    pub ngram_map: NgramHashMap,
}

pub fn create_file_path_index(file_path: String, hash_map: NgramHashMap) {
    println!("serialize save start:{}", Local::now());
    let hash_map = HashMapStruct {
        ngram_map: hash_map,
    };

    let serialized = serde_json::to_string(&hash_map).unwrap();
    fs::write(file_path, serialized).unwrap();
}

pub fn serialize_ngram_hashmap_to_json_files(file_path: String, hash_map: NgramHashMap) {
    for (key, value) in &hash_map {
        let hash_map_struct = create_hash_map_struct(key.clone(), value.clone());

        let create_dir_path = format!("{}{}", file_path, first_two_chars(key));
        create_dir_if_not_exists(create_dir_path.clone()).unwrap();

        let create_file_path =
            format!("{}/{}{}", create_dir_path.clone(), key, ".json".to_string());

        let serialized = serde_json::to_string(&hash_map_struct).unwrap();
        fs::write(create_file_path.clone(), serialized).unwrap();
    }
}

fn create_hash_map_struct(key: String, v: Vec<(i32, String, usize)>) -> HashMapStruct {
    let mut hash_map: NgramHashMap = HashMap::new();

    hash_map.insert(key.to_string(), v.clone());

    let map = HashMapStruct {
        ngram_map: hash_map,
    };

    map
}

fn create_dir_if_not_exists<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn first_two_chars(s: &str) -> String {
    s.chars().take(2).collect()
}

pub fn get_index(file_path: String) -> NgramHashMap {
    let data = fs::read_to_string(file_path).unwrap();
    let deserialized: HashMapStruct = serde_json::from_str(&data).unwrap();
    let map = deserialized.ngram_map;
    map
}

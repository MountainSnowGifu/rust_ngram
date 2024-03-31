use std::{collections::HashMap, fs, path::Path};

use crate::{
    create_ngram::{merge_maps, NgramHashMap},
    create_ngram_index::get_index,
};

pub fn get_values_by_key(map: &NgramHashMap, key: String) -> Option<&Vec<(i32, String, usize)>> {
    map.get(&key)
}

pub fn create_ngram_hash_from_key_files(key: String, index_path: String) -> NgramHashMap {
    let path = Path::new(&index_path);
    let files = find_files_in_dir(path, &key);
    let hash = create_ngram_hash(files);
    hash
}

fn create_ngram_hash(files: Vec<String>) -> NgramHashMap {
    let mut result: NgramHashMap = HashMap::new();
    for file in files {
        println!("{}", file);
        let ngram_index = get_index(file);
        merge_maps(&mut result, ngram_index.clone());
    }
    result
}

fn find_files_in_dir(dir: &Path, file_to_find: &str) -> Vec<String> {
    let mut matches = Vec::new(); // Initialize an empty vector to store matching file paths

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            // Check if the file name contains the desired string and is a file
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.contains(file_to_find) {
                            matches.push(path.to_string_lossy().to_string()); // Add matching file to the vector
                        }
                    }
                }
            } else if path.is_dir() {
                // If it's a directory, search recursively and append any found files to the matches
                matches.extend(find_files_in_dir(&path, file_to_find));
            }
        }
    }
    matches // Return the vector containing all matching file paths
}

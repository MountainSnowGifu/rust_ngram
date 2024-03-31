//https://qiita.com/someone7140/items/a50f104aef535f2f70bcs
//【Rust】MacでVSCodeでのLLDBのdebugが動かない

extern crate chrono;

use crate::create_ngram::NgramHashMap;
use chrono::Local;

mod create_ngram;
mod create_ngram_index;
mod dumy_text;
mod search_ngram;

fn main() {
    println!("push start:{}", Local::now());

    let excels = dumy_text::get_dummy_date();
    let hash_map = create_ngram::merge_sheet_pairs_from_excels(excels);

    let path_index_file_path = "ngram-index/output.json".to_string();

    create_ngram_index::create_file_path_index(path_index_file_path.clone(), hash_map.clone());

    //デシリアライズ
    let map: NgramHashMap = create_ngram_index::get_index(path_index_file_path.clone());
    println!("len:{:?}", map.len());

    let ngram_index_file_path = "ngram-index/".to_string();

    create_ngram_index::serialize_ngram_hashmap_to_json_files(
        ngram_index_file_path.clone(),
        hash_map.clone(),
    );

    let values =
        search_ngram::create_ngram_hash_from_key_files("明日".to_string(), ngram_index_file_path);
    let result = search_ngram::get_values_by_key(&values, "明日".to_string()).unwrap();
    println!("明日:{:?}", result);

    println!("deserialized end:{}", Local::now());
}

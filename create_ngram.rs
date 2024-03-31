use crate::dumy_text::Excel;
use std::collections::HashMap;

pub type NgramHashMap = HashMap<String, Vec<(i32, String, usize)>>;

const NGRAM_N: usize = 2;

pub fn merge_sheet_pairs_from_excels(excels: Vec<Excel>) -> NgramHashMap {
    let mut result: NgramHashMap = HashMap::new();

    for excel in excels {
        let excel_path_id = excel.path_id;
        for sheet in excel.sheets {
            let hash_map =
                extract_pairs(excel_path_id, sheet.sheet_name, sheet.sheet_text, NGRAM_N);
            merge_maps(&mut result, hash_map.clone());
        }
    }

    result
}

pub fn extract_pairs(path_id: i32, sheet_name: String, input: String, n: usize) -> NgramHashMap {
    let mut result = HashMap::new();
    let chars: Vec<char> = input.chars().collect();
    for i in 0..=chars.len().saturating_sub(n) {
        let pair: String = chars[i..i + n].iter().collect();
        result
            .entry(pair)
            .or_insert_with(Vec::new)
            .push((path_id, sheet_name.clone(), i));
    }
    result
}

pub fn merge_maps(map1: &mut NgramHashMap, map2: NgramHashMap) {
    for (key, value) in map2 {
        map1.entry(key).or_insert_with(Vec::new).extend(value);
    }
}

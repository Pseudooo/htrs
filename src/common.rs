use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

pub fn get_params_from_path(path: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\{([A-Za-z0-1]|_|-)+}").unwrap();
    }
    RE.find_iter(path)
        .filter_map(|s| s.as_str().parse().ok())
        .map(|s: String| s[1..s.len() - 1].to_string())
        .collect()
}

pub fn get_duplicates_from_vec(vec: Vec<String>) -> Vec<String> {
    let mut duplicates: HashSet<String> = HashSet::new();
    let mut set: HashSet<String> = HashSet::new();
    for value in vec {
        if !set.contains(&value) {
            set.insert(value);
        } else {
            duplicates.insert(value);
        }
    }
    duplicates.into_iter().collect()
}

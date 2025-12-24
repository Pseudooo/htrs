use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

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

/// Attempts to parse a string in the format of `key=value` into its constituent parts
pub fn parse_key_value_string(s: &str) -> Result<(String, String), ()> {
    let (left, right) = match s.split_once("=") {
        Some(x) => x,
        None => return Err(()),
    };

    if left.is_empty() || right.is_empty() {
        return Err(());
    }

    Ok((left.to_string(), right.to_string()))
}

/// Merges two given HashMaps into a new HashMap, if a value is present in both hashmaps the value
/// provided in HashMap `b` will be used in the returned HashMap
pub fn merge_hashmaps<T: Eq + Hash, U>(a: HashMap<T, U>, b: HashMap<T, U>) -> HashMap<T, U> {
    let mut merged = HashMap::new();
    for (key, value) in a {
        merged.insert(key, value);
    }
    for (key, value) in b {
        merged.insert(key, value);
    }

    merged
}

#[cfg(test)]
mod common_tests {
    use crate::common::parse_key_value_string;
    use rstest::rstest;

    #[test]
    fn given_valid_key_value_string_then_should_parse() {
        let s = "key=value";

        let result = parse_key_value_string(s);
        assert!(result.is_ok());
        let (left, right) = result.unwrap();
        assert_eq!(left, "key");
        assert_eq!(right, "value");
    }

    #[rstest]
    #[case("")]
    #[case("foo")]
    #[case("key=")]
    #[case("=value")]
    fn given_invalid_key_value_string_then_should_fail(
        #[case] s: &str,
    ) {
        let result = parse_key_value_string(s);
        assert!(result.is_err());
    }
}

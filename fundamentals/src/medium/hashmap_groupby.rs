/*
  Problem 32: HashMap — Group By First Letter

  Write a function that takes a Vec<String> and returns a HashMap<char, Vec<String>>
  where words are grouped by their first character (lowercase).
  Ignore empty strings.

  Run the tests for this problem with:
    cargo test --test hashmap_groupby_test
*/

use std::collections::HashMap;

pub fn group_by_first_letter(words: Vec<String>) -> HashMap<char, Vec<String>> {
    let mut word_map = HashMap::new();
    if words.is_empty() {
        return word_map;
    }

    for word in words.iter() {
        if word.trim().is_empty() {
            continue;
        }

        let first_char = word.chars().next().unwrap();
        let entry = word_map.entry(first_char).or_insert(Vec::new());
        entry.push(word.to_string());
    }

    word_map
}

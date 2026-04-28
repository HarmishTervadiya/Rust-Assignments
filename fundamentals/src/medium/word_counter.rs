/*
  Problem 19: Word Counter

  Write a function that takes a &str and returns a HashMap<String, usize>
  where each key is a lowercase word and each value is the number of occurrences.
  Split on whitespace and convert to lowercase.

  Run the tests for this problem with:
    cargo test --test word_counter_test
*/

use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();

    for word in text.to_lowercase().split_whitespace() {
        map.entry(word.to_string())
            .or_insert(text.to_lowercase().match_indices(word).count());
    }

    map
}

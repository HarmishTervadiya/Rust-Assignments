/*
  Problem 33: String Compression

  Write a function that performs basic string compression using the counts of repeated characters.
  For example, "aabcccccaaa" becomes "a2b1c5a3".
  If the compressed string is not shorter than the original, return the original string.

  Run the tests for this problem with:
    cargo test --test string_compression_test
*/

pub fn compress(s: &str) -> String {
    let mut compressed_string = String::new();

    if s.is_empty() {
        return s.to_string();
    }

    let mut check_var = s.chars().next().unwrap();
    let mut count = 0;

    for (i, char) in s.chars().enumerate() {
        // compressed_string.push(char);
        // compressed_string.push();
        if char == check_var {
            count += 1;
        }
        
        if char != check_var || (i == s.len() - 1) {
            compressed_string += &format!("{}{}", &check_var, count);
            check_var = char;
            count = 1;
        }

        println!("{}{}", &check_var, count);
        println!("{}", compressed_string);
    }

    if compressed_string.len() >= s.len() {
        s.to_string()
    } else {
        compressed_string
    }
}

fn main() {
    assert_eq!(compress("aaaa"), "a4");
}

/*
  Problem 29: Lifetime — Longest String

  Write a function that takes two string slices and returns the longer one.
  If they are equal length, return the first.
  You must annotate lifetimes correctly so the return value lives as long as both inputs.

  Run the tests for this problem with:
    cargo test --test lifetime_longest_test
*/

pub fn longest<'a>(a: &'a str, b: &'a str)-> & 'a str {
  if a.len() >= b.len() { a } else {b}
}

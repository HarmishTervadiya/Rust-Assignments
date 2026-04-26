/*
  Problem 28: Generic — Find Max

  Write a generic function that takes a slice of items implementing PartialOrd
  and returns Option<&T> for the maximum element.
  Do not use any built-in max functions.

  Run the tests for this problem with:
    cargo test --test generic_find_max_test
*/

pub fn find_max<T: PartialOrd>(items: &[T]) -> Option<&T> {
    if items.is_empty() {
        return None;
    }

    let mut i = 0;
    let mut currect_max=&items[0];
    while i < items.len() {
        if items[i] > *currect_max {
          currect_max=&items[i];
        }
        i+=1;
    }
    
    Some(currect_max)
}

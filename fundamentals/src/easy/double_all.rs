/*
  Problem 10: Mutable Reference — Double All

  Write a function that takes a mutable reference to a Vec<i32> and doubles every element in-place.
  The function should return nothing (unit type).

  Run the tests for this problem with:
    cargo test --test double_all_test
*/

pub fn double_all(nums: &mut Vec<i32>) {
    *nums = nums.iter_mut().map(|x| *x * 2).collect();

    // for num in nums{
    //   *num= *num * 2;
    // }
}

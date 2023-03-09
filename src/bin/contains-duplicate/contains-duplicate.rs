use std::collections::HashSet;
use std::ops::ControlFlow;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut num_set = HashSet::new();

        for num in &nums {
            if num_set.contains(num) {
                return true;
            }
            num_set.insert(*num);
        }

        false
    }

    pub fn better_contains_duplicate(nums: Vec<i32>) -> bool {
        let mut num_set = HashSet::with_capacity(nums.len());

        for num in &nums {
            if !num_set.insert(*num) {
                return true;
            }
        }
        false
    }

    pub fn using_iter_without_collecting_contains_duplicate(nums: Vec<i32>) -> bool {
        let mut num_set = HashSet::with_capacity(nums.len());

        nums.iter()
            .try_for_each(|x| {
                if !num_set.insert(*x) {
                    return ControlFlow::Break(true);
                }
                ControlFlow::Continue(())
            })
            .is_break()
            || false
    }
}

fn main() {
    let num_vec = vec![1, 2, 3, 4];
    println!("{}", Solution::contains_duplicate(num_vec.clone()));
    println!("{}", Solution::better_contains_duplicate(num_vec.clone()));
    println!(
        "{}",
        Solution::using_iter_without_collecting_contains_duplicate(num_vec.clone())
    );
}

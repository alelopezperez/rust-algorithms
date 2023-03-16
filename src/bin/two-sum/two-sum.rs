use std::collections::HashMap;
enum Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut set = HashMap::<i32, i32>::with_capacity(nums.len());

        for (i, value) in nums.iter().enumerate() {
            if set.contains_key(&(target - *value)) {
                return vec![set[&(target - *value)], i as i32];
            }

            set.insert(*value, i as i32);
        }

        vec![0, 0]
    }
}

fn main() {
    let ans = Solution::two_sum(vec![1, 2, 4, 5, 6], 6);

    println!("{:?}", ans);
}

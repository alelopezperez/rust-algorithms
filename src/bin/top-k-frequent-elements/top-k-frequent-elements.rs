use std::collections::BinaryHeap;
use std::collections::HashMap;
enum Solution {}
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut h_map = HashMap::new();

        for elem in nums {
            let count = h_map.entry(elem).or_insert(0);
            *count += 1;
        }

        println!("{:?}", h_map);

        let mut freq_heap = BinaryHeap::new();

        for (key, val) in h_map {
            freq_heap.push((-val, key));
            if freq_heap.len() > k as usize {
                freq_heap.pop();
            }
        }

        freq_heap
            .into_sorted_vec()
            .iter()
            .rev()
            .take(k as usize)
            .map(|elem| elem.1)
            .collect::<Vec<i32>>()
    }
}

fn main() {
    Solution::top_k_frequent(vec![1, 2, 2, 4, 4], 2);
}

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut char_count = HashMap::with_capacity(s.len());

        for letter in s.chars() {
            let count = char_count.entry(letter).or_insert(0);
            *count += 1;
        }

        for letter in t.chars() {
            if !char_count.contains_key(&letter) {
                return false;
            }

            char_count.entry(letter).and_modify(|x| *x -= 1);
        }

        for (_, v) in &char_count {
            if *v != 0 {
                return false;
            }
        }

        true
    }
    pub fn is_anagram_some_iter(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut char_count = HashMap::with_capacity(s.len());

        s.chars()
            .for_each(|letter| *char_count.entry(letter).or_insert(0) += 1);

        for letter in t.chars() {
            if !char_count.contains_key(&letter) {
                return false;
            }

            char_count.entry(letter).and_modify(|x| *x -= 1);
        }

        char_count.into_values().all(|x| x == 0)
    }

    pub fn is_anagram_optimized_for_memory(s: String, t: String) -> bool {
        let mut char_count = [0 as u16; 26];

        if s.len() != t.len() {
            return false;
        }

        s.chars().for_each(|l| {
            char_count[l as usize - 'a' as usize] += 1;
        });

        for l in t.chars() {
            if char_count[l as usize - 'a' as usize] == 0 {
                return false;
            }
            char_count[l as usize - 'a' as usize] -= 1;
        }

        char_count.iter().all(|x| *x == 0)
    }
}

fn main() {
    println!(
        "{}",
        Solution::is_anagram(String::from("anagram"), String::from("gramana"))
    );
    println!(
        "{}",
        Solution::is_anagram_some_iter(String::from("anagram"), String::from("gramana"))
    );
    println!(
        "{}",
        Solution::is_anagram_optimized_for_memory(String::from("anagram"), String::from("gramana"))
    );
}

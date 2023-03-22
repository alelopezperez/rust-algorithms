use std::collections::HashMap;

fn main() {
    let strs = String::from("Asd");
    let mut letter_set = vec![0; 26];

    let fol = letter_set.iter().fold(true, |acc, item| acc && *item == 0);
}

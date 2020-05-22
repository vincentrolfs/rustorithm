// https://www.techiedelight.com/find-pair-with-given-sum-array/

use std::collections::HashMap;

pub fn main() {
    let arr = [8, 7, 4, 2, 4, 19, 2, 3, 1, 4, 1, 2];
    let goal_sum = 10;
    let mut lookup = HashMap::new();

    for (index, &value) in arr.iter().enumerate() {
        let partner_value = goal_sum - value;
        match lookup.get(&partner_value) {
            Some(partner_index) => {
                println!("Found pair with goal sum {}:", goal_sum);
                println!("Index {}, value {}", partner_index, partner_value);
                println!("Index {}, value {}", index, value);
                return;
            },
            _ if lookup.contains_key(&value) => (),
            _ => {
                lookup.insert(value, index);
            }
        };
    }

    println!("No pair with goal sum {} was found.", goal_sum);
}

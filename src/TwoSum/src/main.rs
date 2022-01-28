use std::collections::HashMap;

fn main() {
   println!("{:?}",two_sum(vec![3,2,4],6))
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map = HashMap::new();
    for (index, value) in nums.iter().enumerate() {
        match hash_map.get(&(target - value)){
            Some(&item) => return vec![item as i32, index as i32],
            None => hash_map.insert(value, index)
        };
    }
    vec![]
}
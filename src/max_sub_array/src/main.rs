fn main() {
    println!("{}",max_sub_array(vec![-1]));
}

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];
    let mut cur_max_section = nums[0];

    for i in 1..nums.len() {
        cur_max_section = if cur_max_section < 0 {
            nums[i]
        } else {
            cur_max_section + nums[i]
        };
        max = max.max(cur_max_section);
    }
    max
}
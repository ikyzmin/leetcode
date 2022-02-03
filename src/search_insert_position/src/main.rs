fn main() {
    println!("{}", search_insert(vec![0, 1], 1));
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut mid = nums.len() / 2;
    while right != mid && left != mid {
        if target > nums[mid] {
            left = mid;
        } else if target < nums[mid] {
            right = mid;
        } else {
            return mid as i32;
        }
        mid = (left + right) / 2
    }
    println!("{} {} {}", left, mid, right);
    let mut result_index = 0;
    if target < nums[left] { result_index = (left as i32) - 1; }
    else if target == nums[left] { result_index = left as i32 } 
    else if target > nums[right] { result_index = (right as i32) + 1; } else {
        result_index = right as i32;
    }

    if result_index < 0 { return 0; } else {
        result_index
    }
}
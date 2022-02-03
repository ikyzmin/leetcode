fn main() {
    println!("{:?}", plus_one(vec![1,0,0,9]));
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; digits.len()];
    let mut delayed = 0;
    for i in (0..digits.len()).rev() {
        let mut cur = if i == digits.len() - 1 { digits[i] + 1 } else { digits[i] + delayed };
        if cur != 0 && cur % 10 == 0 {
            cur %= 10;
            delayed = 1;
        } else {
            delayed = 0;
        }
        result[i] = cur;
    }
    if delayed == 1 {
        result.insert(0, 1)
    }
    result
}
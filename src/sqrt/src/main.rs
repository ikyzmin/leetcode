fn main() {
    println!("{}", my_sqrt(2147395599));
}

pub fn my_sqrt(x: i32) -> i32 {
    let mut left = 1;
    let mut right = x;
    let mut mid = (1 + x) / 2;
    while left != mid && right != mid {
       // println!("{} {} {}", left, mid, right);
        if x / mid > mid {
            left = mid;
            mid = (left + right) / 2;
        } else if x / mid < mid {
            right = mid;
            mid = (left + right) / 2;
        } else {
            return mid;
        }
    }
  //  println!("{} {} {}", left, mid, right);
    if left * left <= x { return left; }
    if right * right <= x { return right; }
    0
}

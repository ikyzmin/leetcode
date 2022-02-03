fn main() {
    println!("{}",climb_stairs(4));
}

pub fn climb_stairs(n: i32) -> i32 {
    if n == 0 { return 0; };
    if n == 1 { return 1; };
    if n == 2 { return 2; };
    let mut n1 = 1;
    let mut n2 = 2;
    let mut result = 0;
    for _ in 2..n {
        result = n1 + n2;
        n1 = n2;
        n2 = result;
    }
    result
}

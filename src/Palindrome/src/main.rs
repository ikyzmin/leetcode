fn main() {
    print!("{}", is_palindrome(-12021));
}

pub fn is_palindrome(x: i32) -> bool {
    let mut temp = x;
    let mut reverse = 0;
    let mut first = true;
    while reverse < temp {
        let number = temp % 10;
        reverse = if first {
            number
        } else {
            if reverse > 0 {
                reverse * 10 + number
            } else {
                number * 10
            }
        };

        if reverse > temp || reverse == temp {
            break;
        }
        temp /= 10;
        first = false;
    }
    reverse == temp
}

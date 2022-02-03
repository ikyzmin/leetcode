use std::collections::HashMap;

fn main() {
    print!("{}", roman_to_int(String::from("III")));
}
/*
Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000*/


pub fn roman_to_int(s: String) -> i32 {
    let romanian_map: HashMap<char, i32> = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ].iter().cloned().collect();
    let mut result = 0;
    let mut prev = 'I';
    for romanian in s.chars().rev() {
        let value = romanian_map.get(&romanian);
        let prev_value = romanian_map.get(&prev);
        if let Some(&prev) = prev_value {
            if let Some(&i) = value {
                if i >= prev {
                    result += i;
                } else {
                    result = result - i;
                }
            }
        }
        prev = romanian;
    }
    result
}

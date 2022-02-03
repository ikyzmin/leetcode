use std::ops::Index;

fn main() {
    println!("{}", str_str(String::from("hello"), String::from("ll")));
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.len() > haystack.len() { return -1; }
    if needle.is_empty() { return 0; }

    for i in 0..=(haystack.len() - needle.len()) {
            if haystack[i..i + needle.len()] == needle {
                return i as i32;
            }
    }

    -1
}

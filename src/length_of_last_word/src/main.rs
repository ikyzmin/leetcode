fn main() {
    println!("{}", length_of_last_word(String::from("   fly me   to   the moon  ")));
}

pub fn length_of_last_word(s: String) -> i32 {
    if let Some(word) = s.split_whitespace().last() {
        word.len() as i32
    } else {
        0
    }
}

fn main() {
    println!("{}", is_valid(String::from("({[[][]]})")));
}

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for ch in s.chars() {
        if let Some(prev_item) = stack.pop() {
            match prev_item {
                '(' => if ch != ')' {
                    stack.push(prev_item);
                    stack.push(ch);
                }
                '[' => if ch != ']' {
                    stack.push(prev_item);
                    stack.push(ch);
                }
                '{' => if ch != '}' {
                    stack.push(prev_item);
                    stack.push(ch);
                }
                _ => {
                    stack.push(prev_item);
                    stack.push(ch);
                }
            }
        } else {
            stack.push(ch);
        }
    }
    stack.is_empty()
}
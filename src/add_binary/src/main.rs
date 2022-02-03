fn main() {
    println!("{}", add_binary(String::from("111"), String::from("111")));
}

pub fn add_binary(a: String, b: String) -> String {
    let mut result = String::new();
    let mut hold = false;
    let (max_string, min_string, diff) = if a.len() > b.len() {
        (a.as_bytes(), b.as_bytes(), a.len() - b.len())
    } else {
        (b.as_bytes(), a.as_bytes(), b.len() - a.len())
    };
    for i in (0..max_string.len()).rev() {
        if (i as i32 - diff as i32) < 0 {
            let cur_result = if hold {
                if max_string[i] == '1' as u8{
                    '0'
                }else{
                    hold = false;
                    '1'
                }
            }else{
                max_string[i] as char
            };
            result.insert(0, cur_result);
            continue;
        }
        let cur_result = if max_string[i] != min_string[i - diff] {
            if hold { '0' } else {
                '1'
            }
        } else if max_string[i] == '0' as u8 && min_string[i - diff] == '0' as u8 {
            if hold {
                hold = false;
                '1'
            } else {
                '0'
            }
        } else {
            if hold { '1' } else {
                hold = true;
                '0'
            }
        };
        result.insert(0, cur_result);
    }
    if hold {
        result.insert(0, '1');
    }
    result
}

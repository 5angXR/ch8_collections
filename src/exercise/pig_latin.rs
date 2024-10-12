#[allow(dead_code)]
fn is_vowel_str(s: &str) -> bool {
    s.starts_with(['a', 'i', 'u', 'e', 'o',
                   'A', 'I', 'U', 'E', 'O',])
}

fn is_vowel_char(c: char) -> bool {
    c == 'a' || c == 'i' || c == 'u' || c == 'e' || c == 'o'||
    c == 'A' || c == 'I' || c == 'U' || c == 'E' || c == 'O'
}

fn find_first_vowel(s: &str) -> Result<usize, &'static str> {
    for (i, c) in s.char_indices() {
        if is_vowel_char(c) {
            return Ok(i);
        }
    }
    Err("no vowel")
}

pub fn strings_to_pig_latin(ori_str: &str) -> Result<String, &'static str> {
    if !ori_str.is_ascii() {
        return Err("Not a ascii string");
    }
    
    let words = ori_str.split_ascii_whitespace();
    let mut output = String::new();
    for word in words {
        match find_first_vowel(word) {
            Ok(index) => {
                if index == 0 {
                    output.push_str(format!("{word}-hay, ").as_str());
                } else {
                    let (first, last) = word.split_at(index);
                    output.push_str(format!("{last}-{first}ay, ").as_str());
                }
            }
            _ => return Err("err")
        }
    }
    Ok(output)
}
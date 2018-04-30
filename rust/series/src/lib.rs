pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    match split(digits, len) {
        None => {}
        Some(s) => {
            vec.push(s);
            if digits.len() > 0 {
                vec.append(&mut series(&digits[1..], len));
            }
        }
    }

    vec
}

fn split(digits: &str, len: usize) -> Option<String> {
    if digits.len() >= len {
        return Some(digits.chars().take(len).collect());
    }

    None
}

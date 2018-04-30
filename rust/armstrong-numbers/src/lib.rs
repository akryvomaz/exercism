pub fn is_armstrong_number(num: u32) -> bool {
    let len: u32 = num.to_string().len() as u32;

    num
        .to_string()
        .chars()
        .fold(0, |sum, d| sum + char_to_pow_number(d, len))
    == num
}

fn char_to_pow_number(c: char, square: u32) -> u32 {
    c
        .to_string()
        .parse::<u32>()
        .unwrap()
        .pow(square)
}

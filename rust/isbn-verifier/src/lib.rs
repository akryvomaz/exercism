/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut vec: Vec<usize> = Vec::new();
    let last_index = isbn.len() - 1;

    for (index, c) in isbn.chars().enumerate() {
        if c == 'X' {
            if index != last_index {
                break;
            }
            vec.push(10);
        } else if c.is_numeric() {
            vec.push(c.to_string().parse().unwrap());
        }
    }

    if vec.len() != 10 {
        return false;
    }

    let mut count: usize = vec.len();
    vec.iter().fold(0, |mut sum, n| {
        sum += n * count;
        count -= 1;

        sum
    }) % 11 == 0
}

use std::collections::HashSet;

pub fn check(value: &str) -> bool {
    let mut hash: HashSet<char> = HashSet::new();

    value
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| hash.insert(c))
}

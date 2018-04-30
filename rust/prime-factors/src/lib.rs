pub fn factors(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();

    match (2..n + 1).find(|i| n % i == 0) {
        Some(p) => {
            result.push(p);
            result.append(&mut factors(n / p));
        }
        None => {}
    }

    result
}

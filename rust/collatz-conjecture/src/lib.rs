pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    Some(count_collatz(n, 0))
}

fn count_collatz(n: u64, count: u64) -> u64 {
    match n {
        1 => count,
        n if n % 2 == 0 => count_collatz(n / 2, count + 1),
        _ => count_collatz((n * 3) + 1, count + 1)
    }
}

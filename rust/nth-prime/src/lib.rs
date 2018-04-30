pub fn nth<'a>(n: u32) -> Result<u32, &'a str> {
    match n {
        0 => Err("zero"),
        _ => {
            let mut count = 0;
            for x in 1.. {
                if is_prime(x) {
                    count += 1;
                }
                if count == n {
                    return Ok(x);
                }
            }
            Err("prime not found")
        }
    }
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if (n == 2) || (n == 3) {
        return true;
    }
    if (n % 2 == 0) || (n % 3 == 0) {
        return false;
    }

    let mut i = 5;
    let mut w = 2;
    while i < n {
        if n % i == 0 {
            return false;
        }
        i += w;
        w = 6 - w;
    }

    true
}

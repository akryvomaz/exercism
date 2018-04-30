pub fn encode(value: &str) -> String {
    let mut count: u32 = 1;

    let mut result: String = value
        .chars()
        .rev()
        .fold(String::from(""), |mut acc, c| match acc.chars().last() {
            None => {
                acc.push(c);
                acc
            }
            Some(last) => {
                if last == c {
                    count += 1;
                } else {
                    if count > 1 {
                        for n in count.to_string().chars().rev() {
                            acc.push(n);
                        }
                        count = 1;
                    }
                    acc.push(c);
                }
                acc
            }
        })
        .chars()
        .rev()
        .collect();

    if count > 1 {
        result.insert_str(0, &count.to_string());
    }

    result
}

pub fn decode(value: &str) -> String {
    let mut tmp: String = String::from("");

    value.chars().fold(String::from(""), |mut acc, c| {
        if c.is_numeric() {
            tmp.push(c);
            return acc;
        }

        let n: usize = tmp.parse().unwrap_or(1);
        tmp.clear();

        acc.push_str(&c.to_string().repeat(n));

        acc
    })
}

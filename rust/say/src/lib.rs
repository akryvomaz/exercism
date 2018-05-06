pub fn encode(number: u64) -> String {
    if number == 0 {
        return String::from("zero");
    }

    let hundreds: Vec<Vec<char>> = split_in_hundreds(number);

    let mut len: usize = hundreds.len();

    hundreds.into_iter().fold(String::from(""), |acc, h| {
        let hundred: String = convert_hundred(h);
        len = len - 1;

        match len {
            0 => {
                if acc.is_empty() {
                    return hundred;
                } else if !hundred.is_empty() {
                    return format!("{} {}", acc, hundred);
                }

                acc
            }
            _ => {
                let thousand: String = thousands(len);

                if !hundred.is_empty() {
                    if !acc.is_empty() {
                        return format!("{} {} {}", acc, hundred, thousand);
                    } else {
                        return format!("{} {}", hundred, thousand);
                    }
                }

                acc
            }
        }
    })
}

fn split_in_hundreds(number: u64) -> Vec<Vec<char>> {
    let mut hundreds: Vec<Vec<char>> = Vec::new();

    let mut tmp: Vec<char> = Vec::new();
    for c in number.to_string().chars().rev() {
        match tmp.len() {
            2 => {
                tmp.insert(0, c);
                hundreds.push(tmp.clone());
                tmp.clear();
            }
            _ => tmp.insert(0, c),
        }
    }
    if !tmp.is_empty() {
        hundreds.push(tmp.clone());
    }

    hundreds.into_iter().rev().map(|h| h).collect()
}

fn convert_hundred(chars: Vec<char>) -> String {
    match chars.len() {
        1 => from_single_number(chars[0]),
        2 => from_double_number(chars[0], chars[1]),
        3 => {
            let hundred: String = from_single_number(chars[0]);
            let tens: String = from_double_number(chars[1], chars[2]);

            if hundred.is_empty() {
                return tens;
            }

            if tens.is_empty() {
                return format!("{} hundred", hundred);
            }

            format!("{} hundred {}", hundred, tens)
        }
        _ => String::from(""),
    }
}

fn from_single_number(number: char) -> String {
    match number {
        '1' => String::from("one"),
        '2' => String::from("two"),
        '3' => String::from("three"),
        '4' => String::from("four"),
        '5' => String::from("five"),
        '6' => String::from("six"),
        '7' => String::from("seven"),
        '8' => String::from("eight"),
        '9' => String::from("nine"),
        _ => String::from(""),
    }
}

fn from_double_number(first_number: char, second_number: char) -> String {
    if first_number == '0' {
        return from_single_number(second_number);
    }

    if first_number == '1' {
        return match second_number {
            '0' => String::from("ten"),
            '1' => String::from("eleven"),
            '2' => String::from("twelve"),
            '3' => String::from("thirteen"),
            '4' => String::from("fourteen"),
            '5' => String::from("fifteen"),
            '6' => String::from("sixteen"),
            '7' => String::from("seventeen"),
            '8' => String::from("eighteen"),
            '9' => String::from("nineteen"),
            _ => String::from(""),
        };
    }

    let first: String = match first_number {
        '2' => String::from("twenty"),
        '3' => String::from("thirty"),
        '4' => String::from("forty"),
        '5' => String::from("fifty"),
        '6' => String::from("sixty"),
        '7' => String::from("seventy"),
        '8' => String::from("eighty"),
        '9' => String::from("ninety"),
        _ => String::from(""),
    };

    if second_number == '0' {
        return first;
    }

    format!("{}-{}", first, from_single_number(second_number))
}

fn thousands(len: usize) -> String {
    match len {
        1 => String::from("thousand"),
        2 => String::from("million"),
        3 => String::from("billion"),
        4 => String::from("trillion"),
        5 => String::from("quadrillion"),
        6 => String::from("quintillion"),
        _ => String::from(""),
    }
}

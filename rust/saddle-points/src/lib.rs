pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut vec: Vec<(usize, usize)> = Vec::new();

    if input.len() == 0 {
        return vec;
    }

    let first: Option<&Vec<u64>> = input.iter().next();
    if first == None {
        return vec;
    }

    let len: usize = first.unwrap().len();
    if len == 0 {
        return vec;
    }

    let mut row_max: Vec<u64> = Vec::new();
    let mut column_min: Vec<u64> = Vec::new();

    for row in input {
        row_max.push(*row.iter().max().unwrap());
    }

    for column_index in 0..len {
        let mut min: Option<u64> = None;

        for row in input {
            let value: u64 = *row.iter().skip(column_index).next().unwrap();

            match min {
                None => min = Some(value),
                Some(prev) => {
                    if value < prev {
                        min = Some(value);
                    }
                }
            };
        }

        column_min.push(min.unwrap());
    }

    for (column_index, column_value) in column_min.iter().enumerate() {
        for (row_index, row_value) in row_max.iter().enumerate() {
            if column_value == row_value {
                vec.push((row_index, column_index));
            }
        }
    }

    vec
}

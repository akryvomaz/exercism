pub fn square_of_sum(n: usize) -> usize {
    let mut sum: usize = 0;

    for i in 1..n + 1 {
        sum += i;
    }

    sum * sum
}

pub fn sum_of_squares(n: usize) -> usize {
    let mut sum: usize = 0;

    for i in 1..n + 1 {
        sum += i * i;
    }

    sum
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}

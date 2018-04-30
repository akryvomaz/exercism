pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut vec: Vec<u32> = Vec::new();

    for i in 1..limit {
        for j in factors {
            if i % j == 0 {
                vec.push(i);
                break;
            }
        }
    }

    vec.iter().sum()
}

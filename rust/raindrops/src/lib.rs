pub fn raindrops(n: usize) -> String {
    let factors = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

    let result: String = factors
        .iter()
        .map(|factor| if n % factor.0 == 0 { factor.1 } else { "" })
        .collect();

    if result.is_empty() {
        return n.to_string();
    }

    result
}

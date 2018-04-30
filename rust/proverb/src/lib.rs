pub fn build_proverb(list: Vec<&str>) -> String {
    match list.first() {
        None => String::from(""),
        Some(first) => {
            let mut result: String = String::from("");

            let mut previous: &str = first;
            for next in list.iter().skip(1) {
                result.push_str(&format!("For want of a {} the {} was lost.\n", previous, next));
                previous = next;
            }

            result.push_str(&format!("And all for the want of a {}.", first));

            result
        }
    }
}

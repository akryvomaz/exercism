const SUM: u32 = 1000;

pub fn find() -> Option<u32> {
    let tmp: u32 = (SUM * SUM) / 2;

    for a in 1..SUM {
        let b: u32 = (tmp - SUM * a) / (SUM - a);
        let c: u32 = SUM - (a + b);

        if a * a + b * b == c * c {
            return Some(a * b * c);
        }
    }

    None
}

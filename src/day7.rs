pub fn optimize_crabs(values: &mut[u32]) -> u32 {
    // Simply find the median of the sequence, and then the
    // total variance from the median
    values.sort_unstable();

    let div = values.len() / 2;
    let rem = values.len() % 2;
    let median = if rem == 0 { values[div] } else { (values[div] + values[div + 1]) / 2 };

    values
        .iter()
        .fold(0, |acc, x| acc + x.abs_diff(median))
}

#[cfg(test)]
mod answers {
    use super::*;

    #[test]
    fn example1() {
        let mut crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(optimize_crabs(&mut crabs), 37);
    }

    #[test]
    fn puzzle1() {
        let mut crabs: Vec<u32> = include_str!("./input/day7")
            .split_terminator(',')
            .map(|x| x.trim().parse().unwrap())
            .collect();

            assert_eq!(optimize_crabs(&mut crabs), 359648);
    }
}

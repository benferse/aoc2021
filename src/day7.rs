pub fn constant_burn(x: u32) -> u32 {
    x
}

pub fn linear_burn(x: u32) -> u32 {
    (x * (x + 1)) / 2
}

pub fn geometric_median(values: &[u32]) -> u32 {
    let mut copied = values.to_vec();
    copied.sort_unstable();

    let div = copied.len() / 2;
    let rem = copied.len() % 2;

    if rem == 0 { copied[div] } else { (copied[div] + copied[div + 1]) / 2 }
}

pub fn arithmetic_mean(values: &[u32]) -> u32 {
    let sum: u32 = values.iter().sum();
    let count = values.len() as u32;
    sum / count
}

pub fn optimize_crabs<B, M>(values: &mut[u32], midpoint_selector: M, burn_rate: B) -> u32
where B: Fn(u32) -> u32, M: Fn(&[u32]) -> u32 {
    let median = midpoint_selector(values);

    let x = values
        .iter()
        .fold(0, |acc, x| acc + burn_rate(x.abs_diff(median)));
    let y = values
        .iter()
        .fold(0, |acc, x| acc + burn_rate(x.abs_diff(median+1)));

    if x < y { x } else { y }
}

#[cfg(test)]
mod answers {
    use super::*;

    #[test]
    fn example1() {
        let mut crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(optimize_crabs(&mut crabs, geometric_median, constant_burn), 37);
    }

    #[test]
    fn example2() {
        let mut crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(optimize_crabs(&mut crabs, arithmetic_mean, linear_burn), 168);
    }

    #[test]
    fn puzzle1() {
        let mut crabs: Vec<u32> = include_str!("./input/day7")
            .split_terminator(',')
            .map(|x| x.trim().parse().unwrap())
            .collect();

            assert_eq!(optimize_crabs(&mut crabs, geometric_median, constant_burn), 359648);
    }

    #[test]
    fn puzzle2() {
        let mut crabs: Vec<u32> = include_str!("./input/day7")
            .split_terminator(',')
            .map(|x| x.trim().parse().unwrap())
            .collect();

        assert_eq!(optimize_crabs(&mut crabs, arithmetic_mean, linear_burn), 100727924);
    }
}

use std::cmp::PartialOrd;
use std::ops::Add;

/// Given a sequence of measurements, count the number of times
/// that the measurement increases from the previous measurement.
///
/// An empty slice returns 0 by convention.
///
/// # Examples
/// ```
/// use aoc2021::day1::count_measurements;
/// let sample = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
/// assert_eq!(count_measurements(&sample), 7);
/// ```
pub fn count_measurements<T>(measurements: &[T]) -> usize where T: PartialOrd {
    measurements
        .windows(2)
        .filter(|&x| x[1] > x[0])
        .count()
}

/// Given a sequence of measurements, count the number of times
/// that the sum of subsequent sliding three-element windows is
/// strictly increasing.
///
/// # Example s
/// ```
/// use aoc2021::day1::count_triples;
/// let sample = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
/// assert_eq!(count_triples(&sample), 5);
/// ```
pub fn count_triples<T>(measurements: &[T]) -> usize where T: PartialOrd + Copy + Add<Output=T> {
    let triplet_sums: Vec<T> = measurements
        .windows(3)
        .map(|a| a[0] + a[1] + a[2])
        .collect();

    count_measurements(&triplet_sums)
}

#[cfg(test)]
mod answers {
    use super::*;
    use once_cell::sync::OnceCell;

    static INPUT: OnceCell<Vec<u32>> = OnceCell::new();

    fn get_input() -> &'static Vec<u32> {
        INPUT.get_or_init(|| {
            include_str!("./input/day1")
                .split_ascii_whitespace()
                .filter_map(|line| line.parse().ok())
                .collect()
        })
    }

    #[test]
    fn puzzle1() {
        println!("Puzzle answer is {}", count_measurements(get_input()));
    }

    #[test]
    fn puzzle2() {
        println!("Puzzle answer is {}", count_triples(get_input()));
    }
}

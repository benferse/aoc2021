use std::cmp::PartialOrd;

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

#[cfg(test)]
mod answers {
    use super::*;

    #[test]
    fn puzzle1() {
        let input = include_str!("./input/day1")
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect::<Vec<_>>();

        println!("Puzzle answer is {}", count_measurements(&input));
    }
}

/// Simulate a population of lanternfish based on the lifecycle
/// rules given
/// - Newborns take 8 days before producing offspring
/// - Adults take 6 days of gestation
///
/// # Examples
///
/// ```
/// use aoc2021::day6::be_fruitful;
/// let mut population = [0, 1, 1, 2, 1, 0, 0, 0, 0];
/// for _ in 1..=80 {
///     be_fruitful(&mut population);
/// }
///
/// assert_eq!(population.iter().sum::<u64>(), 5934);
pub fn be_fruitful(population: &mut [u64; 9]) {
    population.rotate_left(1);
    population[6] += population[8];
}

#[cfg(test)]
mod answers {
    use super::*;

    #[test]
    fn puzzle1() {
        let mut population = include_str!("./input/day6")
            .split_terminator(',')
            .map(|x| { x.trim().parse::<usize>().unwrap() })
            .fold([0; 9], |mut acc, x| { acc[x] += 1; acc });

        for _ in 1..=80 {
            be_fruitful(&mut population);
        }

        assert_eq!(population.iter().sum::<u64>(), 396210);
    }

    #[test]
    fn puzzle2() {
        let mut population = include_str!("./input/day6")
            .split_terminator(',')
            .map(|x| { x.trim().parse::<usize>().unwrap() })
            .fold([0; 9], |mut acc, x| { acc[x] += 1; acc });

        for _ in 1..=256 {
            be_fruitful(&mut population);
        }

        assert_eq!(population.iter().sum::<u64>(), 1770823541496);
    }
}

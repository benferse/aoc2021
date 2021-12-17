pub struct PowerConsumption {
    registers: Vec<u32>,
    diag_count: u32,
}

impl Default for PowerConsumption {
    fn default() -> Self {
        Self::new()
    }
}

impl PowerConsumption {
    pub fn new() -> Self {
        Self {
            registers: vec![],
            diag_count: 0,
        }
    }

    pub fn add_diagnostic(&mut self, input: &str) {
        // Dynamically resize the set of collectors
        self.registers.resize(input.len(), 0);

        // Process each index in turn and count the set "bits"
        input
            .chars()
            .enumerate()
            .filter(|&(_, val)| val == '1')
            .for_each(|(idx, _)| self.registers[idx] += 1);

        self.diag_count += 1;
    }

    /// Calculate the gamma rate of the sampled diagnostics
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc2021::day3::PowerConsumption;
    /// let mut pc = PowerConsumption::new();
    /// let samples = vec!["00100", "11110", "10110", "10111", "10101", "01111",
    ///                    "00111", "11100", "10000", "11001", "00010", "01010"];
    ///
    /// for s in samples {
    ///     pc.add_diagnostic(s);
    /// }
    ///
    /// assert_eq!(pc.gamma_rate(), 22);
    /// ```
    pub fn gamma_rate(&self) -> u32 {
        let pivot = self.diag_count / 2;

        self.registers
            .iter()
            .rev()
            .enumerate()
            .filter(|&(_, val)| val > &pivot)
            .fold(0, |acc, x| acc | (1 << x.0))
    }

    /// Calculate the epsilon rate of the sampled diagnostics
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc2021::day3::PowerConsumption;
    /// let mut pc = PowerConsumption::new();
    /// let samples = vec!["00100", "11110", "10110", "10111", "10101", "01111",
    ///                    "00111", "11100", "10000", "11001", "00010", "01010"];
    ///
    /// for s in samples {
    ///     pc.add_diagnostic(s);
    /// }
    ///
    /// assert_eq!(pc.epsilon_rate(), 9);
    /// ```
    pub fn epsilon_rate(&self) -> u32 {
        // Epsilon is just the bitwise complement of gamma.
        // Simply taking the complement isn't sufficient, due to
        // extension
        !self.gamma_rate() & ((1 << self.registers.len()) - 1)
    }
}

/// Rate the life support system, producing the O2 generator rating and the
/// CO2 scrubber rating
///
/// # Examples
///
/// ```
/// use aoc2021::day3::rate_life_support;
/// let mut samples = vec!["00100", "11110", "10110", "10111", "10101", "01111",
///                    "00111", "11100", "10000", "11001", "00010", "01010"];
///
/// assert_eq!(rate_life_support(&mut samples), (23, 10));
pub fn rate_life_support(samples: &mut [&str]) -> (u32, u32) {
    // Making the assumption that all of the samples are the same length
    // and that there's at least one of em :)
    let bit_count = samples[0].len();

    // Partition the data set based on the first bit. The partition with
    // the more frequent bit is the O2 reading, and the other is the CO2
    // reading
    let (mut o2_samples, mut co2_samples) = partition_by_bit(samples, 0);

    for n in 1..bit_count {
        if o2_samples.len() > 1 {
            o2_samples = partition_by_bit(o2_samples, n).0;
        }

        if co2_samples.len() > 1 {
            co2_samples = partition_by_bit(co2_samples, n).1;
        }
    }

    // When all is said and done there should be a single sample left in each one
    let o2_rating = u32::from_str_radix(o2_samples[0], 2).unwrap_or(0);
    let co2_rating = u32::from_str_radix(co2_samples[0], 2).unwrap_or(0);
    (o2_rating, co2_rating)
}

/// Partitions the incoming samples based on the frequency of the bits in the
/// `which_bit` position. The returned tuple has the more frequent sample in the
/// first position, and the less frequent sample in the second position. If both
/// occur with equal frequency, then the first element contains the samples with
/// a 1 bit.
pub fn partition_by_bit<'a, 'b>(samples: &'a mut[&'b str], which_bit: usize) -> (&'a mut [&'b str], &'a mut [&'b str]) {
    let pivot = samples.iter_mut().partition_in_place(|x| x.chars().nth(which_bit) == Some('1'));
    let (ones, zeroes) = samples.split_at_mut(pivot);

    if ones.len() >= zeroes.len() {
        (ones, zeroes)
    } else {
        (zeroes, ones)
    }
}

#[cfg(test)]
mod answers {
    use super::*;

    #[test]
    fn puzzle1() {
        let mut pc = PowerConsumption::new();
        for line in include_str!("./input/day3").lines() {
            pc.add_diagnostic(line);
        }

        assert_eq!(pc.gamma_rate() * pc.epsilon_rate(), 2498354);
    }

    #[test]
    fn puzzle2() {
        let mut data: Vec<&str> = include_str!("./input/day3").lines().collect();
        let result = rate_life_support(&mut data);

        assert_eq!(result.0 * result.1, 3277956);
    }
}

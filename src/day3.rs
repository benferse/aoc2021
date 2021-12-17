pub struct PowerConsumption {
    registers: Vec<u32>,
    diag_count: u32,
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
    /// pc.add_diagnostic("00100");
    /// pc.add_diagnostic("11110");
    /// pc.add_diagnostic("10110");
    /// pc.add_diagnostic("10111");
    /// pc.add_diagnostic("10101");
    /// pc.add_diagnostic("01111");
    /// pc.add_diagnostic("00111");
    /// pc.add_diagnostic("11100");
    /// pc.add_diagnostic("10000");
    /// pc.add_diagnostic("11001");
    /// pc.add_diagnostic("00010");
    /// pc.add_diagnostic("01010");
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
    /// pc.add_diagnostic("00100");
    /// pc.add_diagnostic("11110");
    /// pc.add_diagnostic("10110");
    /// pc.add_diagnostic("10111");
    /// pc.add_diagnostic("10101");
    /// pc.add_diagnostic("01111");
    /// pc.add_diagnostic("00111");
    /// pc.add_diagnostic("11100");
    /// pc.add_diagnostic("10000");
    /// pc.add_diagnostic("11001");
    /// pc.add_diagnostic("00010");
    /// pc.add_diagnostic("01010");
    ///
    /// assert_eq!(pc.epsilon_rate(), 9);
    /// ```
    pub fn epsilon_rate(&self) -> u32 {
        // Simply taking the complement isn't sufficient, due to
        // extension
        !self.gamma_rate() & ((1 << self.registers.len()) - 1)
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
}

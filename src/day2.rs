use std::str::FromStr;
use eyre::{Result, WrapErr};

#[derive(Debug, PartialEq)]
pub enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = eyre::Report;

    /// Attempts to parse a `Command` from a simple text-encoded
    /// representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc2021::day2::Command;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Command::from_str("forward 1")?, Command::Forward(1));
    /// assert_eq!(Command::from_str("up 1")?, Command::Up(1));
    /// assert_eq!(Command::from_str("down 3")?, Command::Down(3));
    ///
    /// assert!(Command::from_str("backwards 3").is_err());
    /// assert!(Command::from_str("forward x").is_err());
    /// assert!(Command::from_str("stall").is_err());
    /// assert!(Command::from_str("forward 1 2").is_err());
    /// assert!(Command::from_str("forward 1.2").is_err());
    ///
    /// # Ok::<(), eyre::Report>(())
    /// ```
    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<&str> = s.split_ascii_whitespace().collect();
        eyre::ensure!(tokens.len() == 2, "Commands are strings with two whitespace separated tokens");

        let delta: u32 = tokens[1].parse().wrap_err("Command delta wasn't a positive integer")?;

        match tokens[0] {
            "forward" => Ok(Self::Forward(delta)),
            "down" => Ok(Self::Down(delta)),
            "up" => Ok(Self::Up(delta)),
            unknown => eyre::bail!("Not sure what to make of command: {}", unknown),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Position(pub u32, pub u32);

impl Position {
    /// Executes a single command against this position, and
    /// returns the resulting new position relative to this
    /// starting position
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc2021::day2::{Command, Position};
    /// let p = Position(0, 0)
    ///   .execute(Command::Forward(5))
    ///   .execute(Command::Down(5))
    ///   .execute(Command::Forward(8))
    ///   .execute(Command::Up(3))
    ///   .execute(Command::Down(8))
    ///   .execute(Command::Forward(2));
    /// assert_eq!(p, Position(15, 10));
    ///
    /// // Don't let our submarine fly, because that's weird
    /// assert_eq!(Position(0,0).execute(Command::Up(1)), Position(0,0));
    /// ```
    pub fn execute(self, cmd: Command) -> Self {
        match cmd {
            Command::Forward(x) => Self(self.0 + x, self.1),
            Command::Down(x) => Self(self.0, self.1 + x),
            Command::Up(x) => Self(self.0, self.1.saturating_sub(x)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Bearing {
    pub position: Position,
    pub aim: u32,
}

impl Default for Bearing {
    fn default() -> Self {
        Self {
            position: Position(0, 0),
            aim: 0,
        }
    }
}

impl Bearing {
    /// Execute a single command against this bearing, and return
    /// the resulting new bearing relative to this starting state.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc2021::day2::{Bearing, Command, Position};
    /// let b = Bearing::default()
    ///   .execute(Command::Forward(5))
    ///   .execute(Command::Down(5))
    ///   .execute(Command::Forward(8))
    ///   .execute(Command::Up(3))
    ///   .execute(Command::Down(8))
    ///   .execute(Command::Forward(2));
    /// assert_eq!(b.position, Position(15, 60));
    ///
    /// ```
    pub fn execute(self, cmd: Command) -> Self {
        match cmd {
            Command::Down(x) => Self { aim: self.aim + x, ..self },
            Command::Up(x) => Self { aim: self.aim.saturating_sub(x), ..self },
            Command::Forward(x) => {
                let position = Position(self.position.0 + x, self.position.1 + (x * self.aim));
                Self { position, ..self }
            }
        }
    }
}

#[cfg(test)]
mod answers {
    use super::*;

    #[test]
    fn puzzle1() {
        let position = include_str!("./input/day2")
            .lines()
            .fold(Position(0,0), |acc, x| acc.execute(x.parse().unwrap()));

        assert_eq!(position.0 * position.1, 1250395);
    }

    #[test]
    fn puzzle2() {
        let Bearing { position, .. } = include_str!("./input/day2")
            .lines()
            .fold(Bearing::default(), |acc, x| acc.execute(x.parse().unwrap()));

        assert_eq!(position.0 * position.1, 1451210346);
    }
}

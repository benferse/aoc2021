use std::collections::HashMap;

pub struct Coords {
    pub row: usize,
    pub col: usize,
}

impl Coords {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Default)]
pub struct BingoCard {
    spots: HashMap<u32, Coords>,
    row_counts: [u32; 5],
    col_counts: [u32; 5],
    still_playing: bool,
}

impl BingoCard {
    pub fn from_lines(input: &[&str]) -> Self {
        let mut card = Self {
            still_playing: true,
            ..Default::default()
        };

        for (n, line) in input.iter().enumerate().take(5) {
            line
                .split_whitespace()
                .enumerate()
                .for_each(|(idx, val)| card.add_spot(val.parse().unwrap_or(0), Coords::new(n, idx)));
        }

        card
    }

    pub fn add_spot(&mut self, number: u32, loc: Coords) {
        self.row_counts[loc.row] += 1;
        self.col_counts[loc.col] += 1;
        self.spots.insert(number, loc);
    }

    pub fn mark_spot(&mut self, number: u32) -> bool {
        if let Some(loc) = self.spots.remove(&number) {
            self.row_counts[loc.row] -= 1;
            self.col_counts[loc.col] -= 1;

            self.row_counts[loc.row] == 0 || self.col_counts[loc.col] == 0
        } else {
            false
        }
    }

    pub fn declare_victory(&mut self) -> u32 {
        self.still_playing = false;
        self.spots.keys().sum()
    }
}

/// Play a round of bingo, returning the scores of the cards that won (in order)
///
/// # Examples
///
/// ```
/// use aoc2021::day4::*;
/// let numbers = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1];
///
/// let card1 = BingoCard::from_lines(&["22 13 17 11 0", "8 2 23 4 24", "21 9 14 16 7", "6 10 3 18 5", "1 12 20 15 19"]);
/// let card2 = BingoCard::from_lines(&["3 15 0 2 22", "9 18 13 17 5", "19 8 7 25 23", "20 11 10 24 4", "14 21 16 12 6"]);
/// let card3 = BingoCard::from_lines(&["14 21 17 24 4", "10 16 15 9 19", "18 8 23 26 20", "22 11 13 6 5", "2 0 12 3 7"]);
///
/// let results = play_bingo(vec![card1, card2, card3], numbers);
/// assert_eq!(results.first(), Some(&4512));
/// assert_eq!(results.last(), Some(&1924));
/// ```
pub fn play_bingo(mut cards: Vec<BingoCard>, numbers: Vec<u32>) -> Vec<u32> {
    let mut winning_scores = vec![];

    for number in numbers {
        for card in cards.iter_mut().filter(|x| x.still_playing) {
            if card.mark_spot(number) {
                winning_scores.push(card.declare_victory() * number);
            }
        }
    }

    winning_scores
}

#[cfg(test)]
mod answers {
    use super::*;

    #[test]
    fn puzzle1_and_puzzle2() {
        // Parse the input file - the first line is the list of numbers called,
        // then some number of bingo cards, all separated by whitespace
        let mut input: Vec<&str> = include_str!("./input/day4")
            .split_terminator('\n')
            .filter(|x| !x.trim().is_empty())
            .collect();

        let numbers: Vec<u32> = input.remove(0)
            .split_terminator(',')
            .map(|x| x.parse().unwrap_or(0))
            .collect();

        let mut cards: Vec<BingoCard> = vec![];

        for chunk in input.chunks_exact(5) {
            cards.push(BingoCard::from_lines(chunk));
        }

        let results = play_bingo(cards, numbers);

        assert_eq!(results.first(), Some(&82440));
        assert_eq!(results.last(), Some(&20774));
    }
}

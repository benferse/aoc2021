use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Line {
    start: (u32, u32),
    end: (u32, u32),
}

impl Line {
    pub fn new(start: (u32, u32), end: (u32, u32)) -> Self {
        Self { start, end }
    }

    pub fn points(self) -> impl Iterator<Item=(u32, u32)> {
        let x_iter = Self::range_iterator(self.start.0, self.end.0);
        let y_iter = Self::range_iterator(self.start.1, self.end.1);

        x_iter.zip(y_iter)
    }

    pub fn range_iterator(a: u32, b: u32) -> Box<dyn Iterator<Item=u32>> {
        match a.cmp(&b) {
            Ordering::Equal => Box::new((a..=b).cycle()),
            Ordering::Less => Box::new(a..=b),
            Ordering::Greater => Box::new((b..=a).rev()),
        }
    }

    pub fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn str_to_point(s: &str) -> (u32, u32) {
        let mut tokens = s.split_terminator(',');
        (tokens.next().unwrap_or("0").parse().unwrap_or(0), tokens.next().unwrap_or("0").parse().unwrap_or(0))
    }
}

impl From<&str> for Line {
    fn from(input: &str) -> Self {
        let tokens = input.split_whitespace().collect::<Vec<_>>();
        Self::new(Self::str_to_point(tokens[0]), Self::str_to_point(tokens[2]))
    }

}

#[derive(Default)]
pub struct OceanFloor {
    vent_map: HashMap<(u32, u32), u32>
}

impl OceanFloor {
    pub fn add_vent(&mut self, line: Line) {
        for point in line.points() {
            *self.vent_map.entry(point).or_insert(0) += 1;
        }
    }

/// Count the number of points on the grid where at least
/// two lines overlap
///
/// # Examples
///
/// ```
/// use aoc2021::day5::*;
/// let mut floor = OceanFloor::default();
/// floor.add_vent(Line::from("0,9 -> 5,9"));
/// floor.add_vent(Line::from("8,0 -> 0,8"));
/// floor.add_vent(Line::from("9,4 -> 3,4"));
/// floor.add_vent(Line::from("2,2 -> 2,1"));
/// floor.add_vent(Line::from("7,0 -> 7,4"));
/// floor.add_vent(Line::from("6,4 -> 2,0"));
/// floor.add_vent(Line::from("0,9 -> 2,9"));
/// floor.add_vent(Line::from("3,4 -> 1,4"));
/// floor.add_vent(Line::from("0,0 -> 8,8"));
/// floor.add_vent(Line::from("5,5 -> 8,2"));
///
/// assert_eq!(floor.count_overlap(), 12);
    pub fn count_overlap(&self) -> usize {
        self.vent_map
            .values()
            .filter(|&x| *x > 1)
            .count()
    }
}

#[cfg(test)]
mod answers {
    use super::*;

    #[test]
    fn puzzle1() {
        let mut ocean = OceanFloor::default();

        for line in include_str!("./input/day5").lines() {
            let line = Line::from(line);
            if line.is_vertical() || line.is_horizontal() {
                ocean.add_vent(line);
            }
        }

        assert_eq!(ocean.count_overlap(), 6007);
    }

    #[test]
    fn puzzle2() {
        let mut ocean = OceanFloor::default();

        for line in include_str!("./input/day5").lines() {
            let line = Line::from(line);
            ocean.add_vent(line);
        }

        assert_eq!(ocean.count_overlap(), 19349);
    }
}

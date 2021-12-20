use std::collections::{HashMap, HashSet};

pub struct Display {
    digit_table: HashMap<String, usize>,
    outputs: [String; 4],
}

impl Display {
    pub fn parse_input(input: &str) -> Self {
        let (left, right) = input.split_once('|').unwrap();

        let mut solved: [String; 10] = Default::default();
        let mut outputs: [String; 4] = Default::default();
        let mut unsolved_6 = vec![];
        let mut unsolved_5 = vec![];

        for (_, val) in left.split_whitespace().enumerate().take(10) {
            let val = Self::sorted(val);
            match val.len() {
                2 => solved[1] = val,
                3 => solved[7] = val,
                4 => solved[4] = val,
                5 => { unsolved_5.push(val); }
                6 => { unsolved_6.push(val); }
                7 => solved[8] = val,
                _ => unreachable!(),
            }
        }

        // The unsolved digits fall into two groups - those with five
        // segments, and those with six. We can use the four unique ones
        // above and some simple rules to deduce which is which.
        for digit in unsolved_6 {
            if Self::has_segments_from(&digit, &solved[1]) {
                if Self::has_segments_from(&digit, &solved[4]) {
                    solved[9] = digit;
                } else {
                    solved[0] = digit;
                }
            } else {
                solved[6] = digit;
            }
        }

        for digit in unsolved_5 {
            if Self::has_segments_from(&digit, &solved[1]) {
                solved[3] = digit;
            } else if Self::has_segments_from(&solved[6], &digit) {
                solved[5] = digit;
            } else {
                solved[2] = digit;
            }
        }

        for (idx, val) in right.split_whitespace().enumerate().take(4) {
            outputs[idx] = val.chars().collect();
        }

        let mut digit_table = HashMap::new();
        for (idx, val) in solved.iter().enumerate() {
            println!("{}: {}", idx, val);
            digit_table.insert(val.to_owned(), idx);
        }

        Self { digit_table, outputs, }
    }

    pub fn sorted(input: &str) -> String {
        let mut v: Vec<char> = input.chars().collect();
        v.sort_unstable();
        v.iter().collect()
    }

    pub fn has_segments_from(haystack: &str, needle: &str) -> bool {
        let haystack: HashSet<char> = haystack.chars().collect();
        let needle: HashSet<char> = needle.chars().collect();
        needle.is_subset(&haystack)
    }

    pub fn count_unique_digits(&self) -> usize {
        self.outputs.iter().filter(|&x| [2, 3, 4, 7].contains(&x.len())).count()
    }

    pub fn translate(&self) -> usize {
        let mut value = 0usize;

        for (idx, output) in self.outputs.iter().rev().enumerate() {
            let key = Self::sorted(output);
            if let Some(v) = self.digit_table.get(&key) {
                value += v * 10usize.pow(idx as u32);
            }
        }

        value
    }
}

pub fn load_displays(input: &str) -> Vec<Display> {
    input.lines().map(Display::parse_input).collect()
}

#[cfg(test)]
mod answers {
    use super::*;

    static SAMPLE: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn example1() {
        let displays = load_displays(SAMPLE);
        let result = displays.iter().fold(0, |acc, x| acc + x.count_unique_digits());

        assert_eq!(result, 26);
    }

    #[test]
    fn puzzle1() {
        let displays = load_displays(include_str!("./input/day8"));
        let result = displays.iter().fold(0, |acc, x| acc + x.count_unique_digits());

        assert_eq!(result, 278);
    }

    #[test]
    fn example2() {
        let displays = load_displays(SAMPLE);
        let result = displays.iter().fold(0, |acc, x| acc + x.translate());

        assert_eq!(result, 61229);
    }

    #[test]
    fn puzzle2() {
        let displays = load_displays(include_str!("./input/day8"));
        let result = displays.iter().fold(0, |acc, x| acc + x.translate());

        assert_eq!(result, 986179);
    }

}

use std::collections::{BTreeMap, HashMap};

struct Word {
    chars: Vec<char>,
}

impl Word {
    fn value(&self, char_values: &BTreeMap<char, u8>) -> u32 {
        self.chars
            .iter()
            .fold(0, |sum, ch| sum * 10 + char_values[ch] as u32)
    }
}

impl<'a> From<&'a str> for Word {
    fn from(string: &'a str) -> Word {
        Word {
            chars: string.chars().filter(|&ch| ch.is_uppercase()).collect(),
        }
    }
}

struct Puzzle {
    chars: Vec<char>,
    char_values: BTreeMap<char, u8>,
    lhs: Vec<Word>,
    rhs: Vec<Word>,
}

impl Puzzle {
    fn lhs_value(&self) -> u32 {
        self.lhs
            .iter()
            .map(|word| word.value(&self.char_values))
            .sum()
    }

    fn rhs_value(&self) -> u32 {
        self.rhs
            .iter()
            .map(|word| word.value(&self.char_values))
            .sum()
    }

    fn is_first_char(&self, ch: char) -> bool {
        self.lhs
            .iter()
            .chain(self.rhs.iter())
            .any(|word| word.chars[0] == ch)
    }

    fn solve(&mut self, from: usize) -> bool {
        if from >= self.chars.len() {
            return false;
        }

        let ch = self.chars[from];
        let lowest = if self.is_first_char(ch) { 1 } else { 0 };

        for digit in lowest..=9 {
            self.char_values.insert(ch, digit);

            if from > 0 && self
                .chars
                .iter()
                .take(from)
                .any(|c| self.char_values[c] == digit)
            {
                continue;
            }

            if from == self.chars.len() - 1 {
                if self.lhs_value() == self.rhs_value() {
                    return true;
                }
            }

            if self.solve(from + 1) {
                return true;
            }
        }

        false
    }
}

impl<'a> From<&'a str> for Puzzle {
    fn from(string: &'a str) -> Puzzle {
        let mut lhs = Vec::new();
        let mut rhs = Vec::new();

        let mut sides = string.split("==");

        if let Some(left) = sides.next() {
            lhs = left.split('+').map(Word::from).collect();
        }

        if let Some(right) = sides.next() {
            rhs = right.split('+').map(Word::from).collect();
        }

        let mut chars = Vec::new();

        for &ch in rhs
            .iter()
            .flat_map(|word| word.chars.iter())
            .chain(lhs.iter().flat_map(|word| word.chars.iter()))
        {
            if !chars.contains(&ch) {
                chars.push(ch);
            }
        }

        Puzzle {
            chars,
            char_values: BTreeMap::new(),
            lhs,
            rhs,
        }
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut puzzle = Puzzle::from(input);

    if puzzle.solve(0) {
        Some(puzzle.char_values.iter().map(|(&k, &v)| (k, v)).collect())
    } else {
        None
    }
}

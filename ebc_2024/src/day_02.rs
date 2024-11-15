#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Default)]
pub struct Day02 {
    words: Vec<String>,
    lines: Vec<String>,
}

impl Day02 {
    pub fn new() -> Self {
        Self::default()
    }

    fn left_right(&self, lines: &[String], wrapping: bool) -> Vec<(usize, usize)> {
        let reverse_words = self
            .words
            .iter()
            .map(|w| w.chars().rev().collect::<String>())
            .collect::<Vec<_>>();

        fn matches(s: &str, w: &str, idx: usize, wrapping: bool) -> bool {
            if s.len() < w.len() {
                false
            } else if !wrapping || s.len() >= w.len() + idx {
                s[idx..].starts_with(w)
            } else {
                let tail = &s[idx..];
                let head = &s[..w.len() - (s.len() - idx)];
                w.starts_with(tail) && w.ends_with(head)
            }
        }

        lines
            .iter()
            .enumerate()
            .flat_map(|(y, s)| {
                (0..s.len())
                    .flat_map(|x| {
                        self.words
                            .iter()
                            .zip(reverse_words.iter())
                            .filter_map(|(w, rw)| {
                                if matches(s, w, x, wrapping) || matches(s, rw, x, wrapping) {
                                    Some(x..x + w.len())
                                } else {
                                    None
                                }
                            })
                            .flatten()
                            .map(|x| (x % s.len(), y))
                            .collect::<HashSet<(usize, usize)>>()
                    })
                    .collect::<HashSet<(usize, usize)>>()
            })
            .collect()
    }
}

impl helper::Runner for Day02 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.words.extend(
            lines[0]
                .strip_prefix("WORDS:")
                .unwrap()
                .split(',')
                .map(|s| s.to_string()),
        );
        for line in lines[2..].iter() {
            self.lines.push(line.into());
        }

        Ok(())
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .lines
            .iter()
            .map(|s| {
                (0..s.len())
                    .filter(|i| self.words.iter().any(|w| s[*i..].starts_with(w)))
                    .count()
            })
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.left_right(&self.lines, false).len().into())
    }
}

impl helper::EbcRunner for Day02 {
    fn part3(&mut self) -> Result<helper::RunOutput, Error> {
        let line_chars: Vec<Vec<char>> = self.lines.iter().map(|s| s.chars().collect()).collect();
        let rotated_lines: Vec<String> = (0..line_chars[0].len())
            .map(|i| line_chars.iter().map(|l| l[i]).collect::<String>())
            .collect();

        let mut runes = HashSet::default();
        runes.extend(self.left_right(&self.lines, true).iter().copied());
        runes.extend(
            self.left_right(&rotated_lines, false)
                .iter()
                .copied()
                .map(|(y, x)| (x, y)),
        );
        Ok(runes.len().into())
    }
}

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

    fn left_right(&self, lines: &[String], wrapping: bool) -> HashSet<(usize, usize)> {
        let reverse_words = self
            .words
            .iter()
            .map(|w| w.chars().rev().collect::<String>())
            .collect::<Vec<_>>();

        let mut points: HashSet<(usize, usize)> = HashSet::default();
        fn add_matches(
            s: &str,
            w: &str,
            x: usize,
            y: usize,
            wrapping: bool,
            points: &mut HashSet<(usize, usize)>,
        ) {
            if s.len() < w.len() {
                return;
            }
            if !wrapping || s.len() >= w.len() + x {
                if s[x..].starts_with(w) {
                    points.extend((x..x + w.len()).map(|x| (x, y)))
                }
            } else {
                let tail = &s[x..];
                let head = &s[..w.len() - (s.len() - x)];
                if w.starts_with(tail) && w.ends_with(head) {
                    points.extend((0..head.len()).map(|x| (x, y)));
                    points.extend((x..s.len()).map(|x| (x, y)));
                }
            }
        }

        for (y, s) in lines.iter().enumerate() {
            (0..s.len()).for_each(|x| {
                self.words
                    .iter()
                    .zip(reverse_words.iter())
                    .for_each(|(w, rw)| {
                        add_matches(s, w, x, y, wrapping, &mut points);
                        add_matches(s, rw, x, y, wrapping, &mut points);
                    });
            });
        }

        points
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

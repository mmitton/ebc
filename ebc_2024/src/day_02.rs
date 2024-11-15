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

    fn scan_runes(&self, cylinder: bool) -> usize {
        let mut found_runes: HashSet<(usize, usize)> = HashSet::default();
        let runes: Vec<Vec<char>> = self.lines.iter().map(|l| l.chars().collect()).collect();
        let words: Vec<Vec<char>> = self.words.iter().map(|l| l.chars().collect()).collect();

        let h = runes.len();
        for y in 0..h {
            let w = runes[y].len();
            for x in 0..w {
                for word in words.iter() {
                    if word[0] == runes[y][x] {
                        let mut dirs = [
                            Some((x, y, 1, 0, vec![(x, y)])),
                            Some((x, y, -1, 0, vec![(x, y)])),
                            if cylinder {
                                Some((x, y, 0, 1, vec![(x, y)]))
                            } else {
                                None
                            },
                            if cylinder {
                                Some((x, y, 0, -1, vec![(x, y)]))
                            } else {
                                None
                            },
                        ];

                        for c in word.iter().skip(1).copied() {
                            let mut active = false;
                            for opt_dir in dirs.iter_mut() {
                                if let Some(dir) = opt_dir {
                                    let mut x = dir.0 as isize + dir.2;
                                    let y = dir.1 as isize + dir.3;

                                    if y < 0 || y >= h as isize {
                                        *opt_dir = None;
                                        continue;
                                    }
                                    if x < 0 || x >= w as isize {
                                        if cylinder {
                                            x = x.rem_euclid(w as isize);
                                        } else {
                                            *opt_dir = None;
                                            continue;
                                        }
                                    }

                                    dir.0 = x as usize;
                                    dir.1 = y as usize;
                                    if runes[dir.1][dir.0] != c {
                                        *opt_dir = None;
                                        continue;
                                    }

                                    dir.4.push((dir.0, dir.1));
                                    active = true;
                                }
                            }
                            if !active {
                                break;
                            }
                        }

                        // Any left are good!
                        found_runes.extend(dirs.into_iter().flatten().flat_map(|dir| dir.4));
                    }
                }
            }
        }

        found_runes.len()
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
        Ok(self.scan_runes(false).into())
    }

    fn part3(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.scan_runes(true).into())
    }
}

impl helper::Runner for Day02 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
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

    fn run_part(&mut self, part: u8) -> Result<helper::RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            3 => self.part3(),
            _ => Err(Error::Skipped),
        }
    }
}

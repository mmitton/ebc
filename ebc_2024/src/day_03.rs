#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Default)]
pub struct Day03 {
    grid: HashMap<(isize, isize), usize>,
}

impl Day03 {
    pub fn new() -> Self {
        Self::default()
    }

    fn dig(&mut self, diag: bool) -> usize {
        let mut removed = self.grid.len();
        let mut coords: Vec<(isize, isize)> = self.grid.keys().copied().collect();
        let mut next_coords = Vec::new();
        while !coords.is_empty() {
            'dig: for (x, y) in coords.drain(..) {
                let depth = self.grid.get(&(x, y)).unwrap();
                for neighbor in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                    if let Some(d) = self.grid.get(&neighbor) {
                        if d != depth && *d != depth + 1 {
                            continue 'dig;
                        }
                    } else {
                        continue 'dig;
                    }
                }
                if diag {
                    for neighbor in [
                        (x - 1, y - 1),
                        (x + 1, y - 1),
                        (x - 1, y + 1),
                        (x + 1, y + 1),
                    ] {
                        if let Some(d) = self.grid.get(&neighbor) {
                            if d != depth && *d != depth + 1 {
                                continue 'dig;
                            }
                        } else {
                            continue 'dig;
                        }
                    }
                }

                self.grid.insert((x, y), depth + 1);
                next_coords.push((x, y));
                removed += 1;
            }
            std::mem::swap(&mut coords, &mut next_coords);
        }
        removed
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.dig(false).into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.dig(false).into())
    }

    fn part3(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.dig(true).into())
    }
}

impl helper::Runner for Day03 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    self.grid.insert((x as isize, y as isize), 1);
                }
            }
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

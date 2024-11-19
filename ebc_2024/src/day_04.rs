#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Default)]
pub struct Day04 {
    nails: Vec<isize>,
}

impl Day04 {
    pub fn new() -> Self {
        Self::default()
    }

    fn level_to(&self, level: isize) -> usize {
        self.nails
            .iter()
            .map(|nail| (nail - level).unsigned_abs())
            .sum()
    }

    fn level(&self) -> usize {
        let min = self.nails.iter().copied().min().unwrap();
        self.level_to(min)
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.level().into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.level().into())
    }

    fn part3(&mut self) -> Result<helper::RunOutput, Error> {
        let min = self.nails.iter().copied().min().unwrap();
        let max = self.nails.iter().copied().max().unwrap();
        let mut level_to = (max + min) / 2;
        let mut level_to_strikes = self.level_to(level_to);
        loop {
            let lower = self.level_to(level_to - 1);
            let higher = self.level_to(level_to + 1);

            if lower < level_to_strikes {
                level_to -= 1;
                level_to_strikes = lower;
                continue;
            }
            if higher < level_to_strikes {
                level_to += 1;
                level_to_strikes = higher;
                continue;
            }
            break;
        }
        Ok(level_to_strikes.into())
    }
}

impl helper::Runner for Day04 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.nails.push(line.parse()?);
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

#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Default)]
pub struct Day11 {}

impl Day11 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl helper::Runner for Day11 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let _lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        Ok(())
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Err(Error::Unsolved)
    }
}

impl helper::EbcRunner for Day11 {
    fn part3(&mut self) -> Result<helper::RunOutput, Error> {
        Err(Error::Unsolved)
    }
}

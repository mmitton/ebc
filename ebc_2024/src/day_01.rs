#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Default)]
pub struct Day01 {
    battles: Vec<char>,
}

impl Day01 {
    pub fn new() -> Self {
        Self::default()
    }

    fn potions_needed(a: &[char]) -> usize {
        fn potions_needed(ch: &char, extra: usize) -> usize {
            match ch {
                'B' => 1 + extra,
                'C' => 3 + extra,
                'D' => 5 + extra,
                'x' => 0,
                _ => extra,
            }
        }
        let xs = a.iter().filter(|c| **c == 'x').count();
        let extra = match (a.len(), xs) {
            (3, 0) => 2,
            (3, 1) | (2, 0) => 1,
            _ => 0,
        };
        a.iter().map(|ch| potions_needed(ch, extra)).sum()
    }
}

impl helper::Runner for Day01 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let line = Lines::from_bufread(file, LinesOpt::RAW)?.single_line()?;
        self.battles.extend(line.chars());

        Ok(())
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .battles
            .chunks(1)
            .map(Self::potions_needed)
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .battles
            .chunks(2)
            .map(Self::potions_needed)
            .sum::<usize>()
            .into())
    }
}

impl helper::EbcRunner for Day01 {
    fn part3(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .battles
            .chunks(3)
            .map(Self::potions_needed)
            .sum::<usize>()
            .into())
    }
}

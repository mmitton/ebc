#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};
use std::collections::VecDeque;

#[derive(Default)]
pub struct Day05 {
    columns: Vec<VecDeque<usize>>,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }

    fn round(&mut self, round: usize) {
        let round = round - 1;
        let from = round % self.columns.len();
        let to = (round + 1) % self.columns.len();

        let clapper = self.columns[from].pop_front().unwrap();
        let cycle = self.columns[to].len() * 2;
        let pos = (clapper - 1) % cycle;

        if pos > cycle / 2 {
            // right side, insert after pos
            let pos = cycle - (pos + 1);
            if pos == self.columns[to].len() {
                self.columns[to].push_back(clapper);
            } else {
                self.columns[to].insert(pos + 1, clapper);
            }
        } else {
            // left side, insert before pos
            self.columns[to].insert(pos, clapper);
        }
    }

    fn shout(&self) -> usize {
        self.columns.iter().fold(0, |shout, col| {
            shout * 10usize.pow((10 * col[0]).ilog(10)) + col[0]
        })
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        for round in 1..=10 {
            self.round(round);
        }
        Ok(self.shout().into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut seen: HashMap<usize, usize> = HashMap::default();
        seen.insert(self.shout(), 1);
        for round in 1.. {
            self.round(round);
            let shout = self.shout();
            let seen = seen.entry(shout).or_default();
            *seen += 1;
            if *seen == 2024 {
                return Ok((round * shout).into());
            }
        }
        Err(Error::Unsolved)
    }

    fn part3(&mut self) -> Result<helper::RunOutput, Error> {
        let mut seen: HashMap<usize, usize> = HashMap::default();
        seen.insert(self.shout(), 1);
        for round in 1..5000 {
            self.round(round);
            let shout = self.shout();
            let seen = seen.entry(shout).or_default();
            *seen += 1;
            if *seen == 2024 {
                return Ok((round * shout).into());
            }
        }
        Ok(seen.keys().copied().max().unwrap().into())
    }
}

impl helper::Runner for Day05 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let numbers = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<usize>>();
            self.columns.resize(numbers.len(), VecDeque::new());
            for (i, n) in numbers.iter().enumerate() {
                self.columns[i].push_back(*n);
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

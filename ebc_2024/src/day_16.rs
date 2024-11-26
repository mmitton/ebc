#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Default)]
pub struct Day16 {
    spins: Vec<usize>,
    wheels: Vec<Vec<String>>,
    wheel_maps: Vec<Vec<Vec<u8>>>,
}

impl Day16 {
    pub fn new() -> Self {
        Self::default()
    }

    fn sequence(&self, offset: &[usize]) -> String {
        let mut ans = String::new();
        for (idx, wheel) in self.wheels.iter().enumerate() {
            if idx != 0 {
                ans.push(' ');
            }
            ans.push_str(wheel[offset[idx] % wheel.len()].as_str());
        }
        ans
    }

    fn spin(&self, offset: &mut [usize]) {
        for (idx, spin) in self.spins.iter().enumerate() {
            offset[idx] = (offset[idx] + spin) % self.wheels[idx].len();
        }
    }

    fn score(&self, offset: &[usize]) -> usize {
        let mut num = [0usize; 256];
        let mut min = u8::MAX;
        let mut max = 0;
        for (idx, wheel_maps) in self.wheel_maps.iter().enumerate() {
            let wheel_map = &wheel_maps[offset[idx]];
            for n in wheel_map.iter().step_by(2).copied() {
                min = min.min(n);
                max = max.max(n);
                num[n as usize] += 1;
            }
        }

        let take = (max - min) as usize + 1;
        let min = min as usize;
        num.iter()
            .skip(min)
            .take(take)
            .copied()
            .map(|n| n.saturating_sub(2))
            .sum()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        let mut offset = vec![0; self.spins.len()];
        for _ in 0..100 {
            self.spin(&mut offset);
        }

        Ok(self.sequence(&offset).into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut score = 0;
        let mut seen = HashMap::default();
        let mut offset = vec![0; self.spins.len()];
        const ITERS: usize = 202420242024;
        seen.insert(offset.clone(), (0, 0));
        let mut spin = 0;
        let mut found_cycle = false;
        while spin < ITERS {
            self.spin(&mut offset);
            score += self.score(&offset);
            if !found_cycle {
                if let Some((last_spin, last_score)) = seen.insert(offset.clone(), (spin, score)) {
                    let cycle_len = spin - last_spin + 1;
                    let left = ITERS - spin;
                    let cycles = left / cycle_len;
                    found_cycle = true;

                    score += (score - last_score) * cycles;
                    spin += cycle_len * cycles;
                }
            }
            spin += 1;
        }

        Ok(score.into())
    }

    fn part3(&mut self) -> Result<helper::RunOutput, Error> {
        #[derive(Hash, PartialEq, Eq)]
        struct Work {
            score: usize,
            offset: Vec<usize>,
        }

        let mut work_hash = HashSet::default();
        work_hash.insert(Work {
            score: 0,
            offset: vec![0; self.spins.len()],
        });

        let mut next_hash = HashSet::default();

        const ITERS: usize = 256;
        for _ in 0..ITERS {
            for Work { score, offset } in work_hash.iter() {
                macro_rules! process {
                    ($delta:expr) => {{
                        let mut offset = offset.clone();
                        for i in 0..self.wheels.len() {
                            offset[i] = (offset[i] as isize + $delta)
                                .rem_euclid(self.wheels[i].len() as isize)
                                as usize;
                        }

                        self.spin(&mut offset);
                        let score = score + self.score(&offset);
                        next_hash.insert(Work { score, offset });
                    }};
                }

                process!(-1);
                process!(0);
                process!(1);
            }

            std::mem::swap(&mut work_hash, &mut next_hash);
            next_hash.clear();
        }

        let mut min = usize::MAX;
        let mut max = usize::MIN;
        for Work { score, .. } in work_hash.iter() {
            min = min.min(*score);
            max = max.max(*score);
        }

        Ok(format!("{max} {min}").into())
    }
}

impl helper::Runner for Day16 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for spin in lines[0].split(',') {
            self.spins.push(spin.parse()?);
        }
        let wheel_cnt = (lines[2].len() + 1) / 4;
        self.wheels.extend((0..wheel_cnt).map(|_| Vec::new()));
        self.wheel_maps.extend((0..wheel_cnt).map(|_| Vec::new()));
        for line in lines[2..].iter() {
            for i in 0..wheel_cnt {
                let idx = i * 4;
                if idx >= line.len() {
                    break;
                }
                let wheel = line[idx..idx + 3].trim();
                let wheel_map = wheel.chars().map(|c| c as u8).collect::<Vec<u8>>();
                if !wheel.is_empty() {
                    self.wheels[i].push(wheel.into());
                    self.wheel_maps[i].push(wheel_map);
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

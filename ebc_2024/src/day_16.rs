#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Debug)]
struct Wheel {
    spin: u8,
    slots: Vec<String>,
    map: Vec<[u8; 2]>,
}

impl Wheel {
    fn new(spin: u8) -> Self {
        Self {
            spin,
            slots: Vec::new(),
            map: Vec::new(),
        }
    }

    fn get(&self, idx: u8) -> (&str, &[u8; 2]) {
        (&self.slots[idx as usize], &self.map[idx as usize])
    }

    fn add_slot(&mut self, slot: &str, map: &mut HashMap<char, u8>) {
        let slot = slot.trim();
        if slot.is_empty() {
            return;
        }
        self.slots.push(slot.into());
        let mut chars = slot.chars();
        let first = chars.next().unwrap();
        let last = chars.last().unwrap();

        let first = if let Some(first) = map.get(&first) {
            *first
        } else {
            let next = map.len() as u8;
            map.insert(first, next);
            next
        };
        let last = if let Some(last) = map.get(&last) {
            *last
        } else {
            let next = map.len() as u8;
            map.insert(last, next);
            next
        };

        self.map.push([first, last]);
    }
}

#[derive(Default)]
pub struct Day16 {
    wheels: Vec<Wheel>,
}

impl Day16 {
    pub fn new() -> Self {
        Self::default()
    }

    fn sequence(&self, offset: &[u8; 12]) -> String {
        let mut ans = String::new();
        for (idx, (wheel, offset)) in self.wheels.iter().zip(offset.iter()).enumerate() {
            if idx != 0 {
                ans.push(' ');
            }
            let (slot, _) = wheel.get(*offset);
            ans.push_str(slot);
        }
        ans
    }

    fn score(&self, offset: &[u8; 12]) -> usize {
        let mut num = [0u8; 64];
        for (wheel, offset) in self.wheels.iter().zip(offset.iter()) {
            let (_, map) = wheel.get(*offset);
            for n in map.iter().copied() {
                num[n as usize] += 1;
            }
        }

        num.iter()
            .copied()
            .map(|n| n.saturating_sub(2) as usize)
            .sum()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        let mut offset = [0u8; 12];
        for _ in 0..100 {
            offset
                .iter_mut()
                .zip(self.wheels.iter())
                .for_each(|(offset, wheel)| {
                    *offset = (*offset + wheel.spin).rem_euclid(wheel.slots.len() as u8)
                });
        }

        Ok(self.sequence(&offset).into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut score = 0;
        let mut seen = HashMap::default();
        let mut offset = [0u8; 12];
        const ITERS: usize = 202420242024;
        seen.insert(offset, (0, 0));
        let mut spin = 0;
        let mut found_cycle = false;
        while spin < ITERS {
            offset
                .iter_mut()
                .zip(self.wheels.iter())
                .for_each(|(offset, wheel)| {
                    *offset = (*offset + wheel.spin).rem_euclid(wheel.slots.len() as u8)
                });
            score += self.score(&offset);
            if !found_cycle {
                if let Some((last_spin, last_score)) = seen.insert(offset, (spin, score)) {
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
            offset: [u8; 12],
        }

        let mut work_hash = HashSet::default();
        work_hash.insert(Work {
            score: 0,
            offset: [0u8; 12],
        });

        let mut next_hash = HashSet::default();

        const ITERS: usize = 256;
        for _ in 0..ITERS {
            for Work { score, offset } in work_hash.drain() {
                macro_rules! process {
                    ($delta:expr) => {{
                        let mut offset = offset.clone();

                        offset
                            .iter_mut()
                            .zip(self.wheels.iter())
                            .for_each(|(offset, wheel)| {
                                *offset = (*offset as i8 + $delta + wheel.spin as i8)
                                    .rem_euclid(wheel.slots.len() as i8)
                                    as u8
                            });
                        let score = score + self.score(&offset);
                        next_hash.insert(Work { score, offset });
                    }};
                }

                process!(-1);
                process!(0);
                process!(1);
            }

            std::mem::swap(&mut work_hash, &mut next_hash);
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
            self.wheels.push(Wheel::new(spin.parse()?));
        }

        let mut map = HashMap::default();
        for line in lines[2..].iter() {
            for (i, wheel) in self.wheels.iter_mut().enumerate() {
                let idx = i * 4;
                if idx >= line.len() {
                    break;
                }
                wheel.add_slot(&line[idx..idx + 3], &mut map);
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

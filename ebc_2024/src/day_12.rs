#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Point};
use std::{cmp::Ordering, collections::VecDeque};

#[derive(Debug)]
struct Catapult {
    id: usize,
    point: Point<isize>,
    hits: HashMap<Point<isize>, (usize, usize)>,
}

impl Catapult {
    fn new(c: char, x: isize, y: isize) -> Self {
        Self {
            id: (c as u8 - b'A' + 1) as usize,
            point: Point::new(x, y),
            hits: HashMap::default(),
        }
    }

    fn calc_hits(&mut self, max_power: usize) {
        let mut p = self.point;
        println!("{max_power}");
        'power: for power in 1..max_power {
            p.x += 1;
            p.y -= 1;
            self.hits.insert(p, (power, power));
            // Step to the right
            for t in 1..power {
                let p = Point::new(p.x + t as isize, p.y);
                if power + t > max_power {
                    continue 'power;
                }
                self.hits.insert(p, (power, power + t));
            }
            // Step diag down
            let mut p: Point<isize> = Point::new(p.x + power as isize, p.y);
            let mut t = 2 * power;
            while p.y <= 0 {
                t += 1;
                p.y += 1;
                p.x += 1;
                if t > max_power {
                    continue 'power;
                }
                self.hits.insert(p, (power, t));
            }
        }
    }

    fn can_hit_target(&self, target: &Point<isize>) -> Option<(usize, usize)> {
        let dy = target.y - self.point.y;
        let dx = target.x - self.point.x;
        if dx <= 0 {
            return None;
        }

        match dx.cmp(&(-dy)) {
            Ordering::Equal => Some((dx as usize, dx as usize)),
            Ordering::Less => None,
            Ordering::Greater => {
                let power = (dx - dy) / 3;
                if power == 0 {
                    None
                } else if (power * 3) + dy == dx {
                    // Up, across, down
                    Some((power as usize, dx as usize))
                } else if dy < 0 && dx > -dy && dx <= -dy * 2 {
                    // Up, across
                    // panic!("{dy}");
                    Some((-dy as usize, dx as usize))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Target {
    point: Point<isize>,
    hp: usize,
}

impl Target {
    fn new(hp: usize, x: usize, y: usize) -> Self {
        Self {
            point: Point::new(x as isize, y as isize),
            hp,
        }
    }
}

#[derive(Default)]
pub struct Day12 {
    catapults: Vec<Catapult>,
    targets: Vec<Target>,
    meteors: Vec<Point<isize>>,
}

impl Day12 {
    pub fn new() -> Self {
        Self::default()
    }

    fn find_ranking(&self) -> usize {
        let mut rankings = 0;
        let mut targets: VecDeque<Target> = self.targets.iter().copied().collect();
        let mut remaining: VecDeque<Target> = VecDeque::new();
        while !targets.is_empty() {
            'find_target: while let Some(mut target) = targets.pop_front() {
                if remaining
                    .iter()
                    .any(|t| t.hp > 0 && t.point.x == target.point.x && t.point.y < target.point.y)
                {
                    // Can't target if there is something above it
                    remaining.push_back(target);
                    continue;
                }
                for catapult in self.catapults.iter() {
                    if let Some((power, _)) = catapult.can_hit_target(&target.point) {
                        rankings += power * catapult.id;
                        target.hp -= 1;
                        if target.hp > 0 {
                            remaining.push_back(target);
                        }
                        remaining.append(&mut targets);
                        break 'find_target;
                    }
                }

                remaining.push_back(target);
            }
            std::mem::swap(&mut targets, &mut remaining);
        }
        rankings
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.find_ranking().into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.find_ranking().into())
    }

    fn part3(&mut self) -> Result<helper::RunOutput, Error> {
        let mut ranking = 0;

        let max_power = self
            .meteors
            .iter()
            .map(|p| p.y)
            .min()
            .unwrap()
            .unsigned_abs();
        for catapult in self.catapults.iter_mut() {
            catapult.calc_hits(max_power);
        }

        let mut actual_max_power = usize::MIN;

        for meteor in self.meteors.iter() {
            let mut min_ranking = usize::MAX;
            let mut meteor = *meteor;
            let mut t = 0;
            while meteor.y < 0 && min_ranking == usize::MAX {
                meteor.x -= 1;
                meteor.y += 1;
                t += 1;
                for catapult in self.catapults.iter() {
                    if let Some((power, time)) = catapult.hits.get(&meteor) {
                        if *time <= t {
                            min_ranking = min_ranking.min(catapult.id * power);
                            actual_max_power = actual_max_power.max(*time);
                        }
                    }
                }
            }

            assert_ne!(min_ranking, usize::MAX);
            ranking += min_ranking;
        }

        println!("{max_power} {actual_max_power}");

        Ok(ranking.into())
    }
}

impl helper::Runner for Day12 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        if lines[0].starts_with('.') {
            for (y, line) in lines.iter().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    match c {
                        'A' | 'B' | 'C' => self
                            .catapults
                            .push(Catapult::new(c, x as isize, y as isize)),
                        'T' => self.targets.push(Target::new(1, x, y)),
                        'H' => self.targets.push(Target::new(2, x, y)),
                        _ => assert!(matches!(c, '.' | '=')),
                    }
                }
            }
        } else {
            self.catapults.push(Catapult::new('A', 0, 0));
            self.catapults.push(Catapult::new('B', 0, -1));
            self.catapults.push(Catapult::new('C', 0, -2));

            for line in lines.iter() {
                if let Some((x, y)) = line.split_once(' ') {
                    self.meteors.push(Point::new(x.parse()?, -y.parse()?));
                } else {
                    return Err(Error::InvalidInput(line.into()));
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

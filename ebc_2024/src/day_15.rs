#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Point};
use std::{
    collections::{BTreeMap, VecDeque},
    ops::RangeInclusive,
};

#[derive(Copy, Clone)]
struct Path {
    to: Point<u8>,
    to_picked_up: u32,
    to_last_pickup: u32,
    to_dist: usize,
}

#[derive(Default)]
pub struct Day15 {
    map: Vec<Vec<char>>,
    start: Point<u8>,
    herbs: HashMap<char, Vec<Point<u8>>>,
    paths: HashMap<Point<u8>, Vec<Path>>,
}

impl Day15 {
    pub fn new() -> Self {
        Self::default()
    }

    fn paths_from(&self, from: Point<u8>) -> Vec<Path> {
        let start = self.map[from.y as usize][from.x as usize];
        let mut work = VecDeque::new();
        work.push_front((0, from, 0));
        let mut seen = HashSet::default();
        seen.insert(from);

        let mut paths = Vec::new();
        while let Some((dist, at, picked_up)) = work.pop_front() {
            for next in [
                Point::new(at.x - 1, at.y),
                Point::new(at.x + 1, at.y),
                Point::new(at.x, at.y - 1),
                Point::new(at.x, at.y + 1),
            ] {
                if !seen.insert(next) {
                    continue;
                }
                match self.map[next.y as usize][next.x as usize] {
                    '.' => work.push_back((dist + 1, next, picked_up)),
                    c if c.is_ascii_alphabetic() && c != start => {
                        let last_pickup = 1u32 << (c as u8 - b'A');
                        if picked_up & last_pickup == 0 {
                            let picked_up = picked_up | last_pickup;
                            paths.push(Path {
                                to: next,
                                to_picked_up: picked_up,
                                to_last_pickup: last_pickup,
                                to_dist: dist + 1,
                            });
                        }
                        work.push_back((dist + 1, next, picked_up));
                    }
                    _ => {}
                }
            }
        }
        paths
    }

    fn find_paths(&mut self, start: Option<Point<u8>>, x_range: Option<RangeInclusive<usize>>) {
        if let Some(start) = start {
            self.start = start;
        } else {
            self.start = Point::new(self.map[0].iter().position(|c| *c == '.').unwrap() as u8, 1);
            self.map[0][self.start.x as usize] = '#';
            self.map[1][self.start.x as usize] = 'Z';
        }

        let x_range = if let Some(x_range) = x_range {
            x_range
        } else {
            0..=self.map[0].len()
        };

        self.herbs.clear();
        self.paths.clear();
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if !x_range.contains(&x) {
                    continue;
                }
                if c.is_ascii_alphabetic() {
                    self.herbs
                        .entry(*c)
                        .or_default()
                        .push(Point::new(x as u8, y as u8));
                }
            }
        }

        let herbs: Vec<char> = self.herbs.keys().copied().collect();
        for from_c in herbs.iter() {
            for from in self.herbs.get(from_c).unwrap() {
                let paths = self.paths_from(*from);
                self.paths.insert(*from, paths);
            }
        }
    }

    fn find_full_path(&mut self) -> Result<usize, Error> {
        let mut work: BTreeMap<usize, HashSet<(Point<u8>, u32)>> = BTreeMap::default();
        work.entry(2)
            .or_default()
            .insert((self.start, 1u32 << (b'Z' - b'A')));

        let mut seen: [HashMap<u32, usize>; 255 * 255] =
            std::array::from_fn(|_| Default::default());

        while let Some((dist, cur_work)) = work.pop_first() {
            for (at, picked_up) in cur_work {
                if picked_up.count_ones() as usize == self.herbs.len() {
                    if at == self.start {
                        return Ok(dist);
                    }
                    // return home
                    for Path { to, to_dist, .. } in self.paths.get(&at).unwrap().iter().copied() {
                        if to != self.start {
                            continue;
                        }
                        work.entry(dist + to_dist)
                            .or_default()
                            .insert((to, picked_up));
                    }
                    continue;
                }

                for Path {
                    to,
                    to_picked_up,
                    to_last_pickup,
                    to_dist,
                } in self.paths.get(&at).unwrap().iter().copied()
                {
                    if picked_up & to_last_pickup != 0 {
                        continue;
                    }
                    if picked_up & to_picked_up == to_picked_up {
                        continue;
                    }
                    let dist = dist + to_dist;
                    let at = to;
                    let picked_up = picked_up | to_picked_up;

                    let seen_idx = (at.y as usize * 255) + at.x as usize;
                    let seen = seen[seen_idx].entry(picked_up).or_insert(usize::MAX);
                    if *seen > dist {
                        *seen = dist;
                        work.entry(dist).or_default().insert((at, picked_up));
                    }
                }
            }
        }

        unreachable!();
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        let start = (self.map[0].iter().position(|c| *c == '.').unwrap(), 1);
        let mut work = VecDeque::new();
        work.push_front((0, start));
        let mut seen = HashSet::default();
        seen.insert((start.0, 0));
        seen.insert(start);

        while let Some((dist, at)) = work.pop_front() {
            for next in [
                (at.0 - 1, at.1),
                (at.0 + 1, at.1),
                (at.0, at.1 - 1),
                (at.0, at.1 + 1),
            ] {
                if !seen.insert(next) {
                    continue;
                }
                match self.map[next.1][next.0] {
                    '.' => work.push_back((dist + 1, next)),
                    'H' => return Ok(((dist + 2) * 2).into()),
                    _ => {}
                }
            }
        }
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        self.find_paths(None, None);
        Ok(self.find_full_path()?.into())
    }

    fn part3(&mut self) -> Result<helper::RunOutput, Error> {
        if self.map[0].len() != 255 {
            self.find_paths(None, None);
            Ok(self.find_full_path()?.into())
        } else {
            // split in to three columns
            self.map[75][84] = 'Z';
            self.map[75][85] = '#';
            self.map[75][86] = 'Y';
            self.map[75][168] = 'X';
            self.map[75][169] = '#';
            self.map[75][170] = 'Z';

            let start = Point::new(self.map[0].iter().position(|c| *c == '.').unwrap() as u8, 1);
            self.map[0][start.x as usize] = '#';
            self.map[1][start.x as usize] = 'Z';

            let mut min_dist = 4;

            for (start, x_range) in [
                (Point::new(84, 75), 0..=84),
                (Point::new(170, 75), 170..=255),
                (start, 86..=168),
            ] {
                self.find_paths(Some(start), Some(x_range));
                min_dist += self.find_full_path()?;
            }

            Ok(min_dist.into())
        }
    }
}

impl helper::Runner for Day15 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.map.push(line.chars().collect());
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

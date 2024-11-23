#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Default)]
pub struct Day15 {
    map: Vec<Vec<char>>,
    start: (usize, usize),
    herbs: HashMap<char, Vec<(usize, usize)>>,
    paths: HashMap<(usize, usize), Vec<((usize, usize), u32, u32, usize)>>,
}

impl Day15 {
    pub fn new() -> Self {
        Self::default()
    }

    fn paths_from(&self, from: (usize, usize)) -> Vec<((usize, usize), u32, u32, usize)> {
        let start = self.map[from.1][from.0];
        let mut work = VecDeque::new();
        work.push_front((0, from, 0));
        let mut seen = HashSet::default();
        seen.insert(from);

        let mut paths = Vec::new();
        while let Some((dist, at, picked_up)) = work.pop_front() {
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
                    '.' => work.push_back((dist + 1, next, picked_up)),
                    c if c.is_ascii_alphabetic() && c != start => {
                        let last_pickup = 1u32 << (c as u8 - b'A');
                        if picked_up & last_pickup == 0 {
                            let picked_up = picked_up | last_pickup;
                            paths.push((next, picked_up, last_pickup, dist + 1));
                        }
                        work.push_back((dist + 1, next, picked_up));
                    }
                    _ => {}
                }
            }
        }
        paths
    }

    fn find_paths(&mut self) {
        self.start = (self.map[0].iter().position(|c| *c == '.').unwrap(), 1);
        self.map[0][self.start.0] = '#';
        self.map[1][self.start.0] = 'Z';

        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if c.is_ascii_alphabetic() {
                    self.herbs.entry(*c).or_default().push((x, y));
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
        println!("Making paths");
        self.find_paths();
        println!("Paths made");

        let mut work: BTreeMap<usize, HashSet<((usize, usize), u32)>> = BTreeMap::default();
        work.entry(2)
            .or_default()
            .insert((self.start, 1u32 << (b'Z' - b'A')));

        let mut seen: HashMap<((usize, usize), u32), usize> = HashMap::default();

        while let Some((dist, cur_work)) = work.pop_first() {
            println!("{dist}");
            for (at, picked_up) in cur_work {
                if picked_up.count_ones() as usize == self.herbs.len() {
                    if at == self.start {
                        return Ok(dist);
                    }
                    // return home
                    for (to, _, _, to_dist) in self.paths.get(&at).unwrap().iter().copied() {
                        if to != self.start {
                            continue;
                        }
                        work.entry(dist + to_dist)
                            .or_default()
                            .insert((to, picked_up));
                    }
                    continue;
                }

                for (to, to_picked_up, to_last_pickup, to_dist) in
                    self.paths.get(&at).unwrap().iter().copied()
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

                    let seen = seen.entry((at, picked_up)).or_insert(usize::MAX);
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
        Ok(self.find_full_path()?.into())
    }

    fn part3(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.find_full_path()?.into())
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

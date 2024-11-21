#[allow(unused_imports)]
use helper::{print, println, BitArray, Error, HashMap, HashSet, Lines, LinesOpt, Point};
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Cost(usize);

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Tile {
    point: Point<isize>,
    level: i8,
    is_start: bool,
    is_end: bool,
    neighbors: Vec<(usize, usize)>,
}

#[derive(Default)]
pub struct Day13 {
    tiles: Vec<Tile>,
    start: usize,
    end: usize,
}

impl Day13 {
    pub fn new() -> Self {
        Self::default()
    }

    fn map<F>(&mut self, start: usize, is_end: F) -> usize
    where
        F: Fn(&Tile) -> bool,
    {
        let mut work = BinaryHeap::new();
        work.push((Cost(0), start));

        let mut seen = HashMap::default();
        seen.insert(start, 0);

        while let Some((cost, at)) = work.pop() {
            if is_end(&self.tiles[at]) {
                return cost.0;
            }

            let neighbors = self.tiles[at].neighbors.clone();
            for (to, time) in neighbors {
                let cost = Cost(cost.0 + time);

                let seen = seen.entry(to).or_insert(usize::MAX);
                if *seen > cost.0 {
                    *seen = cost.0;
                    work.push((cost, to));
                }
            }
        }
        unreachable!()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.map(self.start, |t| t.is_end).into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.map(self.start, |t| t.is_end).into())
    }

    fn part3(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.map(self.end, |t| t.is_start).into())
    }
}

impl helper::Runner for Day13 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for (y, line) in lines.iter().enumerate() {
            let y = y as isize;
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                let p = Point::new(x, y);
                match c {
                    'S' => {
                        self.start = self.tiles.len();
                        self.tiles.push(Tile {
                            point: p,
                            level: 0,
                            is_start: true,
                            is_end: false,
                            neighbors: Vec::new(),
                        });
                    }
                    'E' => {
                        self.end = self.tiles.len();
                        self.tiles.push(Tile {
                            point: p,
                            level: 0,
                            is_start: false,
                            is_end: true,
                            neighbors: Vec::new(),
                        });
                    }
                    c if c.is_ascii_digit() => {
                        self.tiles.push(Tile {
                            point: p,
                            level: (c as u8 - b'0') as i8,
                            is_start: false,
                            is_end: false,
                            neighbors: Vec::new(),
                        });
                    }
                    _ => {}
                }
            }
        }

        let mut tiles = HashMap::default();
        for (idx, tile) in self.tiles.iter().enumerate() {
            tiles.insert(tile.point, idx);
        }

        for from in 0..self.tiles.len() {
            let from_level = self.tiles[from].level;
            macro_rules! link {
                ($x:expr, $y:expr) => {{
                    if let Some(to) = tiles.get(&Point::new($x, $y)) {
                        let to_level = self.tiles[*to].level;
                        let time = from_level.abs_diff(to_level);
                        let time = time.min(10 - time) + 1;
                        self.tiles[from].neighbors.push((*to, time as usize));
                    }
                }};
            }

            let x = self.tiles[from].point.x;
            let y = self.tiles[from].point.y;
            link!(x - 1, y);
            link!(x + 1, y);
            link!(x, y - 1);
            link!(x, y + 1);
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

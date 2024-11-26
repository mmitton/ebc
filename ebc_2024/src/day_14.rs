#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Point3D};
use std::{collections::VecDeque, str::FromStr};

#[derive(Debug)]
enum Direction {
    X(isize),
    Y(isize),
    Z(isize),
}

#[derive(Debug)]
struct Branch {
    directions: Vec<Direction>,
    leaf: Point3D<isize>,
}

impl Branch {
    fn grow(&mut self) -> HashSet<Point3D<isize>> {
        let mut segments = HashSet::default();

        for dir in self.directions.iter() {
            macro_rules! grow {
                ($delta:expr, $x:expr, $y:expr, $z:expr) => {
                    for _ in 0..$delta {
                        self.leaf.x += $x;
                        self.leaf.y += $y;
                        self.leaf.z += $z;
                        segments.insert(self.leaf);
                    }
                };
            }
            match dir {
                Direction::X(delta) => grow!(delta.unsigned_abs(), delta / delta.abs(), 0, 0),
                Direction::Y(delta) => grow!(delta.unsigned_abs(), 0, delta / delta.abs(), 0),
                Direction::Z(delta) => grow!(delta.unsigned_abs(), 0, 0, delta / delta.abs()),
            }
        }
        segments
    }
}

impl FromStr for Branch {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut directions = Vec::new();
        for dir in s.split(',') {
            directions.push(dir.parse()?);
        }
        Ok(Self {
            directions,
            leaf: Point3D::new(0, 0, 0),
        })
    }
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = &s[1..];
        match &s[..1] {
            "U" => Ok(Self::Y(num.parse()?)),
            "D" => Ok(Self::Y(-num.parse()?)),
            "L" => Ok(Self::X(num.parse()?)),
            "R" => Ok(Self::X(-num.parse()?)),
            "F" => Ok(Self::Z(num.parse()?)),
            "B" => Ok(Self::Z(-num.parse()?)),
            _ => Err(Error::InvalidInput(s.into())),
        }
    }
}

#[derive(Default)]
pub struct Day14 {
    branches: Vec<Branch>,
}

impl Day14 {
    pub fn new() -> Self {
        Self::default()
    }

    fn find_murkiness_to_trunk(
        tree: &HashSet<Point3D<isize>>,
        from: Point3D<isize>,
    ) -> Vec<(isize, usize)> {
        let mut seen = HashSet::default();
        seen.insert(from);
        let mut work = VecDeque::new();
        work.push_back((from, 0));

        let mut murkiness = Vec::new();
        while let Some((p, dist)) = work.pop_front() {
            if p.x == 0 && p.z == 0 {
                murkiness.push((p.y, dist));
            }

            macro_rules! add_work {
                ($p:expr) => {{
                    if tree.contains(&$p) && !seen.contains(&$p) {
                        seen.insert($p);
                        work.push_back(($p, dist + 1));
                    }
                }};
            }

            add_work!(Point3D::new(p.x - 1, p.y, p.z));
            add_work!(Point3D::new(p.x + 1, p.y, p.z));
            add_work!(Point3D::new(p.x, p.y - 1, p.z));
            add_work!(Point3D::new(p.x, p.y + 1, p.z));
            add_work!(Point3D::new(p.x, p.y, p.z - 1));
            add_work!(Point3D::new(p.x, p.y, p.z + 1));
        }

        murkiness
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        let segments = self.branches[0].grow();
        Ok(segments
            .iter()
            .map(|Point3D { y, .. }| *y)
            .max()
            .unwrap()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .branches
            .iter_mut()
            .fold(HashSet::default(), |mut segments, branch| {
                branch.grow().iter().for_each(|s| {
                    let _ = segments.insert(*s);
                });
                segments
            })
            .len()
            .into())
    }

    fn part3(&mut self) -> Result<helper::RunOutput, Error> {
        let mut leaves = HashSet::default();
        let tree = self
            .branches
            .iter_mut()
            .fold(HashSet::default(), |mut segments, branch| {
                branch.grow().iter().for_each(|s| {
                    let _ = segments.insert(*s);
                });
                leaves.insert(branch.leaf);
                segments
            });

        let mut murkiness: HashMap<isize, usize> = HashMap::default();
        leaves.iter().for_each(|l| {
            Self::find_murkiness_to_trunk(&tree, *l)
                .iter()
                .for_each(|(y, dist)| *murkiness.entry(*y).or_default() += dist)
        });

        Ok(murkiness.values().min().copied().unwrap().into())
    }
}

impl helper::Runner for Day14 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.branches.push(line.parse()?);
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

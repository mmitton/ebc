#[allow(unused_imports)]
use helper::{
    print, println, Error, GroupedPermutations, HashMap, HashSet, Lines, LinesOpt, Permutations,
};
use std::str::FromStr;

#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct Device {
    total: isize,
    name: String,
    val: isize,
    commands: Vec<Command>,
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Command {
    Inc,
    Dec,
    Stay,
    StartEnd,
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inc => write!(f, "+"),
            Self::Dec => write!(f, "-"),
            Self::Stay => write!(f, "="),
            Self::StartEnd => write!(f, "S"),
        }
    }
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Inc),
            "-" => Ok(Self::Dec),
            "=" => Ok(Self::Stay),
            "S" => Ok(Self::StartEnd),
            _ => Err(Error::InvalidInput(s.into())),
        }
    }
}

impl FromStr for Device {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((name, commands_str)) = s.split_once(':') {
            let name = name.into();
            let mut commands: Vec<Command> = Vec::new();
            for command in commands_str.split(',') {
                commands.push(command.parse()?);
            }

            return Ok(Self {
                name,
                commands,
                val: 10,
                total: 0,
            });
        }

        Err(Error::InvalidInput(s.into()))
    }
}

impl Device {
    fn reset(&mut self) {
        self.total = 0;
        self.val = 10;
    }

    fn step(&mut self, step: usize, track: Option<Command>) {
        let cmd = match track {
            None | Some(Command::StartEnd) | Some(Command::Stay) => {
                self.commands[step % self.commands.len()]
            }
            Some(c) => c,
        };
        match cmd {
            Command::Inc => self.val += 1,
            Command::Dec => self.val -= 1,
            Command::StartEnd | Command::Stay => {}
        }
        self.total += self.val;
    }
}

#[derive(Default)]
pub struct Day07 {
    track: Vec<Command>,
    devices: Vec<Device>,
}

impl Day07 {
    pub fn new() -> Self {
        Self::default()
    }

    fn race_track(&mut self, laps: usize) -> String {
        self.devices.iter_mut().for_each(Device::reset);
        let mut lap = 0;
        while lap < laps {
            let lap_step = lap * self.track.len();
            if lap != 0 && lap_step % self.devices[0].commands.len() == 0 {
                let remaining_loops = (laps - lap) / lap;
                lap += remaining_loops * lap;
                if lap == laps {
                    break;
                }
            }
            for (step, track) in self.track.iter().enumerate() {
                self.devices
                    .iter_mut()
                    .for_each(|d| d.step(lap_step + step, Some(*track)));
            }
            lap += 1;
        }
        self.devices.sort();
        let mut rank = String::new();
        self.devices
            .iter()
            .rev()
            .for_each(|d| rank.push_str(&d.name));
        rank
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        for step in 0..10 {
            self.devices.iter_mut().for_each(|d| d.step(step, None));
        }
        self.devices.sort();
        let mut rank = String::new();
        self.devices
            .iter()
            .rev()
            .for_each(|d| rank.push_str(&d.name));
        Ok(rank.into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.race_track(10).into())
    }

    fn part3(&mut self) -> Result<helper::RunOutput, Error> {
        let mut wins = 0;
        if true {
            let mut group_perms = GroupedPermutations::new([
                (Command::Inc, 5),
                (Command::Dec, 3),
                (Command::Stay, 3),
            ]);
            while let Some(commands) = group_perms.next_permutation() {
                self.devices.retain(|d| d.name == "A");
                self.devices.push(Device {
                    commands: commands.into(),
                    total: 0,
                    name: "B".into(),
                    val: 0,
                });

                if self.race_track(2024) == "BA" && self.devices[1].total > self.devices[0].total {
                    wins += 1;
                }
            }
        } else {
            let mut b = vec![
                Command::Inc,
                Command::Inc,
                Command::Inc,
                Command::Inc,
                Command::Inc,
                Command::Dec,
                Command::Dec,
                Command::Dec,
                Command::Stay,
                Command::Stay,
                Command::Stay,
            ];

            let mut wins = 0;
            let mut seen: HashSet<Vec<Command>> = HashSet::default();
            Permutations::iter(&mut b, |commands| {
                if !seen.insert(commands.into()) {
                    return;
                }
                self.devices.retain(|d| d.name == "A");
                self.devices.push(Device {
                    commands: commands.into(),
                    total: 0,
                    name: "B".into(),
                    val: 0,
                });

                if self.race_track(2024) == "BA" && self.devices[1].total > self.devices[0].total {
                    wins += 1;
                }
            });
        }

        Ok(wins.into())
    }
}

impl helper::Runner for Day07 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for (idx, line) in lines.iter().enumerate() {
            if &line[1..=1] == ":" {
                self.devices.push(line.parse()?);
            } else {
                // Parse track
                let track: &[String] = &lines[idx..];

                let mut track_map = HashMap::default();
                for (y, line) in track.iter().enumerate() {
                    for x in 0..line.len() {
                        track_map.insert((x as isize, y as isize), &track[y][x..x + 1]);
                    }
                }

                let mut xy = (0isize, 0isize);
                let mut dxy = (1isize, 0isize);
                loop {
                    let next_xy = (xy.0 + dxy.0, xy.1 + dxy.1);
                    match track_map.get(&next_xy).copied() {
                        None | Some(" ") => {
                            // Change Dir
                            match dxy {
                                (1, 0) | (-1, 0) => {
                                    // Scan up/down
                                    match track_map.get(&(xy.0, xy.1 - 1)).copied() {
                                        Some("+") | Some("-") | Some("=") | Some("S") => {
                                            dxy = (0, -1)
                                        }
                                        _ => dxy = (0, 1),
                                    }
                                }
                                (0, 1) | (0, -1) => {
                                    // Scan left/right
                                    match track_map.get(&(xy.0 - 1, xy.1)).copied() {
                                        Some("+") | Some("-") | Some("=") | Some("S") => {
                                            dxy = (-1, 0)
                                        }
                                        _ => dxy = (1, 0),
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        Some("S") => {
                            self.track.push(Command::StartEnd);
                            break;
                        }
                        Some(s) => {
                            self.track.push(s.parse()?);
                            xy = next_xy;
                        }
                    }
                }
                break;
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

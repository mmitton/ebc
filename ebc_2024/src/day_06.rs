#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Debug)]
pub enum Node {
    Root {
        children: Vec<usize>,
    },
    Branch {
        name: String,
        parent: usize,
        children: Vec<usize>,
        depth: usize,
    },
    Fruit {
        parent: usize,
        depth: usize,
    },
}

impl Node {
    fn parent(&self) -> Option<usize> {
        match self {
            Self::Root { .. } => None,
            Self::Branch { parent, .. } | Self::Fruit { parent, .. } => Some(*parent),
        }
    }
}

#[derive(Default)]
pub struct Day06 {
    nodes: Vec<Node>,
    root: usize,
}

impl Day06 {
    pub fn new() -> Self {
        Self::default()
    }

    fn path_to(&self, shorten_name: bool, fruit_idx: usize) -> String {
        let mut path: Vec<String> = Vec::new();
        let mut idx = fruit_idx;
        while let Some(node) = self.nodes.get(idx) {
            match node {
                Node::Root { .. } => {
                    path.push("RR".into());
                    idx = usize::MAX;
                }
                Node::Branch { name, parent, .. } => {
                    path.push(name.clone());
                    idx = *parent;
                }
                Node::Fruit { parent, .. } => {
                    path.push("@".into());
                    idx = *parent;
                }
            }
        }

        path.reverse();
        if shorten_name {
            path.iter().map(|s| &s[..1]).collect::<Vec<&str>>().join("")
        } else {
            path.join("")
        }
    }

    fn find_best_path(&self, shorten_name: bool) -> Result<String, Error> {
        let mut depths: HashMap<usize, Vec<usize>> = HashMap::default();
        for (idx, node) in self.nodes.iter().enumerate() {
            if let Node::Fruit { depth, .. } = node {
                if *depth != usize::MAX {
                    depths.entry(*depth).or_default().push(idx);
                }
            }
        }

        let min_share = depths
            .iter()
            .fold((0, usize::MAX), |best, (depth, paths)| {
                if paths.len() < best.1 {
                    (*depth, paths.len())
                } else {
                    best
                }
            })
            .0;

        let paths = depths.get(&min_share).unwrap();
        if paths.len() == 1 {
            Ok(self.path_to(shorten_name, paths[0]))
        } else {
            Err(Error::Unsolved)
        }
    }

    fn get_depth(&mut self, idx: usize) -> usize {
        let mut path_depth = 0;
        let mut parent = self.nodes[idx].parent();
        let mut seen = HashSet::default();
        seen.insert(idx);
        while let Some(parent_idx) = parent {
            if !seen.insert(parent_idx) {
                path_depth = usize::MAX;
                break;
            }
            path_depth += 1;
            parent = self.nodes[parent_idx].parent();
        }
        match &mut self.nodes[idx] {
            Node::Root { .. } => {}
            Node::Branch { ref mut depth, .. } | Node::Fruit { ref mut depth, .. } => {
                *depth = path_depth
            }
        }
        path_depth
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.find_best_path(false)?.into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.find_best_path(true)?.into())
    }

    fn part3(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.find_best_path(true)?.into())
    }
}

impl helper::Runner for Day06 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let mut names: HashMap<String, usize> = HashMap::default();
        macro_rules! get_name {
            ($name:expr, $parent_idx:expr) => {{
                if $name == "@" {
                    let id = self.nodes.len();
                    self.nodes.push(Node::Fruit {
                        parent: $parent_idx,
                        depth: usize::MAX,
                    });
                    id
                } else {
                    use std::collections::hash_map::Entry;
                    match names.entry($name.into()) {
                        Entry::Occupied(e) => *e.get(),
                        Entry::Vacant(e) => {
                            let id = self.nodes.len();
                            e.insert(id);
                            if $name == "RR" {
                                self.root = id;
                                self.nodes.push(Node::Root {
                                    children: Vec::new(),
                                });
                            } else {
                                self.nodes.push(Node::Branch {
                                    name: $name.into(),
                                    parent: $parent_idx,
                                    children: Vec::new(),
                                    depth: usize::MAX,
                                });
                            }
                            id
                        }
                    }
                }
            }};
        }

        for line in lines.iter() {
            let (parent, children) = line.split_once(':').unwrap();
            let parent_idx = get_name!(parent, usize::MAX);
            for child in children.split(',') {
                let child_idx = get_name!(child, parent_idx);
                match self.nodes.get_mut(parent_idx) {
                    Some(Node::Root { children }) | Some(Node::Branch { children, .. }) => {
                        children.push(child_idx);
                    }
                    _ => unreachable!(),
                }
                if let Some(Node::Branch { parent, .. }) = self.nodes.get_mut(child_idx) {
                    *parent = parent_idx;
                }
            }
        }

        for i in 0..self.nodes.len() {
            self.get_depth(i);
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

use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::str::FromStr;

use glam::{ivec2, IVec2};

pub fn read_lines(path: &Path) -> Result<Vec<String>, Error> {
    let io = File::open(path)?;
    let br = BufReader::new(io);
    let mut v: Vec<String> = vec![];
    for line in br.lines() {
        v.push(line?);
    }
    Ok(v)
}

pub fn parse_input<T>(input: &[String]) -> Vec<T>
where
    T: FromStr + Default,
{
    input
        .iter()
        .map(|i| i.parse().unwrap_or_default())
        .collect()
}

pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + std::hash::Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

lazy_static! {
    static ref DIRECTIONS_4: [IVec2; 4] = [
        //LEFT
        ivec2(-1, 0),
        //RIGHT
        ivec2(1, 0),
        //DOWN
        ivec2(0, -1),
        //UP
        ivec2(0, 1),
    ];

    static ref DIRECTIONS_8: [IVec2; 8] = [
        //LEFT
        ivec2(-1, 0),
        //RIGHT
        ivec2(1, 0),
        //DOWN
        ivec2(0, -1),
        //UP
        ivec2(0, 1),
        //UP LEFT
        ivec2(-1, 1),
        //UP RIGHT
        ivec2(1, 1),
        //DOWN LEFT
        ivec2(-1, -1),
        //DOWN RIGHT
        ivec2(1, -1),
    ];

}

pub trait Grid<T> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn values(&self) -> &[T];

    fn get_index(&self, pos: IVec2) -> usize {
        let x = pos.x as usize;
        let y = pos.y as usize;
        x + y * self.width()
    }

    fn try_get_index(&self, pos: IVec2) -> Option<usize> {
        if self.has_position(pos) {
            Some(self.get_index(pos))
        } else {
            None
        }
    }

    fn has_position(&self, pos: IVec2) -> bool {
        !(pos.x < 0 || pos.x >= self.width() as i32 || pos.y < 0 || pos.y >= self.height() as i32)
    }

    fn get_position(&self, pos: IVec2) -> &T {
        &self.values()[self.get_index(pos)]
    }

    fn try_get_position(&self, pos: IVec2) -> Option<&T> {
        if self.has_position(pos) {
            Some(self.get_position(pos))
        } else {
            None
        }
    }

    fn get_neighbours(&self, pos: IVec2, include_diagonal: bool) -> Vec<IVec2> {
        match include_diagonal {
            true => DIRECTIONS_8
                .into_iter()
                .map(|d| pos + d)
                .filter(|p| self.has_position(*p))
                .collect(),
            false => DIRECTIONS_4
                .into_iter()
                .map(|d| pos + d)
                .filter(|p| self.has_position(*p))
                .collect(),
        }
    }
}

pub struct Graph<T> {
    pub idx: usize,
    pub nodes: HashMap<usize, GraphNode<T>>,
}

pub struct GraphNode<T> {
    pub value: T,
    pub children: HashSet<usize>,
}

impl<T: PartialEq + Copy> Graph<T> {
    pub fn new() -> Self {
        Self {
            idx: 0,
            nodes: HashMap::new(),
        }
    }

    pub fn has_node(&self, value: T) -> bool {
        self.nodes.values().map(|n| n.value).any(|v| v == value)
    }

    pub fn add_node(&mut self, value: T) -> usize {
        let id = self.idx;
        self.nodes.insert(self.idx, GraphNode::new(value));
        self.idx += 1;
        id
    }

    pub fn add_child_node(&mut self, parent_id: usize, value: T) -> usize {
        let child_id = self.add_node(value);
        self.nodes
            .get_mut(&parent_id)
            .unwrap()
            .children
            .insert(child_id);
        child_id
    }

    pub fn add_child(&mut self, parent_id: usize, child_id: usize) {
        self.nodes
            .get_mut(&parent_id)
            .unwrap()
            .children
            .insert(child_id);
    }

    pub fn try_get_node_id(&self, value: T) -> Option<usize> {
        for node in &self.nodes {
            if node.1.value == value {
                return Some(*node.0);
            }
        }
        None
    }

    pub fn get_node_id(&self, value: T) -> usize {
        self.try_get_node_id(value).unwrap()
    }

    pub fn get_child_ids(&self, parent_id: usize) -> &HashSet<usize> {
        &self.nodes.get(&parent_id).unwrap().children
    }
}

impl<T: PartialEq + Copy> Default for Graph<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> GraphNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            children: HashSet::new(),
        }
    }
}

use std::collections::{HashMap, HashSet};

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

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug)]
pub struct Graph<TId, TValue>
where
    TId: Eq + Hash + PartialEq + Copy,
{
    pub nodes: HashMap<TId, GraphNode<TId, TValue>>,
}

#[derive(Debug, Clone)]
pub struct GraphNode<TId: Eq + Hash + PartialEq + Copy, TValue> {
    pub id: TId,
    pub value: TValue,
    pub children: HashSet<TId>,
}

impl<TId: Eq + Hash + PartialEq + Copy, TValue> Graph<TId, TValue> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn get_node(&self, id: TId) -> &GraphNode<TId, TValue> {
        self.nodes.get(&id).unwrap()
    }

    pub fn get_mut_node(&mut self, id: TId) -> &mut GraphNode<TId, TValue> {
        self.nodes.get_mut(&id).unwrap()
    }

    pub fn get_or_add_node(&mut self, id: TId, value: TValue) -> &GraphNode<TId, TValue> {
        if !self.has_node(id) {
            self.add_node(id, value);
        }
        self.get_node(id)
    }

    pub fn get_or_add_mut_node(&mut self, id: TId, value: TValue) -> &mut GraphNode<TId, TValue> {
        if !self.has_node(id) {
            self.add_node(id, value);
        }
        self.get_mut_node(id)
    }

    pub fn has_node(&self, id: TId) -> bool {
        self.nodes.get(&id).is_some()
    }

    pub fn add_node(&mut self, id: TId, value: TValue) {
        if !self.has_node(id) {
            self.nodes.insert(id, GraphNode::new(id, value));
        }
    }

    pub fn add_nodes(&mut self, nodes: Vec<(TId, TValue)>) {
        for (id, value) in nodes {
            self.add_node(id, value);
        }
    }

    pub fn remove_node(&mut self, id: TId) -> Option<GraphNode<TId, TValue>> {
        self.nodes.remove(&id)
    }

    pub fn add_connection(&mut self, parent_id: TId, child_id: TId, directional: bool) {
        let parent = self.get_mut_node(parent_id);
        parent.add_child(child_id);

        if !directional {
            let child = self.get_mut_node(child_id);
            child.add_child(parent_id);
        }
    }

    pub fn get_children(&self, parent_id: TId) -> Vec<&GraphNode<TId, TValue>> {
        self.get_node(parent_id)
            .children
            .iter()
            .map(|n| self.get_node(*n))
            .collect()
    }

    pub fn get_children_ids(&self, parent_id: TId) -> &HashSet<TId> {
        &self.get_node(parent_id).children
    }
}

impl<TId: Eq + Hash + PartialEq + Copy, TValue> Default for Graph<TId, TValue> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TId: Eq + Hash + PartialEq + Copy, TValue> GraphNode<TId, TValue> {
    pub fn new(id: TId, value: TValue) -> Self {
        Self {
            id,
            value,
            children: HashSet::new(),
        }
    }

    pub fn add_child(&mut self, child_id: TId) {
        self.children.insert(child_id);
    }
}

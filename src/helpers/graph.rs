use std::{collections::{hash_map::OccupiedError, HashMap, HashSet}, hash::Hash};

type NodeInsertError<'a, TId, TValue> = OccupiedError<'a, TId, GraphNode<TId, TValue>>;
pub trait GraphId: Eq + Hash + PartialEq + Copy {}

#[derive(Debug)]
pub struct Graph<TId, TValue>
where
    TId: GraphId,
{
    pub nodes: HashMap<TId, GraphNode<TId, TValue>>,
}

#[derive(Debug, Clone)]
pub struct GraphNode<TId: GraphId, TValue> {
    pub id: TId,
    pub value: TValue,
    pub children: HashSet<TId>,
}

impl<TId: GraphId, TValue> Graph<TId, TValue> {
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

    pub fn has_node(&self, id: TId) -> bool {
        self.nodes.get(&id).is_some()
    }

    pub fn try_add_node(
        &mut self,
        id: TId,
        value: TValue,
    ) -> Result<&mut GraphNode<TId, TValue>, NodeInsertError<TId, TValue>> {
        self.nodes.try_insert(id, GraphNode::new(id, value))
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

impl<TId: GraphId, TValue> Default for Graph<TId, TValue> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TId: GraphId, TValue> GraphNode<TId, TValue> {
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

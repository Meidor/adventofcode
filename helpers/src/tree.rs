use std::ops::Add;

#[derive(Debug)]
pub struct Tree<TValue: Copy + Add<Output = TValue>> {
    pub nodes: Vec<Node<TValue>>,
}

#[derive(Debug)]
pub struct Node<TValue: Copy + Add<Output = TValue>> {
    pub id: usize,
    pub label: String,
    pub value: TValue,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<TValue: Copy + Add<Output = TValue>> Tree<TValue> {
    pub fn new(label: &str, value: TValue) -> Self {
        Self {
            nodes: vec![Node::new(0, label, value, None)],
        }
    }

    pub fn get_parent(&self, child_id: usize) -> usize {
        self.nodes[child_id].parent.expect("child has no parent")
    }

    pub fn get_value(&self, id: usize) -> TValue {
        self.nodes[id].value
    }

    pub fn set_value(&mut self, id: usize, value: TValue) {
        self.nodes[id].value = value;
    }

    pub fn cascade_add_value(&mut self, id: usize, value: TValue) {
        let node = &mut self.nodes[id];
        node.value = node.value + value;
        if let Some(parent) = self.nodes[id].parent {
            self.cascade_add_value(parent, value);
        }
    }

    pub fn get_label(&self, id: usize) -> String {
        self.nodes[id].label.clone()
    }

    pub fn get_child_by_label(&self, parent_id: usize, label: &str) -> usize {
        let node = &self.nodes[parent_id];
        for id in &node.children {
            if self.nodes[*id].label == label {
                return *id;
            }
        }
        unreachable!(
            "couldn't find child in parent {} with label {}",
            parent_id, label
        );
    }

    pub fn add_child(&mut self, parent_id: usize, label: &str, value: TValue) {
        let id = self.nodes.len();
        let child = Node::new(id, label, value, Some(parent_id));
        self.nodes.push(child);
        let parent = &mut self.nodes[parent_id];
        parent.children.push(id);
    }
}

impl<TValue: Copy + Add<Output = TValue>> Node<TValue> {
    pub fn new(id: usize, label: &str, value: TValue, parent: Option<usize>) -> Self {
        Self {
            id,
            label: label.to_string(),
            value,
            parent,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
pub struct BinaryHeap<T> {
    nodes: Vec<T>,
}

impl<T> BinaryHeap<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
        }
    }
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, value: T) {
        self.nodes.push(value);
        self.lift_up(self.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.nodes.is_empty() {
            None
        } else {
            let last_idx = self.len() - 1;
            self.nodes.swap(0, last_idx);
            self.lift_down(0, last_idx);
            self.nodes.pop()
        }
    }

    pub fn into_sorted_vec(mut self) -> Vec<T> {
        for i in (1..self.len()).rev() {
            self.nodes.swap(0, i);
            self.lift_down(0, i);
        }
        self.nodes
    }

    fn lift_down(&mut self, idx: usize, len: usize) {
        let left_child = Self::left_child(idx);
        let right_child = Self::right_child(idx);

        let mut biggest = idx;
        if left_child < len && self.nodes[biggest].cmp(&self.nodes[left_child]).is_le() {
            biggest = left_child;
        }

        if right_child < len && self.nodes[biggest].cmp(&self.nodes[right_child]).is_le() {
            biggest = right_child;
        }

        if biggest != idx {
            self.nodes.swap(idx, biggest);
            self.lift_down(biggest, len);
        }
    }

    fn lift_up(&mut self, idx: usize) {
        if idx == 0 {
            return;
        }

        let parent = Self::parent(idx);
        if self.nodes[parent].cmp(&self.nodes[idx]).is_le() {
            self.nodes.swap(parent, idx);
            self.lift_up(parent);
        }
    }

    fn parent(idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn left_child(idx: usize) -> usize {
        idx * 2
    }

    fn right_child(idx: usize) -> usize {
        idx * 2 + 1
    }
}

impl<T> Clone for BinaryHeap<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut heap = BinaryHeap::new();
        heap.push(3);
        heap.push(1);
        heap.push(5);

        assert_eq!(heap.len(), 3);
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(3));

        heap.push(4);
        heap.push(2);
        assert_eq!(heap.len(), 3);
        assert_eq!(heap.is_empty(), false);
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
        assert_eq!(heap.is_empty(), true);
    }

    #[test]
    fn returns_sorted_vec() {
        let mut heap = BinaryHeap::with_capacity(8);
        heap.push(5);
        heap.push(2);
        heap.push(7);
        heap.push(8);
        heap.push(1);
        heap.push(6);
        heap.push(4);
        heap.push(3);

        assert_eq!(heap.into_sorted_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
}

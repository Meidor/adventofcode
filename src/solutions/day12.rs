use crate::helpers::{has_unique_elements, Graph, GraphNode};

impl GraphNode<&str> {
    fn is_small_cave(&self) -> bool {
        !self.value.chars().any(|c| c.is_uppercase())
    }
}

impl Graph<&str> {
    fn print_path(&self, path: &[usize]) {
        let nodes: Vec<&str> = path
            .iter()
            .map(|i| self.nodes.get(i).unwrap().value)
            .collect();
        println!("{}", nodes.join(" "));
    }

    fn is_start(&self, node: usize) -> bool {
        self.nodes.get(&node).unwrap().value == "start"
    }

    fn is_small_cave(&self, node: usize) -> bool {
        self.nodes.get(&node).unwrap().is_small_cave()
    }

    fn from_input(lines: &[String]) -> Graph<&str> {
        let mut g = Graph::default();
        let connections: Vec<Vec<&str>> = lines.iter().map(|l| l.split('-').collect()).collect();
        for connection in connections {
            let parent = connection[0];
            let child = connection[1];
            let parent_id = if !g.has_node(parent) {
                g.add_node(parent)
            } else {
                g.get_node_id(parent)
            };

            let child_id = if !g.has_node(child) {
                g.add_node(child)
            } else {
                g.get_node_id(child)
            };
            g.add_child(parent_id, child_id);
            g.add_child(child_id, parent_id);
        }
        g
    }

    fn get_paths(&self, start: usize, end: usize, is_part_one: bool) -> Vec<Vec<usize>> {
        let mut paths: Vec<Vec<usize>> = vec![];
        let mut path: Vec<usize> = vec![];
        let mut visited: Vec<usize> = vec![];
        path.push(start);
        visited.push(start);
        self.get_all_paths(start, end, &mut path, &mut paths, is_part_one);
        paths
    }

    fn should_visit_one(&self, node_id: usize, path: &[usize]) -> bool {
        !(path.contains(&node_id) && self.is_small_cave(node_id))
    }

    fn should_visit_two(&self, node_id: usize, path: &[usize]) -> bool {
        if self.is_start(node_id) {
            return false;
        }
        if !self.is_small_cave(node_id) {
            return true;
        }
        if !path.contains(&node_id) {
            return true;
        }
        has_unique_elements(path.iter().filter(|n| self.is_small_cave(**n)))
    }

    fn get_all_paths(
        &self,
        n: usize,
        target: usize,
        path: &mut Vec<usize>,
        paths: &mut Vec<Vec<usize>>,
        is_part_one: bool,
    ) {
        for c in self.get_child_ids(n) {
            let child = *c;
            if child == target {
                let mut solved_path = path.clone();
                solved_path.push(child);
                paths.push(solved_path);
                continue;
            }
            let should_visit = if is_part_one {
                self.should_visit_one(child, path)
            } else {
                self.should_visit_two(child, path)
            };

            if should_visit {
                path.push(child);
                self.get_all_paths(child, target, path, paths, is_part_one);
                path.pop();
            }
        }
    }
}

#[inline]
pub fn part_one(lines: &[String]) -> i64 {
    Graph::from_input(lines)
        .get_paths(
            Graph::from_input(lines).get_node_id("start"),
            Graph::from_input(lines).get_node_id("end"),
            true,
        )
        .len() as i64
}

#[inline]
pub fn part_two(lines: &[String]) -> i64 {
    Graph::from_input(lines)
        .get_paths(
            Graph::from_input(lines).get_node_id("start"),
            Graph::from_input(lines).get_node_id("end"),
            false,
        )
        .len() as i64
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<String> {
        vec![
            "start-A".to_string(),
            "start-b".to_string(),
            "A-c".to_string(),
            "A-b".to_string(),
            "b-d".to_string(),
            "A-end".to_string(),
            "b-end".to_string(),
        ]
    }

    #[test]
    fn test_part_one() {
        let expected = 10;
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 36;
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}

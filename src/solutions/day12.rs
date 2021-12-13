use crate::helpers::{has_unique_elements, Graph, GraphNode};

impl GraphNode<&str, &str> {
    fn is_small_cave(&self) -> bool {
        !self.value.chars().any(|c| c.is_uppercase())
    }
}

impl Graph<&str, &str> {
    fn from_input(lines: &[String]) -> Graph<&str, &str> {
        let mut g = Graph::default();
        let connections: Vec<Vec<&str>> = lines.iter().map(|l| l.split('-').collect()).collect();
        for connection in connections {
            let _ = g.try_add_node(connection[0], connection[0]);
            let _ = g.try_add_node(connection[1], connection[1]);
            g.add_connection(connection[0], connection[1], false);
        }
        g
    }

    fn print_path(&self, path: &[&str]) {
        println!("{}", path.join(" "));
    }

    fn is_start(&self, node: &str) -> bool {
        node == "start"
    }

    fn is_small_cave(&self, node: &str) -> bool {
        self.get_node(node).is_small_cave()
    }

    fn get_paths<'a>(
        &'a self,
        start: &'a str,
        end: &'a str,
        is_part_one: bool,
    ) -> Vec<Vec<&'a str>> {
        let mut paths: Vec<Vec<&str>> = vec![];
        let mut path: Vec<&str> = vec![];
        let mut visited: Vec<&str> = vec![];
        path.push(start);
        visited.push(start);
        self.get_all_paths(start, end, &mut path, &mut paths, is_part_one);
        paths
    }

    fn should_visit(&self, node_id: &str, path: &[&str], is_part_one: bool) -> bool {
        if self.is_start(node_id) {
            return false;
        }
        if !self.is_small_cave(node_id) {
            return true;
        }
        if !path.contains(&node_id) {
            return true;
        }
        if is_part_one {
            return false;
        }
        has_unique_elements(path.iter().filter(|n| self.is_small_cave(*n)))
    }

    fn get_all_paths<'a>(
        &'a self,
        n: &'a str,
        target: &'a str,
        path: &mut Vec<&'a str>,
        paths: &mut Vec<Vec<&'a str>>,
        is_part_one: bool,
    ) {
        let children = self.get_children_ids(n);
        for child in children {
            if *child == target {
                let mut solved_path = path.clone();
                solved_path.push(child);
                paths.push(solved_path);
                continue;
            }

            if self.should_visit(child, path, is_part_one) {
                path.push(child);
                self.get_all_paths(child, target, path, paths, is_part_one);
                path.pop();
            }
        }
    }
}

#[inline]
pub fn part_one(lines: &[String]) -> String {
    Graph::from_input(lines)
        .get_paths("start", "end", true)
        .len()
        .to_string()
}

#[inline]
pub fn part_two(lines: &[String]) -> String {
    Graph::from_input(lines)
        .get_paths("start", "end", false)
        .len()
        .to_string()
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
        let expected = "10";
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = "36";
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}

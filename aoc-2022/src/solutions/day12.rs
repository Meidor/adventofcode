use color_eyre::eyre::Result;
use glam::{ivec2, IVec2};
use helpers::{Graph, GraphNode, Grid, FilterGrid};
use priority_queue::PriorityQueue;
use std::collections::HashMap;

#[derive(Debug)]
struct HeightMap {
    start: IVec2,
    finish: IVec2,
    width: usize,
    height: usize,
    cells: Vec<usize>,
    graph: Option<Graph<IVec2, usize>>,
}

impl HeightMap {
    pub fn new(input: &str) -> Self {
        let lines = input.trim().lines().collect::<Vec<&str>>();
        let width = lines[0].len();
        let height = lines.len();
        let mut start: Option<IVec2> = None;
        let mut finish: Option<IVec2> = None;
        let mut i = 0;
        let cells: Vec<usize> = input
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| {
                let pos = ivec2(i % width as i32, i / width as i32);
                let height = match c {
                    'S' => {
                        start = Some(pos);
                        'a' as usize
                    }
                    'E' => {
                        finish = Some(pos);
                        'z' as usize
                    }
                    char => char as usize,
                };
                i += 1;
                height
            })
            .collect();

        let mut input_grid = Self {
            start: start.unwrap(),
            finish: finish.unwrap(),
            width,
            height,
            cells,
            graph: None,
        };

        let mut graph = Graph::<IVec2, usize>::new();
        for y in 0..height {
            for x in 0..width {
                let pos = ivec2(x as i32, y as i32);
                let height = *input_grid.get_position(pos);
                graph.add_node(pos, height);

                for neighbour_pos in input_grid.get_neighbours(pos, false) {
                    let neighbour_height = *input_grid.get_position(neighbour_pos);
                    graph.add_node(neighbour_pos, neighbour_height);
                    if neighbour_height <= height + 1 {
                        graph.add_connection(pos, neighbour_pos, true);
                    }
                }
            }
        }
        input_grid.graph = Some(graph);
        input_grid
    }
}

impl FilterGrid<usize> for HeightMap {}

impl Grid<usize> for HeightMap {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn values(&self) -> &[usize] {
        &self.cells
    }
}

type Node = GraphNode<IVec2, usize>;

fn find_shortest_path(
    graph: &Graph<IVec2, usize>,
    start: Vec<IVec2>,
    finish: IVec2,
) -> Option<usize> {
    let finish_node: &Node = graph.get_node(finish);
    let mut frontier = PriorityQueue::<IVec2, i32>::new();
    let mut cost_so_far: HashMap<IVec2, usize> = HashMap::new();
    let mut came_from: HashMap<IVec2, Option<IVec2>> = HashMap::new();
    for start_node in start.into_iter().map(|n| graph.get_node(n)) {
        frontier.push(start_node.id, 0);
        cost_so_far.insert(start_node.id, 0);
        came_from.insert(start_node.id, None);
    }

    while let Some((current, _)) = frontier.pop() {
        let node = graph.get_node(current);
        if node.id == finish_node.id {
            break;
        }
        for next in &node.children {
            let new_cost = cost_so_far.get(&current).unwrap() + 1;
            if !cost_so_far.contains_key(next) || new_cost < *cost_so_far.get(next).unwrap() {
                cost_so_far.insert(*next, new_cost);
                let dist = (*next - finish_node.id).abs();
                let h = dist.x as usize + dist.y as usize;
                let priority = 0 - (new_cost + h) as i32;
                frontier.push(*next, priority);

                came_from.insert(*next, Some(current));
            }
        }
    }
    let mut length = 0;
    let mut current_node = finish_node.id;
    loop {
        let n = came_from.get(&current_node);
        //THERE IS NO PATH TO THE FINISH
        n?;

        let n = n.unwrap();
        if n.is_none() {
            break;
        }
        current_node = n.unwrap();
        length += 1;
    }
    Some(length)
}

pub fn part_one(input: &str) -> Result<String> {
    let height_map = HeightMap::new(input);
    if let Some(graph) = height_map.graph {
        if let Some(length) = find_shortest_path(&graph, vec![height_map.start], height_map.finish)
        {
            return Ok(length.to_string());
        }
    }
    unreachable!("invalid input");
}

pub fn part_two(input: &str) -> Result<String> {
    let height_map = HeightMap::new(input);
    if let Some(ref graph) = height_map.graph {
        let start_postitions = height_map.filter_positions(|t| *t == 'a' as usize);
        if let Some(length) = find_shortest_path(graph, start_postitions, height_map.finish) {
            return Ok(length.to_string());
        }
    }
    unreachable!("invalid input");
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "31";
        let actual = part_one(test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "29";
        let actual = part_two(test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}

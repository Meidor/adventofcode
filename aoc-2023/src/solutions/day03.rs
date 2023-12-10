use helpers::{Graph, Grid};
use color_eyre::eyre::Result;
use glam::{ivec2, IVec2};

#[derive(Debug, Copy, Clone)]
enum SchematicPart {
    Symbol(char),
    Number(i32),
}

#[derive(Debug)]
struct EngineSchematic {
    grid: Vec<char>,
    width: usize,
    height: usize,
    graph: Graph<IVec2, SchematicPart>,
}

impl EngineSchematic {
    fn new(input: &str) -> Self {
        let mut g = Self {
            grid: input.chars().filter(|c| !c.is_ascii_whitespace()).collect(),
            width: input.lines().next().unwrap().len(),
            height: input.lines().count(),
            graph: Graph::new(),
        };
        g.build_graph();
        g
    }

    fn build_graph(&mut self) {
        self.build_graph_nodes();
        self.build_graph_connections();
        self.clean_graph();
    }

    fn build_graph_nodes(&mut self) {
        let mut x = 0;
        let mut y = 0;
        let mut pos = ivec2(x, y);
        let mut item: &char;

        while y < self.width as i32 {
            while x < self.height as i32 {
                item = self.get_position(pos);
                if !item.is_ascii_digit() {
                    if Self::is_symbol(*item) {
                        self.graph.add_node(pos, SchematicPart::Symbol(*item));
                    }
                    x += 1;
                    pos.x = x;
                    continue;
                }
                let start_pos = pos;
                let mut part_number = item.to_digit(10).unwrap() as i32;
                x += 1;
                pos.x = x;
                item = self.get_position(pos);
                while x < self.width as i32 && item.is_ascii_digit() {
                    part_number *= 10;
                    part_number += item.to_digit(10).unwrap() as i32;
                    x += 1;
                    pos.x = x;
                    item = self.get_position(pos);
                }
                self.graph
                    .add_node(start_pos, SchematicPart::Number(part_number));
            }
            y += 1;
            x = 0;
            pos.x = x;
            pos.y = y;
        }
    }

    fn build_graph_connections(&mut self) {
        let nodes: Vec<(IVec2, i32)> = self
            .graph
            .nodes
            .iter()
            .filter_map(|(pos, node)| match node.value {
                SchematicPart::Number(n) => Some((*pos, n)),
                _ => None,
            })
            .collect();

        for (pos, n) in nodes {
            let len = n.to_string().len();
            for x in pos.x..pos.x + len as i32 {
                let nb = self.get_neighbours(ivec2(x, pos.y), true);
                nb.iter().for_each(|n| {
                    if self.graph.has_node(*n) {
                        let node = self.graph.get_node(*n).value;
                        if let SchematicPart::Symbol(_) = node {
                            self.graph.add_connection(pos, *n, false);
                        }
                    }
                })
            }
        }
    }

    fn clean_graph(&mut self) {
        let ids = self.graph.nodes.keys().cloned().collect::<Vec<_>>();
        for id in ids {
            let node = self.graph.get_node(id);
            if let SchematicPart::Number(_) = node.value {
                if self.graph.get_children_ids(id).is_empty() {
                    self.graph.remove_node(id);
                }
            }
        }
    }

    fn get_part_numbers(&self) -> Vec<i32> {
        self.graph
            .nodes
            .iter()
            .filter_map(|(_, node)| match node.value {
                SchematicPart::Number(n) => Some(n),
                _ => None,
            })
            .collect()
    }

    fn get_gear_positions(&self) -> Vec<IVec2> {
        self.graph
            .nodes
            .iter()
            .filter_map(|(pos, node)| match node.value {
                SchematicPart::Symbol(c) if Self::is_gear(c) => Some(*pos),
                _ => None,
            })
            .collect()
    }

    fn get_gear_ratios(&self) -> Vec<i32> {
        self.get_gear_positions()
            .into_iter()
            .filter_map(|gp| {
                let children: Vec<i32> = self
                    .graph
                    .get_children(gp)
                    .iter()
                    .filter_map(|n| match n.value {
                        SchematicPart::Number(n) => Some(n),
                        _ => None,
                    })
                    .collect();
                if children.len() == 2 {
                    Some(children.iter().product())
                } else {
                    None
                }
            })
            .collect()
    }

    fn is_symbol(c: char) -> bool {
        !c.is_ascii_digit() && c != '.'
    }

    fn is_gear(c: char) -> bool {
        c == '*'
    }
}

impl Grid<char> for EngineSchematic {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn values(&self) -> &[char] {
        &self.grid
    }

    fn values_mut(&mut self) -> &mut [char] {
        &mut self.grid
    }
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    let schematic = EngineSchematic::new(input);
    Ok(schematic.get_part_numbers().iter().sum::<i32>().to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let schematic = EngineSchematic::new(input);
    Ok(schematic.get_gear_ratios().iter().sum::<i32>().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "4361";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "467835";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}

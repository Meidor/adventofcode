use crate::helpers::Tree;
use color_eyre::eyre::Result;

fn parse_directory_structure(input: &str) -> Tree<usize> {
    let mut dir_tree = Tree::<usize>::new("/", 0);
    let mut current_node: usize = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        if line.starts_with("$") {
            // COMMAND
            let command = parts[1];
            if command == "ls" {
                continue;
            } else if command == "cd" {
                let target_dir = parts[2];
                if target_dir == "/" {
                    current_node = 0;
                } else if target_dir == ".." {
                    current_node = dir_tree.get_parent(current_node);
                } else {
                    current_node = dir_tree.get_child_by_label(current_node, target_dir);
                }
            }
        } else if line.starts_with("dir") {
            // DIRECTORY
            let name = parts[1];
            dir_tree.add_child(current_node, name, 0);
        } else {
            // FILE
            let name = parts[1];
            let size: usize = parts[0].parse::<usize>().expect("couldn't parse filesize");
            dir_tree.add_child(current_node, name, size);
            dir_tree.cascade_add_value(current_node, size);
        }
    }
    dir_tree
}

pub fn part_one(input: &str) -> Result<String> {
    let directory_structure = parse_directory_structure(input);
    Ok(directory_structure
        .nodes
        .iter()
        .filter(|n| n.children.len() > 0)
        .map(|n| n.value)
        .filter(|n| *n <= 100000)
        .sum::<usize>()
        .to_string())
}

pub fn part_two(input: &str) -> Result<String> {
    let directory_structure = parse_directory_structure(input);
    let total_space: usize = 70000000;
    let target_size: usize = total_space - 30000000;
    let current_size = directory_structure.nodes[0].value;
    let min_needed = current_size - target_size;
    Ok(directory_structure
        .nodes
        .iter()
        .filter(|n| n.children.len() > 0)
        .map(|n| n.value)
        .filter(|n| *n >= min_needed)
        .min()
        .expect("no min found")
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "95437";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "24933642";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}

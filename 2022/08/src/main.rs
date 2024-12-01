use std::{collections::HashMap, fs};

fn main() {
    let tree_lines = fs::read_to_string("src/input.txt")
        .unwrap()
        .split('\n')
        .enumerate()
        .map(|(y, line)| (y as i32, line.trim().to_string()))
        .collect::<Vec<(i32, String)>>();

    // Part 2
    let highest_scenic_score = get_highest_scenic_score(&tree_lines);
    println!("Highest scenic score is {highest_scenic_score}");

    // Part 1
    let visible_trees = get_visible_trees(tree_lines);
    println!("Visible {visible_trees}");
}

#[derive(Debug)]
struct Tree {
    height: i32,
}

fn get_highest_scenic_score(tree_lines: &Vec<(i32, String)>) -> i32 {
    let mut trees = HashMap::<String, Tree>::new();
    let width = tree_lines[0].1.len();
    let height = tree_lines.len();

    // Register trees
    for (y, line) in tree_lines {
        for (x, value) in line.chars().enumerate() {
            let key: String = format!("{}-{}", x as i32, *y);
            let height = value.to_digit(10).unwrap() as i32;
            let tree = Tree { height };

            trees.insert(key, tree);
        }
    }

    let mut highest_scenic_score = 0;

    for (c, tree) in &trees {
        let coordinates = c.split_once('-').unwrap();
        let tree_x = coordinates.0.parse::<usize>().unwrap();
        let tree_y = coordinates.1.parse::<usize>().unwrap();

        let mut visible_trees_to_east = 0;

        for x in tree_x..width {
            if tree_x != x {
                let key = format!("{x}-{tree_y}");
                let current_tree = trees.get(&key).unwrap();
                visible_trees_to_east += 1;

                if current_tree.height >= tree.height {
                    break;
                }
            }
        }

        let mut visible_trees_to_west = 0;

        for x in (0..tree_x).rev() {
            let key = format!("{x}-{tree_y}");
            let current_tree = trees.get(&key).unwrap();
            visible_trees_to_west += 1;

            if current_tree.height >= tree.height {
                break;
            }
        }

        let mut visible_trees_to_south = 0;

        for y in tree_y..height {
            if tree_y != y {
                let key = format!("{tree_x}-{y}");
                let current_tree = trees.get(&key).unwrap();
                visible_trees_to_south += 1;

                if current_tree.height >= tree.height {
                    break;
                }
            }
        }

        let mut visible_trees_to_north = 0;

        for y in (0..tree_y).rev() {
            let key = format!("{tree_x}-{y}");
            let current_tree = trees.get(&key).unwrap();
            visible_trees_to_north += 1;

            if current_tree.height >= tree.height {
                break;
            }
        }

        let scenic_score = visible_trees_to_north
            * visible_trees_to_east
            * visible_trees_to_south
            * visible_trees_to_west;

        if scenic_score > highest_scenic_score {
            highest_scenic_score = scenic_score;
        }
    }

    highest_scenic_score
}

fn get_visible_trees(tree_lines: Vec<(i32, String)>) -> i32 {
    let mut trees = HashMap::<String, bool>::new();

    // Left ==> Right
    for (y, line) in &tree_lines {
        let mut highest_tree: Option<i32> = None;

        for (x, value) in line.chars().enumerate() {
            let is_visible = is_visible(char_to_i32(value), &mut highest_tree);

            let key: String = format!("{}-{y}", x as i32);
            trees.insert(key, is_visible);
        }
    }

    // Right ==> Left
    for (y, line) in &tree_lines {
        let mut parsed_line = line.chars().enumerate().collect::<Vec<(usize, char)>>();
        parsed_line.reverse();
        let mut highest_tree: Option<i32> = None;

        for (x, value) in parsed_line {
            let is_visible = is_visible(char_to_i32(value), &mut highest_tree);

            if is_visible {
                let key: String = format!("{}-{y}", x as i32);
                if let Some(tree) = trees.get_mut(&key) {
                    *tree = is_visible;
                }
            }
        }
    }

    let tree_columns = transpose(tree_lines);

    // Top ==> Bottom
    for (x, line) in &tree_columns {
        let mut highest_tree: Option<i32> = None;

        for (y, value) in line.chars().enumerate() {
            let is_visible = is_visible(char_to_i32(value), &mut highest_tree);

            if is_visible {
                let key: String = format!("{x}-{}", y as i32);
                if let Some(tree) = trees.get_mut(&key) {
                    *tree = is_visible;
                }
            }
        }
    }

    // Bottom ==> Top
    for (x, line) in &tree_columns {
        let mut parsed_line = line.chars().enumerate().collect::<Vec<(usize, char)>>();
        parsed_line.reverse();
        let mut highest_tree: Option<i32> = None;

        for (y, value) in parsed_line {
            let is_visible = is_visible(char_to_i32(value), &mut highest_tree);

            if is_visible {
                let key: String = format!("{x}-{}", y as i32);
                if let Some(tree) = trees.get_mut(&key) {
                    *tree = is_visible;
                }
            }
        }
    }

    trees
        .values()
        .fold(0, |acc, v| if *v { acc + 1 } else { acc })
}

fn char_to_i32(c: char) -> i32 {
    c.to_digit(10).unwrap() as i32
}

fn is_visible(height: i32, current_heighest: &mut Option<i32>) -> bool {
    let is_visible = match current_heighest {
        Some(current) => height > *current,
        None => true,
    };

    if is_visible {
        *current_heighest = Some(height);
    };

    is_visible
}

fn transpose(vec: Vec<(i32, String)>) -> Vec<(i32, String)> {
    let len = vec[0].1.len();

    [0..len]
        .map(|v| {
            v.map(|index| {
                vec.iter()
                    .fold(Vec::new(), |mut acc, (_, line)| {
                        let tree = line.chars().nth(index).unwrap();

                        acc.push(tree);
                        acc
                    })
                    .iter()
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
        })
        .into_iter()
        .next()
        .unwrap()
        .into_iter()
        .enumerate()
        .map(|(y, line)| (y as i32, line))
        .collect::<Vec<(i32, String)>>()
}

#[cfg(test)]
mod tests {
    use crate::{char_to_i32, get_highest_scenic_score, get_visible_trees, is_visible, transpose};

    #[test]
    fn is_visible_should_recognize_visible_tree() {
        let mut height: Option<i32> = None;
        let tree = '5';

        let is_visible = is_visible(char_to_i32(tree), &mut height);

        assert_eq!((is_visible, height.unwrap()), (true, 5));
    }

    #[test]
    fn is_visible_should_recognize_hidden_tree() {
        let mut height: Option<i32> = Some(3);
        let tree_one = '3';
        let is_visible_one = is_visible(char_to_i32(tree_one), &mut height);

        assert_eq!((is_visible_one, height.unwrap()), (false, 3));

        let tree_one_two = '2';
        let is_visible_two = is_visible(char_to_i32(tree_one_two), &mut height);

        assert_eq!((is_visible_two, height.unwrap()), (false, 3));
    }

    #[test]
    fn transpose_should_work() {
        let sample = Vec::from([
            (0, String::from("30373")),
            (1, String::from("25512")),
            (2, String::from("65332")),
            (3, String::from("33549")),
            (4, String::from("35390")),
        ]);
        let index = transpose(sample);

        let expected = Vec::from([
            (0, String::from("32633")),
            (1, String::from("05535")),
            (2, String::from("35353")),
            (3, String::from("71349")),
            (4, String::from("32290")),
        ]);

        assert_eq!(index, expected);
    }

    #[test]
    fn get_visible_trees_should_work() {
        let sample = Vec::from([
            (0, String::from("30373")),
            (1, String::from("25512")),
            (2, String::from("65332")),
            (3, String::from("33549")),
            (4, String::from("35390")),
        ]);

        let visible_trees = get_visible_trees(sample);

        assert_eq!(visible_trees, 21);
    }

    #[test]
    fn get_highest_scenic_score_should_work() {
        let sample_one = Vec::from([
            (0, String::from("30373")),
            (1, String::from("25512")),
            (2, String::from("65332")),
            (3, String::from("33549")),
            (4, String::from("35390")),
        ]);

        let highest_score_one = get_highest_scenic_score(&sample_one);
        assert_eq!(highest_score_one, 8);

        let sample_two = Vec::from([
            (0, String::from("20121")),
            (1, String::from("21002")), // 6, 1, 1
            (2, String::from("00002")), // 1, 1, 1
            (3, String::from("11111")), // 2, 3, 3
            (4, String::from("00222")),
        ]);

        let highest_score_two = get_highest_scenic_score(&sample_two);
        assert_eq!(highest_score_two, 6);

        let sample_three = Vec::from([
            (0, String::from("21340")),
            (1, String::from("22332")), // 1, 4, 1
            (2, String::from("14143")), // 8, 1, 8
            (3, String::from("43321")), // 1, 4, 1
            (4, String::from("34013")),
        ]);

        let highest_score_three = get_highest_scenic_score(&sample_three);
        assert_eq!(highest_score_three, 8);
    }
}

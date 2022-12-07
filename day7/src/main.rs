use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let mut current_dir = Vec::new();
    let mut directories: HashMap<String, usize> = HashMap::from([(String::from("/"), 0)]);

    for line in input.lines() {
        if line.starts_with("$ cd") {
            let dir = line.split_whitespace().last().unwrap();

            if dir.ne("..") {
                current_dir.push(dir);
                let current_dir_str = current_dir.join("");
                directories.insert(current_dir_str, 0);
            } else {
                current_dir.pop();
            }
        } else if line.chars().next().unwrap().is_numeric() {
            let current_dir_str = current_dir.join("");
            let file_size = line
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            directories
                .clone()
                .keys()
                .filter(|key| current_dir_str.starts_with(*key))
                .for_each(|key| {
                    if let Some(current_size) = directories.get_mut(key) {
                        *current_size += file_size;
                    }
                });
        }
    }

    let mut sum: usize = 0;
    let total_size = directories.get("/").unwrap();
    let space_needed = 30_000_000 - (70_000_000 - total_size);

    let mut sizes = directories
        .iter()
        .map(|(_, size)| *size)
        .collect::<Vec<usize>>();
    sizes.sort();

    let mut size_to_be_cleared = 0;

    for size in sizes.iter() {
        if *size <= 100_000 {
            sum += size;
        }

        if *size >= space_needed && size_to_be_cleared == 0 {
            size_to_be_cleared = *size;
        }
    }

    println!("Result is {sum}");
    println!("Total size is {}", total_size);
    println!("Space needed {}", space_needed);
    println!("Space to be cleared {}", size_to_be_cleared);
}

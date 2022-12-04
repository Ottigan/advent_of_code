use std::{collections::HashMap, fs};

fn main() {
    let priorities = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
        "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    ]
    .iter()
    .enumerate()
    .map(|(i, letter)| (String::from(*letter), (i + 1) as i32))
    .collect::<HashMap<String, i32>>();

    let mut sum = 0;
    let mut badge_sum = 0;

    let input = fs::read_to_string("src/input.txt").unwrap();
    let mut groups: Vec<Vec<&str>> = Vec::new();
    let rucksacks = input.lines();

    let mut group: Vec<&str> = Vec::new();

    for r in rucksacks {
        if group.len() == 3 {
            groups.push(group);
            group = Vec::new();
        }

        group.push(r);
    }

    groups.push(group);

    groups.iter().for_each(|g| {
        match g[0]
            .chars()
            .find(|item| g[1..].iter().all(|r| r.contains(*item)))
        {
            Some(item) => {
                badge_sum += priorities[&item.to_string()];
            }
            None => println!("Error!"),
        }

        for r in g {
            let rucksack_length = r.len();
            let compartments = r.split_at(rucksack_length / 2);

            let mut duplicates: Vec<char> = Vec::new();

            for i in compartments.0.chars() {
                for j in compartments.1.chars() {
                    if i == j {
                        if !duplicates.iter().find(|&&v| v == i).is_some() {
                            let key = i.to_string();
                            let value = priorities[&key];

                            duplicates.push(i);
                            sum += value;
                        }
                    }
                }
            }
        }
    });

    println!("Sum is {sum}");
    println!("Badge sum is {badge_sum}");
}

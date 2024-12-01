use std::{fs, ops::Sub};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let (raw_stacks, procedure) = input.split_once("\n\n").unwrap();

    // Get stack count to create 2D Vector representation
    let stack_count = raw_stacks
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap();

    // Stacks as columns
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_count as usize];

    // Fill stacks with initial values
    let stack_lines: Vec<&str> = raw_stacks.split('\n').collect();

    stack_lines
        .iter()
        .rev()
        .take(stack_lines.len() - 1)
        .map(|line| {
            line.chars()
                .skip(1)
                .take(line.len() - 2)
                .step_by(4)
                .enumerate()
        })
        .for_each(|line| {
            line.for_each(|(index, value)| {
                if !value.to_string().trim().is_empty() {
                    stacks[index].push(value)
                }
            })
        });

    // Parse rearrangement procedures
    let steps = procedure.lines().map(|line| {
        let container = Vec::new();
        let mut step = line
            .split(' ')
            .fold(container, |mut acc, frag| {
                if let Ok(value) = frag.parse::<u32>() {
                    acc.push(value)
                }

                acc
            })
            .into_iter();

        (
            step.next().unwrap(),
            step.next().unwrap(),
            step.next().unwrap(),
        )
    });

    // Execute rearrangement procedure
    for (amount, from, to) in steps {
        // Part 1
        // for _ in 0..amount {
        //     let crate_value = stacks[(from.sub(1)) as usize].pop().unwrap();
        //     stacks[to.sub(1) as usize].push(crate_value);
        // }

        // Part 2
        let stack = &mut stacks[(from.sub(1)) as usize];
        stack
            .split_off(stack.len() - (amount as usize))
            .iter()
            .for_each(|crate_value| stacks[to.sub(1) as usize].push(*crate_value))
    }

    // Retrieve top crate from each stack
    let result = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    println!("{result}");
}

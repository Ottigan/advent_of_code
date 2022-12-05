use std::{fs, ops::Sub};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let (raw_stacks, procedure) = input.split_once("\n\n").unwrap();
    let stack_lines: Vec<&str> = raw_stacks
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect();

    let num_of_stacks = stack_lines
        .first()
        .unwrap()
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap();

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_of_stacks as usize];

    stack_lines
        .iter()
        .skip(1)
        .map(|line| {
            line.chars()
                .skip(1)
                .take(line.len() - 2)
                .step_by(4)
                .map(|v| v)
                .enumerate()
        })
        .for_each(|line| {
            line.for_each(|(index, value)| {
                if !value.to_string().trim().is_empty() {
                    stacks[index].push(value)
                }
            })
        });

    let actions = procedure.lines().map(|line| {
        let container = Vec::new();
        let mut step = line
            .split(" ")
            .fold(container, |mut acc, frag| {
                match frag.parse::<u32>() {
                    Ok(value) => acc.push(value),
                    _ => (),
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

    for (amount, from, to) in actions {
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

    let result = stacks
        .iter()
        .map(|stack| stack.get(stack.len() - 1).unwrap())
        .collect::<String>();

    println!("{result}")
}

use itertools::Itertools;
use std::{collections::VecDeque, fs};

const SIGNAL_PATTERN: usize = 4;
const MESSAGE_PATTERN: usize = 14;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    find_start(&input, SIGNAL_PATTERN, "Signal");
    find_start(&input, MESSAGE_PATTERN, "Message");
}

fn find_start(input: &String, pattern: usize, name: &str) {
    let mut container = VecDeque::<char>::new();

    for (index, c) in input.chars().enumerate() {
        if container.len() < pattern {
            container.push_back(c)
        } else if container.iter().unique().count() != pattern {
            container.pop_front();
            container.push_back(c)
        } else {
            println!("{name} at {index}");
            break;
        }
    }
}

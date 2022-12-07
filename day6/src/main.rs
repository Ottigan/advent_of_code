use itertools::Itertools;
use std::{collections::VecDeque, fs};

const SIGNAL_PATTERN: usize = 4;
const MESSAGE_PATTERN: usize = 14;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    find_start(&input, SIGNAL_PATTERN, "Signal");
    find_start(&input, MESSAGE_PATTERN, "Message");
}

fn find_start(input: &str, pattern: usize, name: &str) -> usize {
    let mut container = VecDeque::<char>::new();

    for (index, c) in input.chars().enumerate() {
        if container.len() < pattern {
            container.push_back(c)
        } else if container.iter().unique().count() != pattern {
            container.pop_front();
            container.push_back(c)
        } else {
            println!("{name} at {index}");
            return index;
        }
    }

    panic!("No {name}!");
}

#[cfg(test)]
mod tests {
    use crate::{find_start, MESSAGE_PATTERN, SIGNAL_PATTERN};

    #[test]
    fn should_find_signal() {
        let sample = String::from("jkljlmnop");
        let index = find_start(&sample, SIGNAL_PATTERN, "Signal");

        assert_eq!(index, 7);
    }

    #[test]
    fn should_find_message() {
        let sample = String::from("jkljlmnoppasdfghjklzxcvdklfjdsjhhjkgh");
        let index = find_start(&sample, MESSAGE_PATTERN, "Message");

        assert_eq!(index, 23);
    }

    #[test]
    #[should_panic]
    fn should_panic_if_too_few_chars() {
        let sample = String::from("Foo");
        find_start(&sample, SIGNAL_PATTERN, "Signal");
    }
}

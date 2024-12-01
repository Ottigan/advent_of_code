use std::fs;

fn main() {
    let result = fs::read_to_string("src/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let sections = line
                .split(['-', ','])
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let mut first_range = sections[0]..=sections[1];
            let second_range = sections[2]..=sections[3];

            // Part 1
            // (first_range.contains(&sections[2]) && first_range.contains(&sections[3]))
            //     || (second_range.contains(&sections[0]) && second_range.contains(&sections[1]))

            // Part 2
            first_range.any(|v| second_range.contains(&v))
        })
        .fold(0, |acc, valid| if valid { acc + 1 } else { acc });

    println!("{result}");
}

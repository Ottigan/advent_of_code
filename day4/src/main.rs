use std::fs;

fn main() {
    let result = fs::read_to_string("src/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            return line
                .replace(",", " ")
                .replace("-", " ")
                .split(" ")
                .map(|v| match v.parse::<i32>() {
                    Ok(result) => result,
                    Err(err) => panic!("{err}"),
                })
                .collect::<Vec<i32>>()
                .chunks_exact(4)
                .map(|chunk| {
                    let mut first_range = chunk[0]..=chunk[1];
                    let second_range = chunk[2]..=chunk[3];

                    // // Part 1
                    // if (first_range.contains(&chunk[2]) && first_range.contains(&chunk[3]))
                    //     || (second_range.contains(&chunk[0]) && second_range.contains(&chunk[1]))
                    // {
                    //     1
                    // } else {
                    //     0
                    // }

                    // Part 2contains
                    if first_range.any(|v| second_range.contains(&v)) {
                        1
                    } else {
                        0
                    }
                })
                .next()
                .unwrap();
        })
        .fold(0, |acc, v| acc + v);

    println!("{result}");
}

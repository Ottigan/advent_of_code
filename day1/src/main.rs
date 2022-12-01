use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").unwrap();

    let mut highest_amount_of_calories = 0;

    contents.split("\n\n").for_each(|line| {
        let calories_sum: i32 = line
            .split("\n")
            .map(|food| food.parse::<i32>().unwrap())
            .sum();

        if calories_sum > highest_amount_of_calories {
            highest_amount_of_calories = calories_sum;
        }
    });

    println!("Highest amount of calories: {highest_amount_of_calories}",);
}

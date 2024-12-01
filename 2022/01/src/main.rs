use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();

    let mut highest_amount_of_calories = 0;
    let mut second_highest_amount_of_calories = 0;
    let mut third_highest_amount_of_calories = 0;

    contents.split("\n\n").for_each(|line| {
        let calories_sum: i32 = line
            .split("\n")
            .map(|food| food.parse::<i32>().unwrap())
            .sum();

        if calories_sum > highest_amount_of_calories {
            third_highest_amount_of_calories = second_highest_amount_of_calories;
            second_highest_amount_of_calories = highest_amount_of_calories;
            highest_amount_of_calories = calories_sum;
        } else if calories_sum > second_highest_amount_of_calories {
            third_highest_amount_of_calories = second_highest_amount_of_calories;
            second_highest_amount_of_calories = calories_sum;
        } else if calories_sum > third_highest_amount_of_calories {
            third_highest_amount_of_calories = calories_sum;
        }
    });

    let sum_of_highest_calories = highest_amount_of_calories
        + second_highest_amount_of_calories
        + third_highest_amount_of_calories;

    println!("Highest amount of calories: {highest_amount_of_calories}",);
    println!("Second highest amount of calories: {second_highest_amount_of_calories}",);
    println!("Third highest amount of calories: {third_highest_amount_of_calories}",);
    println!("Sum of highest amount of calories: {sum_of_highest_calories}",);
}

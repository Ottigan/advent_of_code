use itertools::Itertools;
use std::{collections::HashMap, fs, str::Lines};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let instructions = input.lines();
    let mut x_history: HashMap<i32, i32> = HashMap::from([(1, 1)]);
    let mut crt_screen = [[' '; 40]; 6];

    register_instructions(instructions, &mut x_history, &mut crt_screen);
    get_signal_strength_sum(&x_history);
    display_crt(crt_screen);
}

fn register_instructions(
    instructions: Lines,
    x_history: &mut HashMap<i32, i32>,
    crt_screen: &mut [[char; 40]; 6],
) {
    instructions.for_each(|instruction| {
        let history = x_history.clone();
        let last_cycle = history.keys().copied().sorted().last().unwrap();
        let last_x_value = history.get(&last_cycle).unwrap();
        draw_pixel(x_history, crt_screen);
        x_history.insert(last_cycle + 1, *last_x_value);

        if let Some((_, action)) = instruction.split_once(' ') {
            let action = action.parse::<i32>().unwrap();
            let new_x = last_x_value + action;

            draw_pixel(x_history, crt_screen);
            x_history.insert(last_cycle + 2, new_x);
        };
    });
}

fn draw_pixel(x_history: &mut HashMap<i32, i32>, crt_screen: &mut [[char; 40]; 6]) {
    let last_cycle = x_history.keys().copied().sorted().last().unwrap();
    let last_x_value = x_history.get(&last_cycle).unwrap();
    let sprite_position = Vec::from([last_x_value - 1, *last_x_value, last_x_value + 1]);
    let current_display_row = ((last_cycle / 40) as f32).floor() as usize;
    let current_index = (last_cycle - 1) % 40;
    let pixel_content = if sprite_position.contains(&current_index) {
        '#'
    } else {
        '.'
    };

    if current_display_row < 6 {
        crt_screen[current_display_row][current_index as usize] = pixel_content;
    }
}

fn display_crt(image: [[char; 40]; 6]) {
    let output = image.map(|row| row.iter().join("")).iter().join("\n");
    println!("{output}");
}

fn get_signal_strength_sum(x_history: &HashMap<i32, i32>) {
    let signal_strength_sum = [20, 60, 100, 140, 180, 220]
        .map(|cycle| {
            let value = x_history.get(&cycle).unwrap();

            cycle * value
        })
        .iter()
        .sum::<i32>();

    println!("\nSignal strength sum: {signal_strength_sum}\n");
}

use std::fs;

const ROCK_SCORE: i32 = 1;
const PAPER_SCORE: i32 = 2;
const SCISSORS_SCORE: i32 = 3;
const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;
const LOSS_SCORE: i32 = 0;

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();

    let mut total_score = 0;

    let rounds = contents.split("\n");

    for round in rounds {
        let (opponent, player) = round.split_at(1);
        let result_score = get_expected_result_score(player.trim());
        let shape_score = get_needed_shape_score(opponent.trim(), result_score);

        total_score += result_score;
        total_score += shape_score;
    }

    println!("{total_score}");
}

fn get_expected_result_score(p: &str) -> i32 {
    if p == "X" {
        return LOSS_SCORE;
    } else if p == "Y" {
        return DRAW_SCORE;
    } else {
        return WIN_SCORE;
    }
}

fn get_needed_shape_score(o: &str, r: i32) -> i32 {
    match r {
        WIN_SCORE => {
            return if o == "A" {
                PAPER_SCORE
            } else if o == "B" {
                SCISSORS_SCORE
            } else {
                ROCK_SCORE
            }
        }
        LOSS_SCORE => {
            return if o == "A" {
                SCISSORS_SCORE
            } else if o == "B" {
                ROCK_SCORE
            } else {
                PAPER_SCORE
            }
        }
        _ => {
            return if o == "A" {
                ROCK_SCORE
            } else if o == "B" {
                PAPER_SCORE
            } else {
                SCISSORS_SCORE
            }
        }
    }
}

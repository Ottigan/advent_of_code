use std::{collections::HashSet, fs};

#[derive(PartialEq, Eq, Debug)]
struct Motion(char, i32);

fn main() {
    let motions = get_motions(fs::read_to_string("src/input.txt").unwrap());

    let positions_visited = get_positions_visited(motions, 10);

    println!("Positions visited: {positions_visited}");
}

fn get_motions(string: String) -> Vec<Motion> {
    string
        .lines()
        .map(|line| {
            let (direction, distance) = line.trim().split_once(' ').unwrap();

            Motion(
                direction.parse::<char>().unwrap(),
                distance.parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

fn get_positions_visited(motions: Vec<Motion>, knot_count: i32) -> i32 {
    let mut visited_positions = HashSet::from([(String::from("0 0"))]);

    motions.iter().fold(
        vec![(0, 0); knot_count as usize],
        |positions, Motion(direction, distance)| {
            let (new_positions, tail_path) = parse_motions(*direction, *distance, positions);

            visited_positions.extend(
                tail_path
                    .iter()
                    .map(|path| format!("{} {}", path.0, path.1)),
            );

            new_positions
        },
    );

    visited_positions.len() as i32
}

fn parse_motions(
    direction: char,
    distance: i32,
    positions: Vec<(i32, i32)>,
) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let range = 0..distance;

    let mut tail_path: Vec<(i32, i32)> = Vec::from([*positions.last().unwrap()]);
    let new_positions = range.fold(positions, |knots, _| {
        let mut prev_knot: Option<(i32, i32)> = None;
        let new_knots = knots
            .iter()
            .map(|knot| {
                let new_knot = match prev_knot {
                    Some(prev) => move_knot(prev, *knot),
                    None => match direction {
                        'U' => (knot.0, knot.1 + 1),
                        'R' => (knot.0 + 1, knot.1),
                        'D' => (knot.0, knot.1 - 1),
                        'L' => (knot.0 - 1, knot.1),
                        _ => panic!("Such direction does not exist!"),
                    },
                };

                prev_knot = Some(new_knot);
                new_knot
            })
            .collect::<Vec<(i32, i32)>>();

        tail_path.push(*new_knots.last().unwrap());
        new_knots
    });

    (new_positions, tail_path)
}

fn move_knot(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let x_diff = (head.0 - tail.0).abs();
    let y_diff = (head.1 - tail.1).abs();
    let adjusted_x = if head.0 > tail.0 {
        head.0 - 1
    } else {
        head.0 + 1
    };
    let adjusted_y = if head.1 > tail.1 {
        head.1 - 1
    } else {
        head.1 + 1
    };

    if x_diff == 2 && y_diff == 2 {
        (adjusted_x, adjusted_y)
    } else if x_diff == 2 {
        (adjusted_x, head.1)
    } else if y_diff == 2 {
        (head.0, adjusted_y)
    } else {
        tail
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{get_motions, get_positions_visited, parse_motions, Motion};

    #[test]
    fn parse_motions_should_work_moving_up() {
        let direction = 'U';
        let distance = 2;
        let positions = Vec::from([(-5, -5), (-6, -5)]);
        let expected_tail_path = Vec::from([(-6, -5), (-6, -5), (-5, -4)]);
        let expected_positions = Vec::from([(-5, -3), (-5, -4)]);
        let expected = (expected_positions, expected_tail_path);

        assert_eq!(parse_motions(direction, distance, positions), expected);
    }

    #[test]
    fn parse_motions_should_work_moving_right() {
        let direction = 'R';
        let distance = 2;
        let positions = Vec::from([(-5, -3), (-5, -2)]);
        let expected_tail_path = Vec::from([(-5, -2), (-5, -2), (-4, -3)]);
        let expected_positions = Vec::from([(-3, -3), (-4, -3)]);
        let expected = (expected_positions, expected_tail_path);

        assert_eq!(parse_motions(direction, distance, positions), expected);
    }

    #[test]
    fn parse_motions_should_work_moving_down() {
        let direction = 'D';
        let distance = 2;
        let positions = Vec::from([(-5, -5), (-6, -5)]);
        let expected_tail_path = Vec::from([(-6, -5), (-6, -5), (-5, -6)]);
        let expected_positions = Vec::from([(-5, -7), (-5, -6)]);
        let expected = (expected_positions, expected_tail_path);

        assert_eq!(parse_motions(direction, distance, positions), expected);
    }

    #[test]
    fn parse_motions_should_work_moving_left() {
        let direction = 'L';
        let distance = 2;
        let positions = Vec::from([(-5, -3), (-5, -2)]);
        let expected_tail_path = Vec::from([(-5, -2), (-5, -2), (-6, -3)]);
        let expected_positions = Vec::from([(-7, -3), (-6, -3)]);
        let expected = (expected_positions, expected_tail_path);

        assert_eq!(parse_motions(direction, distance, positions), expected);
    }

    #[test]
    fn get_motions_should_work() {
        let input = String::from(
            "D 1
             L 2
             D 2
             L 1
             R 1
             D 1
             R 2
             U 1",
        );

        let expected = Vec::from([
            Motion('D', 1),
            Motion('L', 2),
            Motion('D', 2),
            Motion('L', 1),
            Motion('R', 1),
            Motion('D', 1),
            Motion('R', 2),
            Motion('U', 1),
        ]);

        assert_eq!(get_motions(input), expected);
    }

    #[test]
    fn get_positions_visited_part_one_should_work() {
        let input = Vec::from([
            Motion('R', 4),
            Motion('U', 4),
            Motion('L', 3),
            Motion('D', 1),
            Motion('R', 4),
            Motion('D', 1),
            Motion('L', 5),
            Motion('R', 2),
        ]);

        assert_eq!(get_positions_visited(input, 2), 13);
    }

    #[test]
    fn get_positions_visited_part_two_sampple_should_work() {
        let input = Vec::from([
            Motion('R', 5),
            Motion('U', 8),
            Motion('L', 8),
            Motion('D', 3),
            Motion('R', 17),
            Motion('D', 10),
            Motion('L', 25),
            Motion('U', 20),
        ]);

        assert_eq!(get_positions_visited(input, 10), 36);
    }

    #[test]
    fn get_positions_visited_part_two_should_work() {
        let motions = get_motions(fs::read_to_string("src/input.txt").unwrap());

        assert_eq!(get_positions_visited(motions, 10), 2793);
    }
}

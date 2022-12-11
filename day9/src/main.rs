use std::{collections::HashSet, fs};

#[derive(PartialEq, Eq, Debug)]
struct Motion(char, i32);

fn main() {
    let motions = get_motions(fs::read_to_string("src/input.txt").unwrap());

    let positions_visited = get_positions_visited(motions, 2);

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
    let new_positions = match direction {
        'U' => range.fold(positions, |knots, _| {
            let mut prev_knot: Option<(i32, i32)> = None;
            let new_knots = knots
                .iter()
                .map(|knot| {
                    let new_knot = match prev_knot {
                        Some(prev) => {
                            let x_diff = (prev.0 - knot.0).abs();
                            let y_diff = (prev.1 - knot.1).abs();
                            let adjusted_x = if prev.0 > knot.0 {
                                prev.0 - 1
                            } else {
                                prev.0 + 1
                            };

                            if x_diff == 2 && y_diff == 1 {
                                (adjusted_x, prev.1)
                            } else if x_diff == 2 && y_diff == 2 {
                                (adjusted_x, prev.1 - 1)
                            } else if y_diff == 2 {
                                (prev.0, prev.1 - 1)
                            } else {
                                *knot
                            }
                        }
                        None => (knot.0, knot.1 + 1),
                    };

                    prev_knot = Some(new_knot);
                    new_knot
                })
                .collect::<Vec<(i32, i32)>>();

            tail_path.push(*new_knots.last().unwrap());
            new_knots
        }),
        'R' => range.fold(positions, |knots, _| {
            let mut prev_knot: Option<(i32, i32)> = None;
            let new_knots = knots
                .iter()
                .map(|knot| {
                    let new_knot = match prev_knot {
                        Some(prev) => {
                            let x_diff = (prev.0 - knot.0).abs();
                            let y_diff = (prev.1 - knot.1).abs();
                            let adjusted_y = if prev.1 > knot.1 {
                                prev.1 - 1
                            } else {
                                prev.1 + 1
                            };
                            let adjusted_x = if prev.0 > knot.0 {
                                prev.0 - 1
                            } else {
                                prev.0 + 1
                            };

                            if x_diff == 2 && y_diff == 2 {
                                (adjusted_x, adjusted_y)
                            } else if (x_diff == 1 && y_diff == 2) || y_diff == 2 {
                                (prev.0, adjusted_y)
                            } else if x_diff == 2 {
                                (prev.0 - 1, prev.1)
                            } else {
                                *knot
                            }
                        }
                        None => (knot.0 + 1, knot.1),
                    };

                    prev_knot = Some(new_knot);
                    new_knot
                })
                .collect::<Vec<(i32, i32)>>();

            tail_path.push(*new_knots.last().unwrap());
            new_knots
        }),
        'D' => range.fold(positions, |knots, _| {
            let mut prev_knot: Option<(i32, i32)> = None;
            let new_knots = knots
                .iter()
                .map(|knot| {
                    let new_knot = match prev_knot {
                        Some(prev) => {
                            let x_diff = (prev.0 - knot.0).abs();
                            let y_diff = (prev.1 - knot.1).abs();
                            let adjusted_x = if prev.0 > knot.0 {
                                prev.0 - 1
                            } else {
                                prev.0 + 1
                            };

                            if x_diff == 2 && y_diff == 1 {
                                (adjusted_x, prev.1)
                            } else if x_diff == 2 && y_diff == 2 {
                                (adjusted_x, prev.1 + 1)
                            } else if y_diff == 2 {
                                (prev.0, prev.1 + 1)
                            } else {
                                *knot
                            }
                        }
                        None => (knot.0, knot.1 - 1),
                    };

                    prev_knot = Some(new_knot);
                    new_knot
                })
                .collect::<Vec<(i32, i32)>>();

            tail_path.push(*new_knots.last().unwrap());
            new_knots
        }),
        'L' => range.fold(positions, |knots, _| {
            let mut prev_knot: Option<(i32, i32)> = None;
            let new_knots = knots
                .iter()
                .map(|knot| {
                    let new_knot = match prev_knot {
                        Some(prev) => {
                            let x_diff = (prev.0 - knot.0).abs();
                            let y_diff = (prev.1 - knot.1).abs();
                            let adjusted_y = if prev.1 > knot.1 {
                                prev.1 - 1
                            } else {
                                prev.1 + 1
                            };
                            let adjusted_x = if prev.0 > knot.0 {
                                prev.0 - 1
                            } else {
                                prev.0 + 1
                            };

                            if x_diff == 2 && y_diff == 2 {
                                (adjusted_x, adjusted_y)
                            } else if (x_diff == 1 && y_diff == 2) || y_diff == 2 {
                                (prev.0, adjusted_y)
                            } else if x_diff == 2 {
                                (prev.0 + 1, prev.1)
                            } else {
                                *knot
                            }
                        }
                        None => (knot.0 - 1, knot.1),
                    };

                    prev_knot = Some(new_knot);
                    new_knot
                })
                .collect::<Vec<(i32, i32)>>();

            tail_path.push(*new_knots.last().unwrap());
            new_knots
        }),
        _ => panic!("Such direction does not exist!"),
    };

    (new_positions, tail_path)
}

#[cfg(test)]
mod tests {
    use crate::{get_motions, get_positions_visited, parse_motions, Motion};

    #[test]
    fn parse_motions_should_work_moving_up() {
        let direction = 'U';
        let distance = 2;
        let positions = Vec::from([(0, 0), (1, -1)]);
        let expected_tail_path = Vec::from([(1, -1), (0, 0), (0, 1)]);
        let expected_positions = Vec::from([(0, 2), (0, 1)]);
        let expected = (expected_positions, expected_tail_path);

        assert_eq!(parse_motions(direction, distance, positions), expected);
    }

    #[test]
    fn parse_motions_should_work_moving_right() {
        let direction = 'R';
        let distance = 2;
        let positions = Vec::from([(-5, -3), (-4, -3)]);
        let expected_tail_path = Vec::from([(-4, -3), (-4, -3), (-4, -3)]);
        let expected_positions = Vec::from([(-3, -3), (-4, -3)]);
        let expected = (expected_positions, expected_tail_path);

        assert_eq!(parse_motions(direction, distance, positions), expected);
    }

    #[test]
    fn parse_motions_should_work_moving_down() {
        let direction = 'D';
        let distance = 2;
        let positions = Vec::from([(-5, -3), (-4, -3)]);
        let expected_tail_path = Vec::from([(-4, -3), (-4, -3), (-5, -4)]);
        let expected_positions = Vec::from([(-5, -5), (-5, -4)]);
        let expected = (expected_positions, expected_tail_path);

        assert_eq!(parse_motions(direction, distance, positions), expected);
    }

    #[test]
    fn parse_motions_should_work_moving_left() {
        let direction = 'L';
        let distance = 2;
        let positions = Vec::from([(-5, -3), (-4, -3)]);
        let expected_tail_path = Vec::from([(-4, -3), (-5, -3), (-6, -3)]);
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
    fn get_positions_visited_part_two_should_work() {
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

        assert_eq!(get_positions_visited(input, 10), 1);
    }

    #[test]
    fn get_positions_visited_part_two_bigger_should_work() {
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
}

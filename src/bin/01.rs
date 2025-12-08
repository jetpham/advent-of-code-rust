use regex::Regex;

advent_of_code::solution!(1);

type DialRotation = u64;

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    rotations: DialRotation,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<u64> {
    const DIAL_START: DialRotation = 50;
    Some(
        Regex::new(r"([RL])(\d+)")
            .unwrap()
            .captures_iter(input)
            .map(|cap| {
                let dir_char = cap.get(1).unwrap().as_str();
                let direction = match dir_char {
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => unreachable!("Regex ensures this is R or L"),
                };

                let rot_str = cap.get(2).unwrap().as_str();
                let rotations = rot_str
                    .parse::<DialRotation>()
                    .expect("Number too big for u16");

                Rotation {
                    direction,
                    rotations,
                }
            })
            .fold((0, DIAL_START), |(zeros, dial_rotation), rotation| {
                let new_rotation = match rotation.direction {
                    Direction::Left => (dial_rotation as i32 - rotation.rotations as i32)
                        .rem_euclid(100) as DialRotation,
                    Direction::Right => (dial_rotation as i32 + rotation.rotations as i32)
                        .rem_euclid(100) as DialRotation,
                };
                if new_rotation == 0 {
                    (zeros + 1, new_rotation)
                } else {
                    (zeros, new_rotation)
                }
            })
            .0,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    const DIAL_START: DialRotation = 50;
    Some(
        Regex::new(r"([RL])(\d+)")
            .unwrap()
            .captures_iter(input)
            .map(|cap| {
                let dir_char = cap.get(1).unwrap().as_str();
                let direction = match dir_char {
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => unreachable!("Regex ensures this is R or L"),
                };

                let rot_str = cap.get(2).unwrap().as_str();
                let rotations = rot_str
                    .parse::<DialRotation>()
                    .expect("Number too big for u16");

                Rotation {
                    direction,
                    rotations,
                }
            })
            .fold((0, DIAL_START), |(zeros, dial_rotation), rotation| {
                let amount = rotation.rotations as i64;
                let current = dial_rotation as i64;

                let (dist_to_zero, new_pos) = match rotation.direction {
                    Direction::Left => {
                        let dist = if current == 0 { 100 } else { current };
                        let next = (current - amount).rem_euclid(100);
                        (dist, next)
                    }
                    Direction::Right => {
                        let dist = if current == 0 { 100 } else { 100 - current };
                        let next = (current + amount).rem_euclid(100);
                        (dist, next)
                    }
                };

                let hits = if amount >= dist_to_zero {
                    1 + (amount - dist_to_zero) / 100
                } else {
                    0
                };

                (zeros + hits as u64, new_pos as DialRotation)
            })
            .0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

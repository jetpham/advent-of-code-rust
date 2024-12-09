#![feature(iter_map_windows)]
advent_of_code::solution!(2);
use itertools::Itertools;
use std::iter::once;

fn dampened_valid(report: &Vec<i32>) -> bool {
    (0..report.len())
        .map(|i| {
            let mut new_report = report.clone();
            new_report.remove(i);
            new_report
        })
        .chain(once(report.clone()))
        .any(|x| valid(&x))
}

fn parse_line(input: &str) -> Vec<i32> {
    input
        .split_ascii_whitespace()
        .map(|number| number.parse().unwrap())
        .collect()
}

fn valid(report: &Vec<i32>) -> bool {
    (report.iter().is_sorted() || report.iter().rev().is_sorted())
        && report
            .iter()
            .cloned()
            .tuple_windows()
            .map(|(x, y)| x.abs_diff(y))
            .all(|x| (1..=3).contains(&x))
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|report| valid(&parse_line(report)))
            .count()
            .try_into()
            .ok()?,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|report| dampened_valid(&parse_line(report)))
            .count()
            .try_into()
            .ok()?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}

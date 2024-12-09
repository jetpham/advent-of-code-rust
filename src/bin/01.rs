advent_of_code::solution!(1);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    dbg!(input);
    let mut first: [i32; 1000] = [0; 1000];
    let mut second: [i32; 1000] = [0; 1000];
    for (index, line) in input.split('\n').enumerate() {
        let mut iter = line.split_whitespace();
        if let Some(str_first) = iter.next() {
            first[index] = str_first.parse::<i32>().unwrap();
        }
        if let Some(str_second) = iter.next() {
            second[index] = str_second.parse::<i32>().unwrap();
        }
    }
    first.sort();
    second.sort();
    let mut total = 0;
    for (first, second) in first.iter().zip(second.iter()) {
        total += first.abs_diff(*second);
    }
    Some(total.try_into().ok()?)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut first: [i32; 1000] = [0; 1000];
    let mut second: HashMap<i32, i32> = HashMap::new();
    for (index, line) in input.split('\n').enumerate() {
        let mut iter = line.split_whitespace();
        if let Some(str_first) = iter.next() {
            first[index] = str_first.parse::<i32>().unwrap();
        }
        if let Some(str_first) = iter.next() {
            *second.entry(str_first.parse::<i32>().unwrap()).or_insert(0) += 1;
        }
    }
    let mut total = 0;
    for i in first {
        if let Some(&count) = second.get(&i) {
            total += i * count;
        } else {
            continue;
        }
    }
    Some(total.try_into().ok()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

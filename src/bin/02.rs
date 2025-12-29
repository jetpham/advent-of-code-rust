use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        Regex::new(r"(\d+)-(\d+)")
            .unwrap()
            .captures_iter(input)
            .map(|cap| {
                (
                    cap[1].parse::<u64>().unwrap(),
                    cap[2].parse::<u64>().unwrap(),
                )
            })
            .fold(0, |acc, (first, second)| {
                (first..=second).fold(acc, |acc, n| {
                    let len: u64 = (n.checked_ilog10().unwrap_or(0) + 1).into();
                    if len % 2 == 0 {
                        let pow = 10_u64.pow((len / 2).try_into().unwrap());
                        let small = n % pow;
                        let big = (n - small) / pow;
                        if big == small { acc + n } else { acc }
                    } else {
                        acc
                    }
                })
            }),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        Regex::new(r"(\d+)-(\d+)")
            .unwrap()
            .captures_iter(input)
            .map(|cap| {
                (
                    cap[1].parse::<u64>().unwrap(),
                    cap[2].parse::<u64>().unwrap(),
                )
            })
            .fold(0, |acc, (start, end)| {
                acc + (start..=end)
                    .filter(|&n| {
                        let len = n.checked_ilog10().unwrap_or(0) + 1;
                        (1..=len / 2).any(|k| {
                            len % k == 0
                                && (0..len / k).fold(0, |rebuilt, i| {
                                    rebuilt + (n % 10u64.pow(k)) * 10u64.pow(k * i)
                                }) == n
                        })
                    })
                    .sum::<u64>()
            }),
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

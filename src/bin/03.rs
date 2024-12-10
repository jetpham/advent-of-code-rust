use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1},
    combinator::map,
    multi::{fold_many0, many0},
    sequence::{delimited, preceded, tuple},
    IResult,
};

advent_of_code::solution!(3);

#[derive(Debug)]
struct Expression {
    first: u32,
    second: u32,
}
fn parse_expressions(input: &str) -> IResult<&str, Vec<Expression>> {
    fold_many0(
        preceded(
            take_until("mul("),
            alt((
                delimited(
                    tag("mul("),
                    map(
                        tuple((
                            map(digit1, |s: &str| s.parse::<u32>().unwrap()),
                            preceded(char(','), map(digit1, |s: &str| s.parse::<u32>().unwrap())),
                        )),
                        |(first, second)| Some(Expression { first, second }),
                    ),
                    char(')'),
                ),
                map(tag("mul("), |_| None),
            )),
        ),
        Vec::new,
        |mut acc, item| {
            if let Some(expr) = item {
                acc.push(expr);
            }
            acc
        },
    )(input)
}

// Function to parse and return parts of the input not matching "don't()...do()"
fn remove_do_dont(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (remaining, matches) = many0(tuple((
        take_until("don't()"),
        tag("don't()"),
        take_until("do()"),
        tag("do()"),
    )))(input)?;

    let (mut good, bad) = matches.into_iter().fold(
        (Vec::new(), Vec::new()),
        |(mut good, mut bad), (before, _, between, _)| {
            good.push(before);
            bad.push(between);
            (good, bad)
        },
    );
    if !remaining.is_empty() {
        good.push(remaining);
    }

    Ok((remaining, (good, bad)))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_gibberish, expressions) = parse_expressions(input).unwrap();
    Some(expressions.iter().map(|x| x.first * x.second).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, matches) = remove_do_dont(input).unwrap();
    let (mul, _) = matches;
    let (_gibberish, expressions) = parse_expressions(&mul.join("")).unwrap();
    Some(expressions.iter().map(|x| x.first * x.second).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}

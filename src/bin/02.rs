use std::cmp::min;

use nom::{
    bytes::complete::tag,
    character::{self, complete::newline},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

advent_of_code::solution!(2);

type Dimensions = (u32, u32, u32);

fn parse_size(input: &str) -> IResult<&str, Dimensions> {
    tuple((
        character::complete::u32,
        preceded(tag("x"), character::complete::u32),
        preceded(tag("x"), character::complete::u32),
    ))(input)
}

fn parse_sizes(input: &str) -> Vec<Dimensions> {
    let (_, sizes) = separated_list1(newline, parse_size)(input).expect("should parse");

    sizes
}

fn calculate_required_paper(size: Dimensions) -> u32 {
    let (x, y, z) = size;

    let s1 = x * y;
    let s2 = x * z;
    let s3 = y * z;

    let extra = min(s1, min(s2, s3));

    2 * s1 + 2 * s2 + 2 * s3 + extra
}

fn calculate_required_ribbon(size: Dimensions) -> u32 {
    let (x, y, z) = size;

    let p1 = 2 * x + 2 * y;
    let p2 = 2 * x + 2 * z;
    let p3 = 2 * y + 2 * z;

    let ribbon_length = min(p1, min(p2, p3));
    let bow_length = x * y * z;

    ribbon_length + bow_length
}

pub fn part_one(input: &str) -> Option<u32> {
    let sizes = parse_sizes(input);

    Some(sizes.into_iter().map(calculate_required_paper).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let sizes = parse_sizes(input);

    Some(sizes.into_iter().map(calculate_required_ribbon).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(101));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}

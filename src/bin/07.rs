use std::collections::HashMap;

use lazy_static::lazy_static;
use memoize::memoize;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, not_line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(7);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Source {
    And(Input, Input),
    Or(Input, Input),
    Not(Input),
    LShift(Input, Input),
    RShift(Input, Input),
    Signal(Input),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Input {
    Constant(u16),
    Wire(String),
}

lazy_static! {
    static ref CONNECTIONS1: HashMap<String, Source> =
        parse_input(&advent_of_code::template::read_file("inputs", DAY,));
    static ref CONNECTIONS2: HashMap<String, Source> = {
        let mut connections = parse_input(&advent_of_code::template::read_file("inputs", DAY));
        *connections.get_mut("b").unwrap() = Source::Signal(Input::Constant(3176));
        connections
    };
}

fn parse_source_input(input: &str) -> IResult<&str, Input> {
    let (input, source_input) = alt((alpha1, digit1))(input)?;

    if source_input.chars().next().unwrap().is_alphabetic() {
        Ok((input, Input::Wire(source_input.to_string())))
    } else {
        Ok((input, Input::Constant(source_input.parse().unwrap())))
    }
}

fn parse_and(input: &str) -> IResult<&str, Source> {
    let (input, (left, right)) =
        separated_pair(parse_source_input, tag(" AND "), parse_source_input)(input)?;

    Ok((input, Source::And(left, right)))
}

fn parse_or(input: &str) -> IResult<&str, Source> {
    let (input, (left, right)) =
        separated_pair(parse_source_input, tag(" OR "), parse_source_input)(input)?;

    Ok((input, Source::Or(left, right)))
}

fn parse_not(input: &str) -> IResult<&str, Source> {
    let (input, _) = tag("NOT ")(input)?;
    let (input, right) = parse_source_input(input)?;

    Ok((input, Source::Not(right)))
}

fn parse_lshift(input: &str) -> IResult<&str, Source> {
    let (input, (left, right)) =
        separated_pair(parse_source_input, tag(" LSHIFT "), parse_source_input)(input)?;

    Ok((input, Source::LShift(left, right)))
}

fn parse_rshift(input: &str) -> IResult<&str, Source> {
    let (input, (left, right)) =
        separated_pair(parse_source_input, tag(" RSHIFT "), parse_source_input)(input)?;

    Ok((input, Source::RShift(left, right)))
}

fn parse_signal(input: &str) -> IResult<&str, Source> {
    let (input, source_input) = parse_source_input(input)?;

    Ok((input, Source::Signal(source_input)))
}

fn parse_source(input: &str) -> IResult<&str, Source> {
    alt((
        parse_and,
        parse_or,
        parse_not,
        parse_lshift,
        parse_rshift,
        parse_signal,
    ))(input)
}

fn parse_connection(input: &str) -> IResult<&str, (String, Source)> {
    let (input, source) = parse_source(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, destination) = not_line_ending(input)?;

    Ok((input, (destination.to_string(), source)))
}

fn parse_input(input: &str) -> HashMap<String, Source> {
    let (_, connections) = separated_list1(newline, parse_connection)(input).expect("should parse");

    connections.into_iter().collect()
}

#[memoize]
fn resolve_input(input: Input, part1: bool) -> Option<u16> {
    match input {
        Input::Constant(constant) => Some(constant),
        Input::Wire(wire) => resolve_connection(wire, part1),
    }
}

#[memoize]
fn resolve_connection(dest: String, part1: bool) -> Option<u16> {
    let source = if part1 {
        CONNECTIONS1.get(&dest)?
    } else {
        CONNECTIONS2.get(&dest)?
    };

    Some(match source {
        Source::And(left, right) => {
            resolve_input(left.clone(), part1)? & resolve_input(right.clone(), part1)?
        }
        Source::Or(left, right) => {
            resolve_input(left.clone(), part1)? | resolve_input(right.clone(), part1)?
        }
        Source::Not(right) => !resolve_input(right.clone(), part1)?,
        Source::LShift(left, right) => {
            resolve_input(left.clone(), part1)? << resolve_input(right.clone(), part1)?
        }
        Source::RShift(left, right) => {
            resolve_input(left.clone(), part1)? >> resolve_input(right.clone(), part1)?
        }
        Source::Signal(signal) => resolve_input(signal.clone(), part1)?,
    })
}

pub fn part_one(_: &str) -> Option<u16> {
    resolve_connection("a".to_string(), true)
}

pub fn part_two(_input: &str) -> Option<u16> {
    resolve_connection("a".to_string(), false)
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_part_one() {
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}

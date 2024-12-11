use grid_2d::{Coord, Grid, Size};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{self, complete::newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use num::Saturating;

advent_of_code::solution!(6);

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl From<&str> for Action {
    fn from(input: &str) -> Self {
        match input {
            "turn on" => Action::TurnOn,
            "turn off" => Action::TurnOff,
            "toggle" => Action::Toggle,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Command {
    action: Action,
    start_point: Coord,
    end_point: Coord,
}

impl Command {
    fn new(action: Action, start_point: Coord, end_point: Coord) -> Self {
        Self {
            action,
            start_point,
            end_point,
        }
    }
}

fn parse_point(input: &str) -> IResult<&str, Coord> {
    let (input, (x, y)) =
        separated_pair(character::complete::i32, tag(","), character::complete::i32)(input)?;

    Ok((input, Coord::new(x, y)))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, action) = alt((tag("turn on"), tag("turn off"), tag("toggle")))(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, start) = parse_point(input)?;
    let (input, _) = tag(" through ")(input)?;
    let (input, end) = parse_point(input)?;

    Ok((input, Command::new(action.into(), start, end)))
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(newline, parse_command)(input)
}

fn parse_input(input: &str) -> Vec<Command> {
    let (_, commands) = parse_commands(input).expect("should parse");

    commands
}

pub fn part_one(input: &str) -> Option<u32> {
    let commands = parse_input(input);
    let mut grid = Grid::new_copy(Size::new(1000, 1000), false);

    for command in commands {
        for y in command.start_point.y..=command.end_point.y {
            for x in command.start_point.x..=command.end_point.x {
                let cell = grid.get_mut(Coord::new(x, y)).expect("should exist");

                match command.action {
                    Action::TurnOn => *cell = true,
                    Action::TurnOff => *cell = false,
                    Action::Toggle => *cell = !*cell,
                }
            }
        }
    }

    Some(grid.iter().filter(|cell| **cell).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let commands = parse_input(input);
    let mut grid = Grid::new_copy(Size::new(1000, 1000), 0);

    for command in commands {
        for y in command.start_point.y..=command.end_point.y {
            for x in command.start_point.x..=command.end_point.x {
                let cell = grid.get_mut(Coord::new(x, y)).expect("should exist");

                match command.action {
                    Action::TurnOn => *cell += 1,
                    Action::TurnOff => *cell = cell.saturating_sub(1),
                    Action::Toggle => *cell += 2,
                }
            }
        }
    }

    Some(grid.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(998_996));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1_001_996));
    }
}

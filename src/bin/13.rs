use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline},
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(13);

type Person = String;
type Relationship = ((Person, Person), i32);
type Relationships = HashMap<(Person, Person), i32>;

fn parse_relationship(input: &str) -> IResult<&str, Relationship> {
    let (input, p1) = alpha1(input)?;
    let (input, _) = tag(" would ")(input)?;
    let (input, sign) = alpha1(input)?; // "gain" or "lose"
    let (input, _) = tag(" ")(input)?;
    let (input, happiness) = digit1(input)?;
    let (input, _) = tag(" happiness units by sitting next to ")(input)?;
    let (input, p2) = alpha1(input)?;
    let (input, _) = tag(".")(input)?;

    let sign = if sign == "lose" { -1 } else { 1 };
    let happiness: i32 = happiness.parse().expect("must parse");

    Ok((input, ((p1.to_string(), p2.to_string()), sign * happiness)))
}

fn parse_relationships(input: &str) -> IResult<&str, Vec<Relationship>> {
    separated_list1(newline, parse_relationship)(input)
}

fn parse_input(input: &str) -> Relationships {
    let (_, relationships_vec) = parse_relationships(input).expect("should parse");

    relationships_vec.into_iter().collect()
}

fn get_people(relationships: &Relationships) -> HashSet<Person> {
    let mut people = HashSet::new();

    for (p1, p2) in relationships.keys() {
        people.insert(p1.to_string());
        people.insert(p2.to_string());
    }

    people
}

fn find_max_happiness(relationships: &Relationships) -> i32 {
    let people: Vec<Person> = get_people(relationships).into_iter().collect();

    let mut max_happiness = 0;

    for permutation in people.iter().permutations(people.len()) {
        let mut happiness = 0;

        for (p1, p2) in permutation.iter().tuple_windows() {
            happiness += relationships
                .get(&(p1.to_string(), p2.to_string()))
                .expect("should exist");
            happiness += relationships
                .get(&(p2.to_string(), p1.to_string()))
                .expect("should exist");
        }

        // Handle first/last pair the tuple_windows won't cover
        let first = permutation.first().expect("should exist");
        let last = permutation.last().expect("should exist");
        happiness += relationships
            .get(&(first.to_string(), last.to_string()))
            .expect("should exist");
        happiness += relationships
            .get(&(last.to_string(), first.to_string()))
            .expect("should exist");

        max_happiness = max(max_happiness, happiness);
    }

    max_happiness
}

pub fn part_one(input: &str) -> Option<i32> {
    let relationships = parse_input(input);

    Some(find_max_happiness(&relationships))
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut relationships = parse_input(input);
    let people = get_people(&relationships);
    let me = "Me".to_string();

    for person in people {
        relationships.insert((me.clone(), person.to_string()), 0);
        relationships.insert((person.to_string(), me.clone()), 0);
    }

    Some(find_max_happiness(&relationships))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(330));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(286));
    }
}

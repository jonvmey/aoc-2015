use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::{
        self,
        complete::{alpha1, newline},
    },
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(9);

type Location = String;
type Distance = ((Location, Location), u32);
type DistanceMap = HashMap<(Location, Location), u32>;

fn parse_distance(input: &str) -> IResult<&str, Distance> {
    let (input, (l1, l2)) = separated_pair(alpha1, tag(" to "), alpha1)(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, distance) = character::complete::u32(input)?;

    Ok((input, ((l1.to_string(), l2.to_string()), distance)))
}

fn parse_distances(input: &str) -> IResult<&str, DistanceMap> {
    let (input, distances) = separated_list1(newline, parse_distance)(input)?;

    Ok((input, distances.into_iter().collect()))
}

fn parse_input(input: &str) -> DistanceMap {
    let (_, distances) = parse_distances(input).expect("should parse");

    distances
}

fn get_map_locations(map: &DistanceMap) -> HashSet<Location> {
    let mut locations = HashSet::new();

    for (l1, l2) in map.keys() {
        locations.insert(l1.clone());
        locations.insert(l2.clone());
    }

    locations
}

fn calculate_distance<F>(
    locations: &HashSet<Location>,
    map: &DistanceMap,
    compare: F,
    init: u32,
) -> u32
where
    F: Fn(u32, u32) -> u32,
{
    let mut distance = init;

    for permutation in locations.iter().permutations(locations.len()) {
        let mut d = 0;

        for (l1, l2) in permutation.into_iter().tuple_windows() {
            d += if let Some(d) = map.get(&(l1.clone(), l2.clone())) {
                *d
            } else if let Some(d) = map.get(&(l2.clone(), l1.clone())) {
                *d
            } else {
                panic!()
            };
        }

        distance = compare(distance, d);
    }

    distance
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let locations = get_map_locations(&map);

    Some(calculate_distance(&locations, &map, min, u32::MAX))
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let locations = get_map_locations(&map);

    Some(calculate_distance(&locations, &map, max, u32::MIN))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

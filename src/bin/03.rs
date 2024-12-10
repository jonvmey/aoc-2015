use std::collections::HashSet;

advent_of_code::solution!(3);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut location = Point::new(0, 0);
    let mut deliveries = HashSet::new();
    deliveries.insert(location);

    for c in input.chars() {
        match c {
            '^' => location.y += 1,
            'v' => location.y -= 1,
            '>' => location.x += 1,
            '<' => location.x -= 1,
            _ => panic!(),
        }

        deliveries.insert(location);
    }

    Some(deliveries.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut move_santa = true;
    let mut santa_location = Point::new(0, 0);
    let mut robot_location = Point::new(0, 0);

    let mut deliveries = HashSet::new();
    deliveries.insert(santa_location);

    for c in input.chars() {
        let location = if move_santa {
            &mut santa_location
        } else {
            &mut robot_location
        };

        match c {
            '^' => location.y += 1,
            'v' => location.y -= 1,
            '>' => location.x += 1,
            '<' => location.x -= 1,
            _ => panic!(),
        }

        deliveries.insert(*location);
        move_santa = !move_santa;
    }

    Some(deliveries.len() as u32)
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
        assert_eq!(result, Some(11));
    }
}

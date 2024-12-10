advent_of_code::solution!(1);

fn char_to_floor(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(input.chars().map(char_to_floor).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut floor = 0;

    for (index, c) in input.chars().enumerate() {
        floor += char_to_floor(c);

        if floor == -1 {
            return Some(index as i32 + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(-3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}

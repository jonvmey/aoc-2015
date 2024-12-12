use itertools::Itertools;

advent_of_code::solution!(11);

fn increment_character(c: char) -> char {
    match c {
        'a' => 'b',
        'b' => 'c',
        'c' => 'd',
        'd' => 'e',
        'e' => 'f',
        'f' => 'g',
        'g' => 'h',
        'h' | 'i' => 'j',
        'j' => 'k',
        'k' | 'l' => 'm',
        'm' => 'n',
        'n' | 'o' => 'p',
        'p' => 'q',
        'q' => 'r',
        'r' => 's',
        's' => 't',
        't' => 'u',
        'u' => 'v',
        'v' => 'w',
        'w' => 'x',
        'x' => 'y',
        'y' => 'z',
        'z' => 'a',
        _ => panic!(),
    }
}

fn allowed_character(c: char) -> bool {
    !matches!(c, 'l' | 'i' | 'o')
}

fn increment_password(old: String) -> String {
    let mut new = "".to_string();
    let mut done_incrementing = false;

    for mut c in old.chars().rev() {
        if !allowed_character(c) {
            // increment c and roll over all previous letters to 'a'
            c = increment_character(c);
            new = "a".repeat(new.len());
            done_incrementing = true;
        } else if !done_incrementing {
            c = increment_character(c);
        }

        new.push(c);

        if c != 'a' {
            done_incrementing = true;
        }
    }

    new.chars().rev().collect()
}

fn contains_straight(password: &str) -> bool {
    let straights = [
        "abc", "bcd", "cde", "def", "efg", "fgh", "pqr", "qrs", "rst", "stu", "tuv", "uvw", "vwx",
        "wxy", "xyz",
    ];

    straights.iter().any(|straight| password.contains(straight))
}

fn contains_two_pairs(password: &str) -> bool {
    let mut pair_locations = Vec::new();

    for (index, (first, second)) in password.chars().tuple_windows().enumerate() {
        if first == second {
            pair_locations.push(index);
        }
    }

    match pair_locations.len() {
        0 | 1 => false,
        2 => pair_locations[1] - pair_locations[0] >= 2,
        _ => true,
    }
}

fn no_disallowed_letters(password: &str) -> bool {
    !password.chars().any(|c| c == 'l' || c == 'i' || c == 'o')
}

fn is_valid(password: &str) -> bool {
    contains_straight(password) && contains_two_pairs(password) && no_disallowed_letters(password)
}

fn next_password(mut password: String) -> String {
    loop {
        password = increment_password(password);

        if is_valid(&password) {
            break password;
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    Some(next_password(input.to_string()))
}

pub fn part_two(input: &str) -> Option<String> {
    Some(next_password(next_password(input.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("ghjaabcc".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("ghjbbcdd".to_string()));
    }
}

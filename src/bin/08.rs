advent_of_code::solution!(8);

enum UnescapeState {
    Normal,
    Backslash,
    Hex,
}

fn escaping_length_change(s: &str) -> u32 {
    let mut count = 0;
    let mut state = UnescapeState::Normal;

    for c in s.chars() {
        match state {
            UnescapeState::Normal => {
                if c == '\\' {
                    state = UnescapeState::Backslash;
                } else {
                    count += 1;
                }
            }
            UnescapeState::Backslash => {
                if c == 'x' {
                    state = UnescapeState::Hex;
                } else {
                    state = UnescapeState::Normal;
                    count += 1;
                }
            }
            UnescapeState::Hex => {
                state = UnescapeState::Normal;
            }
        }
    }

    count -= 2; // Adjust for leading/trailing "'s

    s.len() as u32 - count
}

fn unescaping_length_change(s: &str) -> u32 {
    let mut count = 0;

    for c in s.chars() {
        match c {
            // Add an additional count for the '\' on character's needing escaping
            '"' => count += 2,
            '\\' => count += 2,
            _ => count += 1,
        }
    }

    count += 2; // Adjust for surrounding "'s

    count - s.len() as u32
}
// let mut count = 0;

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(escaping_length_change).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(unescaping_length_change).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}

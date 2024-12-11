use itertools::Itertools;

advent_of_code::solution!(5);

fn num_vowels(s: &str) -> u32 {
    s.chars()
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count() as u32
}

fn has_double_letters(s: &str) -> bool {
    s.chars().tuple_windows().any(|(c1, c2)| c1 == c2)
}

fn no_disallowed_strings(s: &str) -> bool {
    !(s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy"))
}

fn is_nice1(s: &str) -> bool {
    num_vowels(s) >= 3 && has_double_letters(s) && no_disallowed_strings(s)
}

fn repeated_letter_pair(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    for index in 0..chars.len() - 1 {
        let search = &chars[index..index + 2];

        for search_index in index + 2..chars.len() - 1 {
            if chars[search_index..search_index + 2] == *search {
                return true;
            }
        }
    }

    false
}

fn letter_repeats_after(s: &str) -> bool {
    s.chars().tuple_windows().any(|(c1, _, c3)| c1 == c3)
}

fn is_nice2(s: &str) -> bool {
    repeated_letter_pair(s) && letter_repeats_after(s)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().filter(|s| is_nice1(s)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().filter(|s| is_nice2(s)).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}

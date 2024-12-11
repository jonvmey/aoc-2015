use rle_vec::RleVec;

advent_of_code::solution!(10);

fn look_and_say(input: String) -> String {
    let mut output = "".to_string();
    let rle: RleVec<char> = input.chars().collect();

    for run in rle.runs() {
        output.push_str(&run.len.to_string());
        output.push(*run.value);
    }

    output
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut s = input.to_string();

    for _ in 0..40 {
        s = look_and_say(s);
    }

    Some(s.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut s = input.to_string();

    for _ in 0..50 {
        s = look_and_say(s);
    }

    Some(s.len() as u32)
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

use json::JsonValue;

advent_of_code::solution!(12);

fn sum_numbers(value: &JsonValue) -> i32 {
    match value {
        JsonValue::Number(num) => Into::<f64>::into(*num) as i32,
        JsonValue::Object(obj) => obj.iter().map(|(_, value)| sum_numbers(value)).sum(),

        JsonValue::Array(arr) => arr.iter().map(sum_numbers).sum(),
        _ => 0,
    }
}

fn sum_numbers_no_red(value: &JsonValue) -> i32 {
    match value {
        JsonValue::Number(num) => Into::<f64>::into(*num) as i32,
        JsonValue::Object(obj) => {
            if obj
                .iter()
                .any(|(_, value)| value.is_string() && value.as_str().unwrap() == "red")
            {
                0
            } else {
                obj.iter().map(|(_, value)| sum_numbers_no_red(value)).sum()
            }
        }
        JsonValue::Array(arr) => arr.iter().map(sum_numbers_no_red).sum(),
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let json = json::parse(input).expect("must be valid json");

    Some(sum_numbers(&json))
}

pub fn part_two(input: &str) -> Option<i32> {
    let json = json::parse(input).expect("must be valid json");

    Some(sum_numbers_no_red(&json))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(-1));
    }
}

use md5::{Digest, Md5};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut key_suffix = 1;

    loop {
        let key = input.to_string() + &key_suffix.to_string();
        let mut hasher = Md5::new();

        hasher.update(key);
        let checksum = hasher.finalize();

        if checksum[0] == 0x00 && checksum[1] == 0x00 && (checksum[2] & 0xF0) == 0x00 {
            break;
        }

        key_suffix += 1;
    }

    Some(key_suffix)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut key_suffix = 1;

    loop {
        let key = input.to_string() + &key_suffix.to_string();
        let mut hasher = Md5::new();

        hasher.update(key);
        let checksum = hasher.finalize();

        if checksum[0] == 0x00 && checksum[1] == 0x00 && checksum[2] == 0x00 {
            break;
        }

        key_suffix += 1;
    }

    Some(key_suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1048970));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

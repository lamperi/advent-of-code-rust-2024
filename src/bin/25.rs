use std::iter::zip;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<i32> {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for schematics in input.split("\n\n") {
        if schematics.starts_with('#') {
            locks.push(schematics);
        } else {
            keys.push(schematics);
        }
    }
    let mut count = 0;
    for key in &keys {
        for lock in &locks {
            if zip(key.bytes(), lock.bytes()).all(|(xc, yc)| xc != b'#' || yc != b'#') {
                count += 1;
            }

        }
    }
    Some(count)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}

use std::iter::zip;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<i32> {
    let schematics: Vec<_> = input.split("\n\n").collect();
    let mut count = 0;
    for (i, x) in schematics.iter().enumerate() {
        for y in &schematics[i+1..] {
            if zip(x.chars(), y.chars()).all(|(xc, yc)| xc != '#' || yc != '#') {
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

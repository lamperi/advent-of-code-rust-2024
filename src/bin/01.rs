advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    input.lines().map(|line| {
        let mut i = line.split_ascii_whitespace();
        let left = i.next().unwrap().parse::<u64>().unwrap();
        let right = i.next().unwrap().parse::<u64>().unwrap();
        (left, right)
    }).unzip()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();
    Some(left.iter().zip(right.iter()).map(|(l, r)| {
        l.abs_diff(*r)
    }).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right) = parse(input);
    Some(left.iter().map(|l| {
        l * right.iter().filter(|r| {*r == l}).count() as u64
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

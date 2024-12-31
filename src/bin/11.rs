use std::collections::HashMap;

advent_of_code::solution!(11);

type Num = u64;

fn parse(input: &str) -> Vec<Num> {
    let stones: Result<Vec<_>, _> = input.split_ascii_whitespace()
        .map(|s| s.parse::<Num>())
        .collect();
    stones.unwrap()
}

fn common(input: &str, iterations: u32) -> Num {
    let stones = parse(input);
    let mut counts: HashMap<Num, Num> = HashMap::new();
    for s in stones.iter() {
        *counts.entry(*s).or_default() += 1;
    }
    for _iteration in 0..iterations {
        let mut next_counts: HashMap<Num, Num> = HashMap::new();
        for (stone, count) in counts {
            if stone == 0 {
                *next_counts.entry(1).or_default() += count;
            } else {
                let num_len = stone.ilog10() + 1;
                if num_len % 2 == 0{
                    let d = (10u64).pow(num_len/2);
                    let left = stone / d;
                    let right = stone % d;
                    *next_counts.entry(left).or_default() += count;
                    *next_counts.entry(right).or_default() += count;
                } else {
                    *next_counts.entry(stone * 2024).or_default() += count;
                }
            }
        }
        counts = next_counts;
    }
    counts.values().sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(common(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(common(input, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}

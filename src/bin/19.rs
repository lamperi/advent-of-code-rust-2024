use std::collections::HashSet;

advent_of_code::solution!(19);

fn parse(input: &str) -> (HashSet<&str>, Vec<&str>, usize) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let patterns: HashSet<&str> = patterns.split(", ").collect();
    let designs = designs.lines().collect();
    let max_pattern = patterns.iter().map(|d| d.len()).max().unwrap();
    (patterns, designs, max_pattern)
}

fn dp(design: &str, patterns: &HashSet<&str>, max_pattern: usize) -> u64 {
    let mut dp = vec![0; design.len()+1];
    dp[0] += 1;
    for i in 1..=design.len() {
        let prefix = &design[..i];
        for pat_len in 1..=max_pattern {
            if pat_len <= i && patterns.contains(&prefix[i - pat_len..]) {
                dp[i] += dp[i - pat_len];
            }
        }
    }
    dp[design.len()]
}

fn common(input: &str) -> (u64, u64) {
    let (patterns, designs, max_pattern) = parse(input);
    designs.iter()
        .fold((0, 0), |(p1, p2), design| {
            let s = dp(design, &patterns, max_pattern);
            if s > 0 {
                (p1 + 1, p2 + s)
            } else {
                (p1, p2)
            }
        })
}


pub fn part_one(input: &str) -> Option<u64> {
    Some(common(input).0)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(common(input).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}

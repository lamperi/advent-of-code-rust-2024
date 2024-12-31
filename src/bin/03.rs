use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Some(re.captures_iter(input).map(|c| {
        let (_, [a, b]) = c.extract();
        a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul|do|don't)\((\d+)?,?(\d+)?\)").unwrap();
    Some(re.captures_iter(input).map(|c| {
        let cmd = c.get(1).unwrap().as_str();
        let a = c.get(2).map(|s| s.as_str().parse::<u32>().unwrap());
        let b = c.get(3).map(|s| s.as_str().parse::<u32>().unwrap());
        (cmd, a, b)
    }).fold((true, 0), |(enabled, sum), parsed| {
        match parsed {
            ("mul", Some(a), Some(b)) if enabled => (enabled, sum + a * b),
            ("mul", _, _) => (enabled, sum),
            ("do", _, _) => (true, sum),
            ("don't", _, _) => (false, sum),
            _ => panic!("{:?}",parsed),
        }
    }).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}

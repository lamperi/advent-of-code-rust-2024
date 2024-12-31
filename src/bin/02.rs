advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<Vec<i32>> {
    input.lines()
    .map(|line| {
        line.split_ascii_whitespace()
            .map(|s| { s.parse::<i32>().unwrap() })
            .collect()
    }).collect()
}

fn is_safe_increasing(report: &[i32]) -> bool {
    report.windows(2).all(|pair| {
        let d = pair[1] - pair[0];
        (1..=3).contains(&d)
    })
} 

fn is_safe_report(report: &[i32]) -> bool {
    if is_safe_increasing(report) {
        true
    } else {
        let reversed: Vec<i32> = report.iter().copied().rev().collect();
        is_safe_increasing(&reversed)
    } 
}

pub fn part_one(input: &str) -> Option<usize> {
    let reports = parse(input);
    Some(reports.iter().filter(|&report| { is_safe_report(report) }).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let reports = parse(input);
    Some(reports.iter().filter(|&report| -> bool {
        if is_safe_report(report) {
            return true
        }
        for index in 0..report.len() {
            let mut clone = report.clone();
            clone.remove(index);
            if is_safe_report(&clone) {
                return true
            }
        }
        false
    }).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}

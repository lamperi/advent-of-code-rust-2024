advent_of_code::solution!(7);

type Num = u64;

fn parse(input: &str) -> Vec<(Num, Vec<Num>)> {
    input.lines()
    .map(|line|{
        let (target, operands) = line.split_once(": ").expect("split by ': '");
        let target = target.parse::<Num>().expect("target is a number");
        let operands: Result<Vec<_>, _> = operands.split_whitespace().map(|s| s.parse::<Num>()).collect();
        let operands = operands.expect("operands are numbers");
        (target, operands)
    })
    .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(parse(input).iter()
        .filter(|(target, operands)| {
            let len = operands.len();
            let mut stack = vec![(operands[0], 1)];
            while let Some((v, index)) = stack.pop() {
                if index == len && v == *target {
                    return true
                }
                if index < len && v <= *target {
                    let n = operands[index];
                    stack.push((v * n, index + 1));
                    stack.push((v + n, index + 1));
                }
            }
            false
        })
        .map(|(target,_ )| target)
        .sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(parse(input).iter()
    .filter(|(target, operands)| {
        fn concat(v: Num, n: Num) -> Num {
            if n < 10 { 10*v + n}
            else if n  < 100 { 100*v + n }
            else if n < 1000 {1000*v + n }
            else if n < 10000 { 10000*v + n }
            else { panic!("not enough cases") }
        }
        let len = operands.len();
        let mut stack = vec![(operands[0], 1)];
        while let Some((v, index)) = stack.pop() {
            if index == len && v == *target { 
                return true
            }
            if index < len && v <= *target {
                let n = operands[index];
                stack.push((concat(v, n), index + 1));
                stack.push((v * n, index + 1));
                stack.push((v + n, index + 1));
            }
        }
        false
    })
    .map(|(target,_ )| target)
    .sum())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}

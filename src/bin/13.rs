use regex::Regex;

advent_of_code::solution!(13);

type Num = i64;
type Pos2D = (Num, Num);

fn parse(input: &str, shift: Num) -> Vec<(Pos2D, Pos2D, Pos2D)> {
    let machine_regex: Regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)").unwrap();
    let v: Result<Vec<_>, _> = input.split("\n\n")
    .map(|block| -> Result<(Pos2D, Pos2D, Pos2D), Box<dyn std::error::Error>> {
        let (_, [ax, ay, bx, by, px, py]) =
            machine_regex.captures(block).ok_or("could not match regex")?.extract();
        let ax = ax.parse::<Num>()?;
        let ay = ay.parse::<Num>()?;
        let bx = bx.parse::<Num>()?;
        let by = by.parse::<Num>()?;
        let px = px.parse::<Num>()? + shift;
        let py = py.parse::<Num>()? + shift;
        Ok(((ax, ay), (bx, by), (px, py)))
    }).collect();
    v.expect("was not able to parse")
}

fn solve(params: &(Pos2D, Pos2D, Pos2D)) -> Num {
    let ((ax, ay), (bx, by), (px, py)) = params;
    let m = (ax * py - ay * px)/(ax * by - ay * bx);
    let n = (px - m * bx)/(ax);
    if n * ax + m *bx == *px && n * ay + m * by == *py {
        3*n + m
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<Num> {
    Some(parse(input, 0).iter()
        .map(solve)
        .sum())
}

pub fn part_two(input: &str) -> Option<Num> {
    Some(parse(input, 10000000000000).iter()
    .map(solve)
    .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}

use regex::Regex;

advent_of_code::solution!(14);

fn parse(input: &str) -> Vec<[i32; 4]>{
    let regex = Regex::new(r"-?\d+").unwrap();
    input.lines()
        .map(|line|{
            regex.find_iter(line)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        }).collect()
}

fn safety_factor(robots: &mut dyn Iterator<Item = [i32; 2]>, width: i32, height: i32) -> u32 {
    let mut quadrants = [0, 0, 0, 0];
    let mx = (width - 1)/2;
    let my = (height - 1)/2;
    for [x,y] in robots {
        if x == mx || y == my {
            continue
        }
        let idx = if x < mx {0} else {1} + if y < my {0} else {2};
        quadrants[idx] += 1;
    }
    quadrants.iter().product()
}

pub fn part_one(input: &str) -> Option<u32> {
    let robots = parse(input);
    let (width, height) = if robots.len() < 20 {
        (11, 7)
    } else {
        (101, 103)
    };

    let mut robots = robots.iter().map(|[x,y,vx, vy]| {
        [(x + 100 * vx).rem_euclid(width), (y + 100 * vy).rem_euclid(height)]
    });
    Some(safety_factor(&mut robots, width, height))
}

pub fn part_two(input: &str) -> Option<u32> {
    let robots = parse(input);
    let (width, height) = if robots.len() < 20 {
        (11, 7)
    } else {
        (101, 103)
    };

    // It happens so that the picture of the Christmas tree is displayed when
    // the safety factor is the lowest possible (an easter egg). This is due to
    // border of the picture being in the middle of y or x, which greatly reduces
    // the numbers of the product of safety factor. 
    (1..=width*height).map(|t| {
        let mut robots = robots.iter().map(|[x,y,vx, vy]| {
            [(x + t * vx).rem_euclid(width), (y + t * vy).rem_euclid(height)]
        });

        (safety_factor(&mut robots, width, height), t)
    }).min().map(|(_safety_factor, t)| t as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}

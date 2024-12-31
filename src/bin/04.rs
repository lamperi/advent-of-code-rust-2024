use std::collections::HashMap;

use advent_of_code::direction::Direction;

advent_of_code::solution!(4);

fn to_map(input: &str) -> HashMap<(usize, usize), char> {
    input.split('\n')
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate()
                .map(move |(x, c) | {
                    ((y, x), c)
                })
        })
        .collect()
}


pub fn part_one(input: &str) -> Option<usize> {
    let area = to_map(input);
    let c = area.iter()
        .filter(|(_, &c)| { c == 'X'})
        .map(|(&coord, _)| {
            Direction::eight_directions().iter()
                .filter(|dir| {
                    "MAS".chars().scan(coord, |pos, c| {
                        let next_pos = dir.shift(*pos);
                        *pos = next_pos;
                        Some((next_pos, c))
                    }).all(|(pos, c)| {
                        match area.get(&pos) {
                            Some(&a) => a == c,
                            None => false,
                        }
                    })
                }).count()
        }).sum();
    Some(c)
}

pub fn part_two(input: &str) -> Option<usize> {
    let area = to_map(input);
    let c = area.iter()
        .filter(|(_, &c)| { c == 'A'})
        .filter(|(&coord, _)| {
            let a1 = [Direction::NorthWest, Direction::SouthEast];
            let a2 = [Direction::NorthEast, Direction::SouthWest];
            [a1, a2].iter()
                .all(|dirs| {
                    let c1 = dirs[0].shift(coord);
                    let c2 = dirs[1].shift(coord);
                    matches!(
                        (area.get(&c1), area.get(&c2)),
                        (Some('M'), Some('S')) | (Some('S'), Some('M')))
                })
        })
        .count();
    Some(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}


use std::collections::HashSet;

use advent_of_code::direction::CardinalDirection;
use advent_of_code::graph::dijkstra;
use advent_of_code::graph::dijkstra_equal_paths;

advent_of_code::solution!(16);

type Pos2D = (usize, usize);
type State = (Pos2D, CardinalDirection);

fn parse(input: &str) -> (Pos2D, Pos2D, HashSet<Pos2D>) {
    let mut start: Option<Pos2D> = None;
    let mut end: Option<Pos2D> = None;
    let mut walls = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Some((y,x));
            } else if c == 'E' {
                end = Some((y,x));
            } else if c == '#' {
                walls.insert((y,x));
            }
        }
    }
    (start.expect("start not found"),
    end.expect("end not found"),
    walls)
}

fn edges(state: &State, walls: &HashSet<Pos2D>) -> Vec<(u32, State)> {
    let (pos, dir) = *state;
    [
        (1, (dir.shift(pos), dir)),
        (1000, (pos, dir.turn_left())),
        (1000, (pos, dir.turn_right()))
    ].into_iter()
    .filter(|(_, (pos, _))| !walls.contains(pos))
    .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start, end, walls) = parse(input);

    dijkstra((start, CardinalDirection::East),
    |state| state.0 == end,
    |state| edges(state, &walls))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (start, end, walls) = parse(input);
    dijkstra_equal_paths((start, CardinalDirection::East),
    |state| state.0 == end,
    |state| edges(state, &walls))
    .map(|(_cost, nodes)| {
        nodes.iter()
            .map(|state| state.0)
            .collect::<HashSet<_>>().len()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(64));
    }
}

use std::collections::{HashSet, VecDeque};
use advent_of_code::direction::CardinalDirection;

advent_of_code::solution!(18);


type Num = i32;
type Pos = (Num, Num);

fn parse(input: &str) -> Vec<Pos> {
    input.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        (
            x.parse::<Num>().unwrap(),
            y.parse::<Num>().unwrap()
        )
    }).collect::<Vec<_>>()
}

fn bfs<F>(start: Pos, end: Pos, edges: F) -> Option<i32>
where
    F: Fn(&Pos) -> Vec<Pos> {
    let mut queue = VecDeque::new();
    queue.push_back((0, start));
    let mut visited = HashSet::new();
    visited.insert(start);
    while let Some((steps, pos)) = queue.pop_front() {
        if pos == end {
            return Some(steps)
        }
        for next_pos in edges(&pos) {
            if visited.insert(next_pos) {
                queue.push_back((steps+1, next_pos));
            }
        }
    }
    None
}

fn edges(pos: &Pos, corrupted: &HashSet<&Pos>, max_yx: i32) -> Vec<Pos> {
    CardinalDirection::all().iter().filter_map(|dir| {
        let next_pos = dir.shift_i32(*pos);
        if !corrupted.contains(&next_pos)
                && (0..=max_yx).contains(&next_pos.0)
                && (0..=max_yx).contains(&next_pos.1) {
            Some(next_pos)
        } else {
            None
        }
    }).collect::<Vec<Pos>>()
}


pub fn part_one(input: &str) -> Option<i32> {
    let coords = parse(input);
    let (max_yx, first_n) = if coords.len() < 30 {
        (6, 12)
    } else {
        (70, 1024)
    };
    let start = (0, 0);
    let end = (max_yx, max_yx);

    let corrupted = coords.iter().take(first_n).collect::<HashSet<_>>();

    bfs(start, end, move |pos| edges(pos, &corrupted, max_yx))
}

pub fn part_two(input: &str) -> Option<String> {
    let coords = parse(input);
    let (max_yx, first_n) = if coords.len() < 30 {
        (6, 12)
    } else {
        (70, 1024)
    };
    let start = (0, 0);
    let end = (max_yx, max_yx);

    let mut lo = first_n;
    let mut hi = coords.len();
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        let corrupted = coords.iter().take(mid).collect::<HashSet<_>>();
        if bfs(start, end, move |pos| edges(pos, &corrupted, max_yx)).is_none() {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    let (x,y) = coords[lo];
    Some(format!("{},{}", x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("6,1")));
    }
}

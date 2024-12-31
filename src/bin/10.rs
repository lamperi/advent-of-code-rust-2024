use std::collections::{HashSet, VecDeque};
use advent_of_code::direction::Direction;


advent_of_code::solution!(10);

fn parse(input: &str) -> (Vec<Vec<u8>>, Vec<(usize, usize)>) {
    let mut starts = Vec::new();
    let topology = input.lines().enumerate().map(|(y,line)| {
        line.chars().enumerate()
            .inspect(|(x, c)| {
                if *c == '0' {
                    starts.push((y, *x));
                }
            })
            .map(|(_, c)| c.to_digit(10)
                .expect("char must be [0-9]") as u8)
            .collect()
    }).collect();
    (topology, starts)
}

fn topology_search(input: &str) -> (usize, u32) {
    let (topology, starts) = parse(input);
    starts.iter()
        .map(move |start| {
            let mut peaks = HashSet::new();
            let mut paths = 0;
            let mut deq = VecDeque::from([(*start, 0)]);
            while let Some((pos, height)) = deq.pop_front() {
                for dir in Direction::cardinal_directions() {
                    let new_pos = dir.shift(pos);
                    let neighbor_height = topology.get(new_pos.0).and_then(|line| line.get(new_pos.1)).copied();
                    if let Some(neighbor_height) = neighbor_height {
                        if height + 1 == neighbor_height {
                            if neighbor_height == 9 {
                                peaks.insert(new_pos);
                                paths += 1;
                            } else {
                                deq.push_back((new_pos, neighbor_height));
                            }
                        }
                    }
                }
            }
            (peaks.len(), paths)
        }).fold((0, 0), |(peaks, paths), (item_peaks, item_paths)|
            (peaks + item_peaks, paths + item_paths)
        )
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(topology_search(input).0)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(topology_search(input).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}

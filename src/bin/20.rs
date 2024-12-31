use std::collections::HashMap;

advent_of_code::solution!(20);

use advent_of_code::{graph::{bfs, build_path}, grid::Grid, pos::Pos2D};
use num::abs;

fn generate_pos(pos: &Pos2D<usize>, max_distance: usize) -> Vec<Pos2D<usize>> {
    let max_distance = max_distance as isize;
    (-max_distance..=max_distance).flat_map(|y| {
        let remaining = max_distance - abs(y);
        (-remaining..=remaining).flat_map(move |x| {
            let y = y + pos.y as isize;
            let x = x + pos.x as isize;
            if y >= 0 && x >= 0 {
                Some(Pos2D{y: y as usize, x: x as usize})
            } else {
                None
            }
        })
    })
    .collect()
}

fn get_saved_distances(input: &str, max_distance: usize) -> Vec<usize> {
    let grid = Grid::new(input).unwrap();
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();

    let (_cost, parents) = bfs(start, |st| *st == end, |st| {
        grid.neighbors(st)
            .into_iter()
            .filter(|pos| grid.get_pos(pos) != b'#')
    });
    let path = build_path(end, &parents);

    if max_distance > 2 {
        // For part 2, it's more efficient to check the remaining elements in the path.
        let mut saved_distances = Vec::new();
        for (a,b) in path.iter().enumerate() {
            for (c, d) in path.iter().enumerate().skip(a) {
                let md = b.manhattan_distance(d);
                if md <= max_distance && c > a + md {
                    saved_distances.push(c - a - md);
                }
            }
        }
        saved_distances
    } else {
        // For part 1, it's more efficient to generate all reachable nodes within the distance and check them.
        let pos_from_start = path.iter().enumerate().map(|(index, pos)| {
            (pos, index)
        }).collect::<HashMap<_, _>>();
            pos_from_start.iter()
        .flat_map(|(pos, &from_start)| {
            generate_pos(pos, max_distance).iter()
                .filter_map(|n_pos| {
                    pos_from_start.get(n_pos).and_then(|&n_from_start| {
                        let md = n_pos.manhattan_distance(pos);
                        if n_from_start > from_start + md {
                            Some(n_from_start.saturating_sub(from_start + md))
                        } else {
                            None
                        }
                    })
                }).collect::<Vec<usize>>()
        }).collect::<Vec<usize>>()
    }
}


pub fn part_one(input: &str) -> Option<usize> {
    Some(get_saved_distances(input, 2).into_iter().filter(|&saving| {
        saving >= 100
    }).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(get_saved_distances(input, 20).into_iter().filter(|&saving| {
        saving >= 100
    }).count())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let result: Vec<usize> = get_saved_distances(&advent_of_code::template::read_file("examples", DAY), 2);
        let mut saves: HashMap<usize, usize> = HashMap::new();
        for &r in &result {
            *saves.entry(r).or_default() += 1;
        }
        assert_eq!(result.len(), 44);
        assert_eq!(saves, HashMap::from([
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1)
        ]));
    }

    #[test]
    fn test_part2() {
        let result: Vec<usize> = get_saved_distances(&advent_of_code::template::read_file("examples", DAY), 20);
        let mut saves: HashMap<usize, usize> = HashMap::new();
        for &r in &result {
            if r >= 50 { 
                *saves.entry(r).or_default() += 1;
            }
        }
        assert_eq!(saves, HashMap::from([
            (50, 32),
            (52, 31),
            (54, 29),
            (56, 39),
            (58, 25),
            (60, 23),
            (62, 20),
            (64, 19),
            (66, 12),
            (68, 14),
            (70, 12),
            (72, 22),
            (74, 4),
            (76, 3)
        ]));
    }
}

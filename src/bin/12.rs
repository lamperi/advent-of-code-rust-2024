use std::collections::HashSet;

use advent_of_code::direction::Direction;

advent_of_code::solution!(12);

type Pos = (usize, usize);

fn parse(input: &str) -> Vec<Vec<char>> {
    let topology: Vec<Vec<_>> = input.lines().enumerate().map(|(_,line)| {
        line.chars().enumerate()
            .map(|(_, c)| c)
            .collect()
    }).collect();
    topology
}

fn dfs(topology: &[Vec<char>], pos: Pos, plot: char, visited_plots: &mut HashSet<Pos>) -> (u32, HashSet<(Pos, Direction)>) {
    visited_plots.insert(pos);
    let mut stack = vec![pos];
    let mut area = 0;
    let mut perimeter = HashSet::new();
    while let Some(pos) = stack.pop() {
        area += 1;
        for dir in Direction::cardinal_directions() {
            let neighbor_pos = dir.shift(pos);
            if let Some(neighbor_plot) = topology.get(neighbor_pos.0).and_then(|line| line.get(neighbor_pos.1)) {
                if *neighbor_plot == plot {
                    if visited_plots.insert(neighbor_pos) {
                        stack.push(neighbor_pos);
                    }
                } else {
                    perimeter.insert((pos, dir));
                }
            } else {
                perimeter.insert((pos, dir));
            }
        }
    }
    (area, perimeter)
}

fn count_sides(perimeter: &mut HashSet<(Pos, Direction)>) -> u32 {
    let mut sides = 0;
    while let Some(edge) = perimeter.iter().next().cloned() {
        sides += 1;
        perimeter.remove(&edge);
        let (pos, dir) = &edge;
        let dirs = if dir.is_horizontal() {
            [Direction::North, Direction::South]
        } else {
            [Direction::West, Direction::East]
        };
        
        for edge_dir in dirs {
            let mut v = Vec::new();
            let mut next_pos = edge_dir.shift(*pos);
            while perimeter.remove(&(next_pos, *dir)) {
                v.push(next_pos);
                next_pos = edge_dir.shift(next_pos);
            }
            //println!("processed a side to {:?} starting from {:?} containing {:?}", dir, edge, v);
        }
    }
    sides
}

pub fn part_one(input: &str) -> Option<u32> {
    let topology = parse(input);
    let mut visited_plots: HashSet<Pos> = HashSet::new();
    let mut total_price = 0;
    for (y, line) in topology.iter().enumerate() {
        for (x, plot) in line.iter().enumerate() {
            let pos = (y, x);
            if visited_plots.contains(&pos) {
                continue
            }
            let (area, perimeter) = dfs(&topology, pos, *plot, &mut visited_plots);
            let perimeter = perimeter.len() as u32;
            //println!("plot {} area {} perimeter {}, price {}", plot, area, perimeter.len(), area*perimeter.len() as u32);
            total_price += area*perimeter;
        }
    }
    Some(total_price)
}

pub fn part_two(input: &str) -> Option<u32> {
    let topology = parse(input);
    let mut visited_plots: HashSet<Pos> = HashSet::new();
    let mut total_price = 0;
    for (y, line) in topology.iter().enumerate() {
        for (x, plot) in line.iter().enumerate() {
            let pos = (y, x);
            if visited_plots.contains(&pos) {
                continue
            }
            let (area, mut perimeter) = dfs(&topology, pos, *plot, &mut visited_plots);
            let sides = count_sides(&mut perimeter);
            //println!("plot {} area {} sides {}, price {}", plot, area, sides, area*sides);
            total_price += area*sides;
        }
    }
    Some(total_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two_three() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_four() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 4));
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_five() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 5));
        assert_eq!(result, Some(368));
    }
}

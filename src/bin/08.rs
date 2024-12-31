use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

type Pos2D = (usize, usize);
type AntennaMap = HashMap<char, Vec<Pos2D>>;

fn add(pos1: &Pos2D, pos2: &Pos2D) -> Pos2D {
    (pos1.0.wrapping_add(pos2.0), pos1.1.wrapping_add(pos2.1))
}
fn sub(pos1: &Pos2D, pos2: &Pos2D) -> Pos2D {
    (pos1.0.wrapping_sub(pos2.0), pos1.1.wrapping_sub(pos2.1))
}

fn parse(input: &str) -> (AntennaMap, Pos2D) {
    let mut antennas: AntennaMap = HashMap::new();
    let max_yx = input.lines().enumerate().map(|(y,line)| {
        line.chars().enumerate()
            .inspect(|(x, c)| {
                if *c != '.' {
                    antennas.entry(*c).or_default().push((y,*x));
                }
            }) 
            .map(|(x, _)| (y, x))
            .max().unwrap()
    }).max().unwrap();
    (antennas, max_yx)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (antennas, (max_y, max_x)) = parse(input);
    let ret = antennas.values()
        .flat_map(|values| {
            let mut antinodes = Vec::new();
            for pos1 in values {
                for pos2 in values {
                    if pos1 == pos2 {
                        continue
                    }
                    let d = sub(pos1, pos2);
                    let antinode = add(pos1, &d);
                    if (0..=max_y).contains(&antinode.0) && (0..=max_x).contains(&antinode.1) {
                        antinodes.push(antinode);
                    }
                    let antinode = sub(pos2, &d);
                    if (0..=max_y).contains(&antinode.0) && (0..=max_x).contains(&antinode.1) {
                        antinodes.push(antinode);
                    }
                }
            }
            antinodes
        })
        .collect::<HashSet<_>>().len();
    Some(ret)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (antennas, (max_y, max_x)) = parse(input);
    let ret = antennas.values()
        .flat_map(|values| {
            let mut antinodes = Vec::new();
            for pos1 in values {
                for pos2 in values {
                    if pos1 == pos2 {
                        continue
                    }
                    let d = sub(pos1, pos2);
                    let mut antinode = *pos1;
                    while (0..=max_y).contains(&antinode.0) && (0..=max_x).contains(&antinode.1) {
                        antinodes.push(antinode);
                        antinode = add(&antinode, &d)
                    }
                    let mut antinode = *pos2;
                    while (0..=max_y).contains(&antinode.0) && (0..=max_x).contains(&antinode.1) {
                        antinodes.push(antinode);
                        antinode = sub(&antinode, &d)
                    }
                }
            }
            antinodes
        })
        .collect::<HashSet<_>>().len();
    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}

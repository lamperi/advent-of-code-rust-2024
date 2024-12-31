use std::collections::{hash_map::Entry, HashMap, HashSet};
use advent_of_code::{direction::CardinalDirection, grid::Grid, pos::Pos2D};

advent_of_code::solution!(15);

struct Box {
    id: usize,
    pos: Pos2D<usize>,
    width: usize
}

impl Box {
    fn all_pos(&self) -> impl Iterator<Item = Pos2D<usize>> + '_ {
        (0..self.width).into_iter()
            .map(|x| self.pos + Pos2D{y: 0, x})
    }

    fn all_pos_after_move(&self, dir: CardinalDirection) -> impl Iterator<Item = Pos2D<usize>> + '_ {
        (0..self.width).into_iter()
            .map(move |x| (self.pos + Pos2D{y: 0, x}) + dir)
    }

    fn move_to(&mut self, dir: CardinalDirection) {
        self.pos += dir;
    } 
}

// This routine can be used to print the grid.
#[allow(dead_code)]
fn print_state(pos: Pos2D<usize>, grid: &Grid<'_>, boxes: &Vec<Box>) {
    let mut s = String::new();
    let boxes = boxes.iter().flat_map(|b| b.all_pos()).collect::<HashSet<_>>();
    for y in 0..grid.height {
        for x in 0..grid.width {
            let grid_pos = Pos2D{y, x};
            if pos == grid_pos {
                s += "@";
            } else if boxes.contains(&grid_pos) {
                s += "O";
            } else {
                match grid.get_pos(&grid_pos) as char {
                    '#' => { s. push('#'); }
                    _ => { s.push('.') }
                }
            }
        }
        s.push('\n');
    }
    println!("{}", s);
}

fn common(moves: &str, grid: Grid<'_>, mut boxes: Vec<Box>) -> usize {
    let mut pos = grid.find(b'@').unwrap();
    let mut boxgrid = boxes.iter()
        .flat_map(|b| b.all_pos().map(move |pos| (pos, b.id)))
        .collect::<HashMap<_, _>>();

    let move_to_dir = HashMap::from([
        ('v', CardinalDirection::South),
        ('^', CardinalDirection::North),
        ('<', CardinalDirection::West),
        ('>', CardinalDirection::East)
    ]);
    
    for mov in moves.chars() {
        if mov == '\n' { continue }
        let dir = *move_to_dir.get(&mov).unwrap();
        let mut colliding_pos = vec![pos + dir];
        let mut collides_wall = false;
        let mut moved_boxes: HashSet<usize> = HashSet::new();
        while !collides_wall && !colliding_pos.is_empty(){
            let mut colliding_boxes: Vec<&Box> = Vec::new();
            for pos in &colliding_pos {
                if grid.get_pos(pos) == b'#' {
                    collides_wall = true;
                    break
                }
                if let Entry::Occupied(colliding) = boxgrid.entry(*pos) {
                    let id = *colliding.get();
                    if moved_boxes.insert(id) {
                        colliding_boxes.push(boxes.get(id).unwrap());
                    }
                }
            }
            colliding_pos.clear();
            let next_pos = colliding_boxes.iter()
                .flat_map(|b| b.all_pos_after_move(dir));
            colliding_pos.extend(
                next_pos
            );
        }
        if !collides_wall {
            pos += dir;
            for b in &moved_boxes {
                let b = boxes.get_mut(*b).unwrap();
                for p in b.all_pos() {
                    boxgrid.remove(&p);
                }
            }
            for b in &moved_boxes {
                boxes.get_mut(*b).unwrap().move_to(dir);
            }
            for b in &moved_boxes {
                let b = boxes.get_mut(*b).unwrap();
                for p in b.all_pos() {
                    boxgrid.insert(p, b.id);
                }
            }
        }
    }
    boxes.iter().map(|b| b.pos.y * 100 + b.pos.x).sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let grid = Grid::new(grid).unwrap();
    let boxes = grid.find_all(b'O')
        .enumerate()
        .map(|(id, pos)|Box{pos, id, width: 1}).collect::<Vec<_>>();
    Some(common(moves, grid, boxes))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    // Duplicate grid width.
    let grid = grid
        .replace('#', "##")
        .replace('.', "..")
        .replace('O', "O.")
        .replace('@', "@.");
    let grid = Grid::new(grid.as_str()).unwrap();
    let boxes = grid.find_all(b'O')
        .enumerate()
        .map(|(id, pos)|Box{pos, id, width: 2}).collect::<Vec<_>>();
    Some(common(moves, grid, boxes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}

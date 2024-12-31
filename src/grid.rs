use crate::pos::Pos2D;
use crate::direction::CardinalDirection;

pub struct Grid<'a> {
    pub grid: &'a [u8],
    pub width: usize,
    pub height: usize
}

impl <'a> Grid<'a> {
    pub fn new(grid: &'a str) -> Option<Self> {
        let grid = grid.as_bytes();
        let width = grid.iter().enumerate()
            .find_map(|(pos, b)| if *b == b'\n' { Some(pos) } else { None });
        let height = width.map(|w| grid.len() / w);
        width.and_then(|width| {
            height.and_then(|height| {
                Some(Grid{grid, width, height})
            })
        })
    }

    pub fn find(&self, p: u8) -> Option<Pos2D<usize>> {
        let pos = self.grid.iter().enumerate()
            .find_map(|(pos, b)| if *b == p { Some(pos) } else { None });
        return pos.map(|p| self.index_to_pos(p));
    }

    pub fn find_all(&self, p: u8) -> impl Iterator<Item = Pos2D<usize>> + '_ {
        self.grid.iter().enumerate()
            .filter_map(move |(pos, b)| if *b == p { Some(self.index_to_pos(pos)) } else { None })
    }

    pub fn get_pos(&self, pos: &Pos2D<usize>) -> u8 {
        self.grid[self.pos_to_index(pos)]
    }

    pub fn neighbors(&self, pos: &Pos2D<usize>) -> Vec<Pos2D<usize>> {
        let pos = *pos;
        CardinalDirection::all().iter()
            .map(|&dir| {
                pos + dir
            })
            .filter(|pos| self.contains(&pos))
            .collect()
    }

    pub fn contains(&self, pos: &Pos2D<usize>) -> bool {
        pos.y < self.height && pos.x < self.width
    }

    fn pos_to_index(&self, pos: &Pos2D<usize>) -> usize {
        pos.y * (1 + self.width) + pos.x
    }

    fn index_to_pos(&self, index: usize) -> Pos2D<usize> {
        Pos2D{y: index / (1 + self.width), x: index % (1 + self.width)}
    }
}
use crate::direction::CardinalDirection;

#[derive(Debug,PartialEq,Eq,Copy,Clone,Hash)]
pub struct Pos2D<T> {
    pub x: T,
    pub y: T,
}

impl <T> Pos2D<T> where
T: Copy + Ord + std::ops::Add<Output = T> + std::ops::Sub<Output = T> {
   pub fn manhattan_distance(&self, other: &Pos2D<T>) -> T {
        let max_x = if self.x > other.x { self.x } else { other.x };
        let min_x = if self.x < other.x { self.x } else { other.x };
        let max_y = if self.y > other.y { self.y } else { other.y };
        let min_y = if self.y < other.y { self.y } else { other.y };
        (max_x - min_x) + (max_y - min_y)
    }
}

impl std::ops::Add<Pos2D<usize>> for Pos2D<usize> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Pos2D { x: self.x.wrapping_add(rhs.x), y: self.y.wrapping_add(rhs.y) }
    }
}

impl std::ops::Add<CardinalDirection> for Pos2D<usize> {
    type Output = Self;

    fn add(self, rhs: CardinalDirection) -> Self {
        Pos2D {
            x: self.x.wrapping_add_signed(rhs.dx().into()),
            y: self.y.wrapping_add_signed(rhs.dy().into())
        }
    }
}

impl std::ops::AddAssign<CardinalDirection> for Pos2D<usize> {
    fn add_assign(&mut self, rhs: CardinalDirection) {
        self.x = self.x.wrapping_add_signed(rhs.dx().into());
        self.y = self.y.wrapping_add_signed(rhs.dy().into());
    }
}
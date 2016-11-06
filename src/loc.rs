use vec2::Vec2;
use map::CELL_SIZE;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Loc(pub i64, pub i64);

impl Loc {
    pub fn top_left(self) -> Vec2 {
        Vec2(self.0 as f64 * CELL_SIZE, self.1 as f64 * CELL_SIZE)
    }

    pub fn top_right(self) -> Vec2 {
        Vec2(self.0 as f64 * (CELL_SIZE + 1.), self.1 as f64 * CELL_SIZE)
    }

    pub fn bottom_left(self) -> Vec2 {
        Vec2(self.0 as f64 * CELL_SIZE, self.1 as f64 * (CELL_SIZE + 1.))
    }

    pub fn bottom_right(self) -> Vec2 {
        Vec2(self.0 as f64 * (CELL_SIZE + 1.), self.1 as f64 * (CELL_SIZE + 1.))
    }

    pub fn middle_point(self) -> Vec2 {
        Vec2(self.0 as f64 * (CELL_SIZE + 0.5), self.1 as f64 * (CELL_SIZE + 0.5))
    }
}

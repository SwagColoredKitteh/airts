use loc::Loc;
use map::CELL_SIZE;
use vec2::Vec2;

use std::iter;

// NOTE: Might want this unsigned.

/// A size specified in grid cells.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size(pub i64, pub i64);

impl Size {
    /// Convert this `Size` from cells to world coordinates.
    pub fn to_vec(self) -> Vec2 {
        Vec2(self.0 as f64 * CELL_SIZE, self.1 as f64 * CELL_SIZE)
    }

    /// Get the area described by this `Size`.
    ///
    /// # Examples
    ///
    /// ```
    /// let size = Size(2, 2);
    ///
    /// assert_eq!(size.area(), 4);
    /// ```
    pub fn area(self) -> i64 {
        self.0 * self.1
    }

    /// Create an iterator that yields all the locations in this `Size`.
    ///
    /// # Examples
    ///
    /// ```
    /// let size = Size(2, 2);
    /// let mut iter = size.loc_iter();
    ///
    /// assert_eq!(iter.next(), Some(Loc(0, 0)));
    /// assert_eq!(iter.next(), Some(Loc(1, 0)));
    /// assert_eq!(iter.next(), Some(Loc(0, 1)));
    /// assert_eq!(iter.next(), Some(Loc(1, 1)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn loc_iter(self) -> LocIterator {
        LocIterator {
            first: true,
            start_x: 0,
            x: 0,
            y: 0,
            w: self.0,
            h: self.1
        }
    }
}

/// An iterator over the locations in a rectangle described by a Size.
pub struct LocIterator {
    first: bool,
    start_x: i64,
    x: i64,
    y: i64,
    w: i64,
    h: i64
}

impl iter::Iterator for LocIterator {
    type Item = Loc;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            Some(Loc(self.x, self.y))
        }
        else {
            self.x += 1;
            if self.x >= self.w {
                self.y += 1;
                self.x = self.start_x;
                if self.y >= self.h {
                    return None;
                }
            }
            Some(Loc(self.x, self.y))
        }
    }
}

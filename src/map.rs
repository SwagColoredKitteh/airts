use size::Size;
use loc::Loc;
use vec2::Vec2;

use std::io::prelude::*;
use std::io;

use std::ops::{Index, IndexMut};

pub type TileId = usize;

pub const CELL_SIZE: f64 = 64.;

pub struct TileInfo {
    pub solid: bool
}

static TILE_INFO: [TileInfo; 2] = [
    TileInfo {
        solid: false
    },
    TileInfo {
        solid: true
    }
];

pub fn tile_info(id: TileId) -> &'static TileInfo {
    &TILE_INFO[id]
}

#[derive(Clone, Debug)]
pub struct Map {
    size: Size,
    data: Vec<TileId>
}

impl Map {
    pub fn new_filled(size: Size, fill: TileId) -> Map {
        Map {
            size: size,
            data: vec![fill; size.area() as usize]
        }
    }

    // TODO: better error handling
    // TODO: better data checking (sanity checks are very lacking)
    // TODO: check whether values are in the tilemap's range
    pub fn from_stream<R: BufRead>(r: R) -> Result<Map, ()> {
        let mut lines = r.lines().map(Result::unwrap);
        let wh_line = try!(lines.next().ok_or(()));
        let wh_vec: Vec<usize> = wh_line.split(" ").take(2).filter_map(|n| n.parse().ok()).collect();
        if wh_vec.len() < 2 { return Err(()); }
        let (w, h) = (wh_vec[0], wh_vec[1]);
        let s = Size(w as i64, h as i64);
        let mut data = Vec::with_capacity(s.area() as usize);
        for line in lines.take(h) {
            for c in line.chars().take(w) {
                data.push(c.to_string().parse::<TileId>().unwrap_or(0));
            }
        }
        Ok(Map {
            size: s,
            data: data
        })
    }

    pub fn dump_state<W: Write>(&self, w: &mut W) -> Result<(), io::Error> {
        try!(writeln!(w, "{} {}", self.size.0, self.size.1));
        for y in 0..self.size.1 {
            let idx = (y * self.size.0) as usize;
            let row = &self.data[idx..idx + self.size.0 as usize];
            try!(writeln!(w, "{}", row.iter().map(|id| id.to_string().chars().next().unwrap()).collect::<String>()));
        }
        Ok(())
    }

    pub fn loc_iter(&self) -> impl Iterator<Item = Loc> {
        self.size.loc_iter()
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn to_loc(&self, pos: Vec2) -> Loc {
        Loc((pos.0 / CELL_SIZE) as i64, (pos.1 / CELL_SIZE) as i64)
    }

    pub fn is_solid(&self, pos: Vec2) -> bool {
        let loc = self.to_loc(pos);
        tile_info(self[loc]).solid
    }
}

impl Index<Loc> for Map {
    type Output = TileId;

    fn index(&self, Loc(x, y): Loc) -> &Self::Output {
        if x < 0 || y < 0 || x >= self.size.0 || y >= self.size.1 {
            panic!("Index out of bounds: {:?}", Loc(x, y));
        }
        unsafe { self.data.get_unchecked(y as usize * self.size.0 as usize + x as usize) }
    }
}

impl IndexMut<Loc> for Map {
    fn index_mut(&mut self, Loc(x, y): Loc) -> &mut Self::Output {
        if x < 0 || y < 0 || x >= self.size.0 || y >= self.size.1 {
            panic!("Index out of bounds: {:?}", Loc(x, y));
        }
        unsafe { self.data.get_unchecked_mut(y as usize * self.size.0 as usize + x as usize) }
    }
}

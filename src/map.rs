use tile_map::{TileMap, TileInfo, TileId};
use size::Size;
use loc::Loc;

use std::io::prelude::*;

use std::ops::{Index, IndexMut};

pub const CELL_SIZE: f64 = 64.;

#[derive(Clone, Debug)]
pub struct Map {
    size: Size,
    pub tile_map: TileMap,
    data: Vec<TileId>
}

impl Map {
    pub fn new_filled(tile_map: TileMap, size: Size, fill: TileId) -> Map {
        Map {
            size: size,
            tile_map: tile_map,
            data: vec![fill; size.area() as usize]
        }
    }

    // TODO: better error handling
    // TODO: better data checking (sanity checks are very lacking)
    // TODO: check whether values are in the tilemap's range
    pub fn from_stream<R: BufRead>(tile_map: TileMap, r: R) -> Result<Map, ()> {
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
            tile_map: tile_map,
            data: data
        })
    }

    pub fn loc_iter(&self) -> impl Iterator<Item = Loc> {
        self.size.loc_iter()
    }

    pub fn size(&self) -> Size {
        self.size
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

pub type TileId = usize;

#[derive(Clone, Debug)]
pub struct TileMap {
    tiles: Vec<TileInfo>
}

impl TileMap {
    pub fn new(tiles: Vec<TileInfo>) -> TileMap {
        TileMap {
            tiles: tiles
        }
    }

    pub fn tile_info<'a>(&'a self, id: TileId) -> Option<&'a TileInfo> {
        self.tiles.get(id)
    }
}

#[derive(Clone, Debug)]
pub struct TileInfo {
    pub solid: bool
}

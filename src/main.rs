#![feature(conservative_impl_trait)]

extern crate piston_window;

mod vec2;
mod unit;
mod structure;
mod player;
mod owner;
mod game;
mod command;
mod loc;
mod size;
mod map;
mod tile_map;

mod renderer;

use game::GameState;
use player::PlayerState;
use structure::StructureType;
use unit::UnitType;
use renderer::Renderer;
use loc::Loc;
use size::Size;
use map::Map;
use tile_map::{TileMap, TileInfo, TileId};
use vec2::Vec2;
use command::Command;

use std::thread;
use std::time::Duration;
use std::fs::File;

use std::io::BufReader;

fn main() {
    let mut renderer = Renderer::new(480, 480).unwrap();

    let tile_map = TileMap::new(vec![
        TileInfo { solid: false },
        TileInfo { solid: true }
    ]);

    let mut map = Map::from_stream(tile_map, BufReader::new(File::open("tilemap.txt").unwrap())).unwrap();

    let mut game = GameState::new(map, vec![
        PlayerState::new("Test1".to_owned())
    ]);
    
    let p1 = game.get_player_by_name("Test1").unwrap().owner();

    let p1hq = game.new_structure(p1, Loc(1, 1), StructureType::HQ).unwrap();
    
    let wk_pos = game.get_structure(p1hq).unwrap().middle_point();
    let wk = game.new_unit(p1, wk_pos, UnitType::Worker).unwrap();
    
    loop {
        game.simulate(vec![vec![Command::MoveTo(wk, Vec2(500., 300.))]]);
        renderer.render(&game);
        thread::sleep(Duration::from_millis(500));
    }
}

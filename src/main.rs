#![feature(conservative_impl_trait)]

#[cfg(feature = "piston_renderer")]
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
mod ai_control;

#[cfg(feature = "piston_renderer")]
mod renderer;

use game::GameState;
use player::PlayerState;
use structure::StructureType;
use unit::UnitType;
use renderer::Renderer;
use loc::Loc;
use size::Size;
use map::Map;
use vec2::Vec2;
use command::Command;
use owner::Owner;
use ai_control::AIControl;

use std::thread;
use std::time::Duration;
use std::fs::File;
use std::sync::Arc;

use std::io::{self, BufReader};
use std::io::prelude::*;

fn main() {
    let mut renderer = Renderer::new(480, 480).unwrap();

    let mut map = Map::from_stream(BufReader::new(File::open("tilemap.txt").unwrap())).unwrap();

    let mut game = GameState::new(map, vec![
        PlayerState::new("Test1".to_owned())
    ]);

    let metal1 = game.new_structure(Owner::Neutral, Loc(5, 2), StructureType::Metal).unwrap();
    let metal2 = game.new_structure(Owner::Neutral, Loc(5, 3), StructureType::Metal).unwrap();
    let metal3 = game.new_structure(Owner::Neutral, Loc(4, 3), StructureType::Metal).unwrap();
    
    let p1 = game.get_player_by_name("Test1").unwrap().owner();

    let p1hq = game.new_structure(p1, Loc(1, 1), StructureType::HQ).unwrap();
    
    let wk_pos = game.get_structure(p1hq).unwrap().middle_point();
    let wk = game.new_unit(p1, wk_pos, UnitType::Worker).unwrap();
    
    let game_arc = Arc::new(game.clone());

    let mut ai = AIControl::new(
        Box::new(io::BufReader::new(io::stdin())),
        Box::new(io::stdout()),
        game_arc.clone());

    loop {
        let game_arc = Arc::new(game.clone());
        let cmds = ai.run(game_arc).unwrap();
        game.simulate(vec![cmds]);
        renderer.render(&game);
        thread::sleep(Duration::from_millis(500));
    }
}

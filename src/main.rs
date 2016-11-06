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
use std::process;

use std::io::{self, BufReader};
use std::io::prelude::*;

fn main() {
    let mut renderer = Renderer::new(480, 480).unwrap();

    let mut map = Map::from_stream(BufReader::new(File::open("tilemap2.txt").unwrap())).unwrap();

    let mut game = GameState::new(map, vec![
        PlayerState::new("Test1".to_owned()),
        PlayerState::new("Test2".to_owned())
    ]);

    let metal1 = game.new_structure(Owner::Neutral, Loc(7, 2), StructureType::Metal).unwrap();
    let metal2 = game.new_structure(Owner::Neutral, Loc(7, 3), StructureType::Metal).unwrap();
    let metal3 = game.new_structure(Owner::Neutral, Loc(7, 4), StructureType::Metal).unwrap();

    let metal4 = game.new_structure(Owner::Neutral, Loc(16, 22), StructureType::Metal).unwrap();
    let metal5 = game.new_structure(Owner::Neutral, Loc(16, 21), StructureType::Metal).unwrap();
    let metal6 = game.new_structure(Owner::Neutral, Loc(16, 20), StructureType::Metal).unwrap();
    
    let p1 = game.get_player_by_name("Test1").unwrap().owner();
    let p2 = game.get_player_by_name("Test2").unwrap().owner();

    let p1hq = game.new_structure(p1, Loc(1, 1), StructureType::HQ).unwrap();
    let p2hq = game.new_structure(p2, Loc(19, 20), StructureType::HQ).unwrap();
    
    let p1wk_pos = game.get_structure(p1hq).unwrap().middle_point();
    let p1wk = game.new_unit(p1, p1wk_pos, UnitType::Worker).unwrap();

    let p2wk_pos = game.get_structure(p2hq).unwrap().middle_point();
    let p2wk = game.new_unit(p2, p2wk_pos, UnitType::Worker).unwrap();
    
    let game_arc = Arc::new(game.clone());
    
    let mut p1p = process::Command::new("./test_ai.py")
                                   .stdin(process::Stdio::piped())
                                   .stdout(process::Stdio::piped())
                                   .spawn()
                                   .unwrap();

    let mut p1ai = AIControl::new(
        Box::new(io::BufReader::new(p1p.stdout.unwrap())),
        Box::new(p1p.stdin.unwrap()),
        0,
        game_arc.clone());

    let mut p2p = process::Command::new("./test_ai.py")
                                   .stdin(process::Stdio::piped())
                                   .stdout(process::Stdio::piped())
                                   .spawn()
                                   .unwrap();

    let mut p2ai = AIControl::new(
        Box::new(io::BufReader::new(p2p.stdout.unwrap())),
        Box::new(p2p.stdin.unwrap()),
        1,
        game_arc.clone());

    loop {
        let game_arc = Arc::new(game.clone());
        let p1cmds = p1ai.run(game_arc.clone()).unwrap();
        let p2cmds = p2ai.run(game_arc.clone()).unwrap();
        game.simulate(vec![p1cmds, p2cmds]);
        renderer.render(&game);
        thread::sleep(Duration::from_millis(200));
    }
}

use player::{PlayerState, PlayerId};
use structure::{Structure, StructureType, StructureId};
use unit::{Unit, UnitType, UnitId};
use command::Command;
use owner::Owner;
use vec2::Vec2;
use loc::Loc;
use size::Size;
use map::Map;

use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct GameState {
    players: Vec<PlayerState>,
    structures: BTreeMap<StructureId, Structure>,
    units: BTreeMap<UnitId, Unit>,
    last_structure_id: StructureId,
    last_unit_id: UnitId,
    pub map: Map
}

impl GameState {
    pub fn new(map: Map, mut players: Vec<PlayerState>) -> GameState {
        for (i, p) in players.iter_mut().enumerate() {
            p.id = i;
        }
        GameState {
            players: players,
            structures: BTreeMap::new(),
            units: BTreeMap::new(),
            last_structure_id: 1, // 0 is a special value
            last_unit_id: 1, // 0 is a special value
            map: map
        }
    }

    pub fn simulate(&mut self, commands: Vec<Vec<Command>>) {
        for (pid, cmds) in commands.into_iter().enumerate() {
            for cmd in cmds.into_iter() {
                match cmd {
                    Command::MoveTo(uid, pos) => {
                        // TODO
                    },
                    Command::Produce(sid, kind) => {
                        // TODO
                    },
                    Command::Build(uid, st, pos) => {
                        // TODO
                    }
                }
            }
        }
        // TODO: do actions, resolve collisions, etc
    }

    pub fn get_player_by_name<'a>(&'a self, name: &str) -> Option<&'a PlayerState> {
        self.players.iter().find(|p| p.name == name)
    }

    pub fn get_player_by_id<'a>(&'a self, id: PlayerId) -> Option<&'a PlayerState> {
        self.players.get(id)
    }

    pub fn new_structure(&mut self, owner: Owner, loc: Loc, kind: StructureType) -> Option<StructureId> {
        let s = Structure::new(owner, loc, kind);
        self.register_structure(s)
    }

    pub fn new_unit(&mut self, owner: Owner, pos: Vec2, kind: UnitType) -> Option<UnitId> {
        let u = Unit::new(owner, pos, kind);
        self.register_unit(u)
    }

    pub fn register_structure(&mut self, mut s: Structure) -> Option<StructureId> {
        // TODO: bounds checks
        let id = self.last_structure_id;
        self.last_structure_id += 1;
        s.id = id;
        self.structures.insert(id, s);
        Some(id)
    }

    pub fn register_unit(&mut self, mut u: Unit) -> Option<UnitId> {
        // TODO: bounds checks
        let id = self.last_unit_id;
        self.last_unit_id += 1;
        u.id = id;
        self.units.insert(id, u);
        Some(id)
    }

    pub fn get_structure(&self, sid: StructureId) -> Option<&Structure> {
        self.structures.get(&sid)
    }

    pub fn get_unit(&self, uid: UnitId) -> Option<&Unit> {
        self.units.get(&uid)
    }

    pub fn structures_iter<'a>(&'a self) -> impl Iterator<Item = &'a Structure> {
        self.structures.values()
    }

    pub fn units_iter<'a>(&'a self) -> impl Iterator<Item = &'a Unit> {
        self.units.values()
    }
}
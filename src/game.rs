use player::{PlayerState, PlayerId};
use structure::{Structure, StructureType, StructureId};
use unit::{Unit, UnitType, UnitId};
use command::Command;
use owner::Owner;
use vec2::Vec2;
use loc::Loc;
use size::Size;
use map::Map;

use std::io::prelude::*;
use std::io;

use std::collections::{BTreeMap, BTreeSet};

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
            let mut units_ordered = BTreeSet::new();
            let mut structures_ordered = BTreeSet::new();
            for cmd in cmds.into_iter() {
                match cmd {
                    Command::MoveTo(uid, pos) => {
                        if units_ordered.contains(&uid) {
                            continue;
                        }
                        units_ordered.insert(uid);
                        let u_opt = self.units.get_mut(&uid);
                        if let Some(u) = u_opt {
                            let next_pos = u.pos.move_to(pos, u.kind.speed());
                            if !self.map.is_solid(next_pos) {
                                u.pos = next_pos;
                            }
                        }
                    },
                    Command::Produce(sid, kind) => {
                        if structures_ordered.contains(&sid) {
                            continue;
                        }
                        structures_ordered.insert(sid);
                        // TODO
                    },
                    Command::Build(uid, st, pos) => {
                        if units_ordered.contains(&uid) {
                            continue;
                        }
                        units_ordered.insert(uid);
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

    pub fn dump_state<W: Write>(&self, w: &mut W) -> Result<(), io::Error> {
        try!(writeln!(w, "{}", self.structures.len()));
        for s in self.structures_iter() {
            try!(writeln!(w, "{} {} {} {} {} {} {}", s.id, s.kind.to_id(), s.owner.to_i64(), s.loc.0, s.loc.1, s.health, s.resources));
        }
        try!(writeln!(w, "{}", self.units.len()));
        for u in self.units_iter() {
            try!(writeln!(w, "{} {} {} {} {} {} {}", u.id, u.kind.to_id(), u.owner.to_i64(), u.pos.0.floor(), u.pos.1.floor(), u.health, u.resources));
        }
        Ok(())
    }
}

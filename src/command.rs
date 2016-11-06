use vec2::Vec2;
use unit::{UnitType, UnitId};
use structure::{StructureType, StructureId};

use std::str::FromStr;
use std::string::ToString;

#[derive(Copy, Clone, Debug)]
pub enum Command {
    MoveTo(UnitId, Vec2),
    Produce(StructureId, UnitType),
    Build(UnitId, StructureType, (i64, i64))
}

impl FromStr for Command {
    type Err = (); // TODO: might be discarding too much error information

    fn from_str(s: &str) -> Result<Command, Self::Err> {
        let mut iter = s.split(" ");
        let cmd = try!(iter.next().ok_or(()));
        Ok(match cmd {
            "MOVETO" => {
                let args = iter.take(3).collect::<Vec<_>>();
                if args.len() < 3 {
                    return Err(());
                }
                let uid: UnitId = try!(args[0].parse().map_err(|_| ()));
                let x: i64 = try!(args[1].parse().map_err(|_| ()));
                let y: i64 = try!(args[2].parse().map_err(|_| ()));
                Command::MoveTo(uid, Vec2(x as f64, y as f64))
            },
            "PRODUCE" => {
                let args = iter.take(2).collect::<Vec<_>>();
                if args.len() < 2 {
                    return Err(());
                }
                let sid: StructureId = try!(args[0].parse().map_err(|_| ()));
                let ut: UnitType = try!(args[1].parse().ok().and_then(UnitType::from_id).ok_or(()));
                Command::Produce(sid, ut)
            },
            "BUILD" => {
                let args = iter.take(4).collect::<Vec<_>>();
                if args.len() < 4 {
                    return Err(());
                }
                let uid: UnitId = try!(args[0].parse().map_err(|_| ()));
                let st: StructureType = try!(args[1].parse().ok().and_then(StructureType::from_id).ok_or(()));
                let x: i64 = try!(args[2].parse().map_err(|_| ()));
                let y: i64 = try!(args[3].parse().map_err(|_| ()));
                Command::Build(uid, st, (x, y))
            },
            _ => { return Err(()); }
        })
    }
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match *self {
            Command::MoveTo(uid, pos) => {
                format!("MOVETO {} {} {}", uid, pos.0.floor(), pos.1.floor())
            },
            Command::Produce(sid, ut) => {
                format!("PRODUCE {} {}", sid, ut.to_id())
            },
            Command::Build(uid, st, (x, y)) => {
                format!("BUILD {} {} {} {}", uid, st.to_id(), x, y)
            }
        }
    }
}

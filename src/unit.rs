use owner::Owner;
use vec2::Vec2;

pub type UnitId = usize;

#[derive(Clone, Debug)]
pub struct Unit {
    // Generic
    pub id: UnitId,
    pub kind: UnitType,
    pub owner: Owner,
    pub pos: Vec2,
    pub health: i64,
    
    // Specific
    pub resources: i64
}

impl Unit {
    pub fn new(owner: Owner, pos: Vec2, kind: UnitType) -> Unit {
        Unit {
            id: 0,
            kind: kind,
            owner: owner,
            pos: pos,
            health: kind.max_health(),
            resources: 0
        }
    }
}

pub type UnitTypeId = usize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnitType {
    Worker
}

impl UnitType {
    pub fn to_id(self) -> UnitTypeId {
        match self {
            UnitType::Worker => 0
        }
    }
    
    pub fn from_id(id: UnitTypeId) -> Option<UnitType> {
        Some(match id {
            0 => UnitType::Worker,
            _ => { return None; }
        })
    }

    pub fn max_health(self) -> i64 {
        match self {
            UnitType::Worker => 15
        }
    }

    pub fn radius(self) -> f64 {
        match self {
            UnitType::Worker => 15.
        }
    }

    pub fn speed(self) -> f64 {
        match self {
            UnitType::Worker => 10.
        }
    }
}

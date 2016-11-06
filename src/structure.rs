use owner::Owner;
use loc::Loc;
use size::Size;

pub type StructureId = usize;

#[derive(Clone, Debug)]
pub struct Structure {
    // Generic
    pub id: StructureId,
    pub kind: StructureType,
    pub owner: Owner,
    pub loc: Loc,
    pub health: i64,

    // Specific
    pub resources: i64
}

impl Structure {
    pub fn new(owner: Owner, loc: Loc, kind: StructureType) -> Structure {
        Structure {
            id: 0,
            owner: owner,
            kind: kind,
            loc: loc,
            health: kind.max_health(),
            resources: kind.resources()
        }
    }
}

pub type StructureTypeId = usize;

#[derive(Copy, Clone, Debug)]
pub enum StructureType {
    HQ,
    Outpost,
    Metal
}

impl StructureType {
    pub fn to_id(self) -> StructureTypeId {
        match self {
            StructureType::HQ => 0,
            StructureType::Outpost => 1,
            StructureType::Metal => 2
        }
    }

    pub fn from_id(id: StructureTypeId) -> Option<StructureType> {
        Some(match id {
            0 => StructureType::HQ,
            1 => StructureType::Outpost,
            2 => StructureType::Metal,
            _ => { return None; }
        })
    }

    pub fn size(self) -> Size {
        match self {
            StructureType::HQ => Size(3, 2),
            StructureType::Outpost => Size(2, 2),
            StructureType::Metal => Size(1, 1)
        }
    }

    pub fn max_health(self) -> i64 {
        match self {
            StructureType::HQ => 350,
            StructureType::Outpost => 150,
            StructureType::Metal => 0
        }
    }

    pub fn resources(self) -> i64 {
        match self {
            StructureType::Metal => 500,
            _ => 0
        }
    }
}

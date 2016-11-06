use owner::Owner;
use loc::Loc;
use size::Size;
use vec2::Vec2;

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

    pub fn top_left(&self) -> Vec2 {
        self.loc.top_left()
    }

    pub fn bottom_right(&self) -> Vec2 {
        self.loc.top_left() + self.kind.size().to_vec()
    }

    pub fn middle_point(&self) -> Vec2 {
        let tl = self.top_left();
        let off = self.kind.size().to_vec() * 0.5;
        tl + off
    }

    pub fn contains_pos(&self, pos: Vec2) -> bool {
        let tl = self.top_left();
        let br = self.bottom_right();
        pos.0 >= tl.0 && pos.1 >= tl.1 && pos.0 <= br.0 && pos.1 <= br.1
    }
}

pub type StructureTypeId = usize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

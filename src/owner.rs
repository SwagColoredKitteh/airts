use player::PlayerId;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Owner {
    Neutral,
    Player(PlayerId)
}

impl Owner {
    pub fn is_neutral(self) -> bool {
        self == Owner::Neutral
    }

    pub fn is_player(self) -> bool {
        if let Owner::Player(_) = self {
            true
        }
        else {
            false
        }
    }

    pub fn player_id(self) -> Option<PlayerId> {
        if let Owner::Player(id) = self {
            Some(id)
        }
        else {
            None
        }
    }
}

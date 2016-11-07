use owner::Owner;

pub type PlayerId = usize;

#[derive(Clone, Debug)]
pub struct PlayerState {
    pub id: PlayerId,
    pub name: String,
    pub metal: i64
}

impl PlayerState {
    pub fn new(name: String) -> PlayerState {
        PlayerState {
            id: 0,
            name: name,
            metal: 100 // TODO: temporary measure until the GameState has proper mutability here
        }
    }

    pub fn owner(&self) -> Owner {
        Owner::Player(self.id)
    }
}

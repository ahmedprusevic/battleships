use crate::game::fleet::Fleet;

pub struct Player {
    pub name: String,
    pub fleet: Fleet,
    pub num_of_hits: u8,
}

impl Player {
    pub fn new(name: &str, fleet: Fleet) -> Player {
        Player {
            name: name.to_string(),
            fleet,
            num_of_hits: 0,
        }
    }
}

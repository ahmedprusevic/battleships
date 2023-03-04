use crate::game::fleet::Fleet;

pub struct Player {
    name: String,
    fleet: Fleet,
}

impl Player {
    pub fn new (name: &str, fleet: Fleet) -> Player {
        Player {
            name: name.to_string(),
            fleet,
        }
    }
}

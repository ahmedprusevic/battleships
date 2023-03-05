use crate::game::fleet::Fleet;

pub struct Player {
   pub name: String,
   pub fleet: Fleet,
}

impl Player {
    pub fn new (name: &str, fleet: Fleet) -> Player {
        Player {
            name: name.to_string(),
            fleet,
        }
    }
}

use crate::game::ship::Ship;

struct Player {
    name: String,
    ships: [Ship; 5],
}

impl Player {
    fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            ships: [Ship::new(3), Ship::new(5), Ship::new(4), Ship::new(4), Ship::new(3)],
        }
    }
}

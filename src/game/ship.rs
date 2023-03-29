pub struct Position(pub String, pub i8);

pub struct BoardPosition(pub Position, pub Position);

#[derive(Debug)]
pub enum ShipName {
    Poseidon,
    Kraken,
    Hydra,
    Phoenix,
    Medusa,
}

pub struct Ship {
    pub length: i8,
    pub position: BoardPosition,
    pub name: ShipName,
}

impl Ship {
    pub fn new(length: i8, name: ShipName) -> Ship {
        let position = BoardPosition(Position(String::new(), 0), Position(String::new(), 0));
        Ship {
            length,
            position,
            name,
        }
    }

    pub fn set_ship_position(&mut self, position: BoardPosition) {
        self.position = position
    }
}

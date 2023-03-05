

pub struct Position(pub String,pub u8);

pub struct BoardPosition(pub Position,pub Position);


pub struct Ship {
   pub length: u8,
   pub(crate) position: BoardPosition
}


impl Ship {
    pub fn new (length: u8) -> Ship {
        let position = BoardPosition(Position(String::new(), 0), Position(String::new(), 0));
        Ship {
            length,
            position
        }
    }

    pub fn set_ship_position(&mut self, position: BoardPosition) {
        self.position = position
    }
}

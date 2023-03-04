

pub(crate) struct Position(pub(crate) String,pub(crate) u8);

pub(crate) struct BoardPosition(pub(crate) Position,pub(crate) Position);


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
}

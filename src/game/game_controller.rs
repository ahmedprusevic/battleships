use crate::game::board::Board;
use crate::game::ship::{BoardPosition, Position, Ship};
use crate::game::fleet::Fleet;
use crate::game::player::Player;

pub struct GameController {
    player1: Player,
    player2: Player,
    board: Board,
}

impl GameController {
   pub fn new() -> GameController {
        let fleet_player1 = Fleet::new([Ship::new(5), Ship::new(4), Ship::new(4), Ship::new(3), Ship::new(2)]);
        let fleet_player2 = Fleet::new([Ship::new(5), Ship::new(4), Ship::new(4), Ship::new(3), Ship::new(2)]);
        let player1 = Player::new("Ahmed", fleet_player1);
        let player2 = Player::new("Suad", fleet_player2);
        let board = Board::new();
        GameController {
            player1,
            player2,
            board
        }
    }

    pub fn start_game(&mut self) {
        self.board.display_board();
        self.player1.fleet.ships[0].set_ship_position(BoardPosition(Position(String::from("A"), 1), Position(String::from("C"), 1)));
        self.board.place_ship_on_board(&self.player1.fleet.ships[0]);
        self.board.display_board();
    }
}

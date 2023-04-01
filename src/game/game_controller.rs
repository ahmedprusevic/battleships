use crate::game::board::Board;
use crate::game::fleet::{Fleet, FLEET_SIZE};
use crate::game::player::Player;
use crate::game::ship::{BoardPosition, Position, Ship, ShipName};

use crate::utils::inputs::{parse_input, read_player_input};
use std::io;
use unicode_segmentation::UnicodeSegmentation;

pub struct CurrentPlayer(pub u8);

pub struct GameController {
    player1: Player,
    player2: Player,
    board: Board,
    current_player: CurrentPlayer,
}

impl GameController {
    pub fn new() -> GameController {
        let player1 = Player::new("Ahmed", Fleet::new());
        let player2 = Player::new("Suad", Fleet::new());
        let board = Board::new();
        GameController {
            player1,
            player2,
            board,
            current_player: CurrentPlayer(1),
        }
    }

    pub fn start_game(&mut self) {
        self.board.display_board();

        let mut current_ship: usize = 0;

        while current_ship < FLEET_SIZE {
            println!("Please enter a ship 🚢 position please take note that you are currently placing {:?} , who has length of {}", self.player1.fleet.ships[current_ship].name, self.player1.fleet.ships[current_ship].length);

            let first_position = take_position();
            let second_position_result =
                self.board.calculate_available_positions_and_take_position(
                    &first_position,
                    &self.player1.fleet.ships[current_ship].length,
                    &self.current_player,
                );

            match second_position_result {
                Ok(second_position) => self.player1.fleet.ships[current_ship]
                    .set_ship_position(BoardPosition(first_position, second_position)),
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            let is_ship_placed = self
                .board
                .place_ship_on_board(&self.player1.fleet.ships[current_ship]);

            match is_ship_placed {
                Ok(_) => self.board.display_board(),
                Err(e) => {
                    println!("{}", e);
                    println!("Please enter correct position");
                    continue;
                }
            }

            current_ship += 1;
        }
    }
}

fn take_position() -> Position {
    let player_input = read_player_input();
    let position = parse_input(player_input).expect("Please enter a valid input eg. A1");

    Position(position.0, position.1)
}

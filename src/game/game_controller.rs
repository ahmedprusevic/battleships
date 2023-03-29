use crate::game::board::Board;
use crate::game::fleet::{Fleet, FLEET_SIZE};
use crate::game::player::Player;
use crate::game::ship::{BoardPosition, Position, Ship, ShipName};

use crate::utils::inputs::read_player_input;
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
            self.board.calculate_available_positions(
                &first_position,
                self.player1.fleet.ships[current_ship].length,
                &self.current_player,
            );

            let second_position = take_position();
            self.player1.fleet.ships[current_ship]
                .set_ship_position(BoardPosition(first_position, second_position));

            let is_ship_placed = self
                .board
                .place_ship_on_board(&self.player1.fleet.ships[current_ship]);

            match is_ship_placed {
                Ok(_) => self.board.display_board(),
                Err(e) => {
                    println!(
                        "Failed to place ship on board because of following error: {}",
                        e
                    );
                    println!("Please enter correct position");
                    continue;
                }
            }

            current_ship += 1;
        }
    }
}

fn take_position() -> Position {
    let mut player_input = read_player_input();
    io::stdin()
        .read_line(&mut player_input)
        .expect("Failed to read line!");

    let letter_part = player_input
        .chars()
        .nth(0)
        .expect("Please enter one letter and at least one number eg: 'A1'");
    let first_number_part = player_input
        .chars()
        .nth(1)
        .expect("Please enter one letter and at least one number eg: 'A1'");

    let player_selected_number: i8;

    if player_input.graphemes(true).count() >= 4 {
        let second_number_part = player_input.chars().nth(2).unwrap();
        player_selected_number = format!("{}{}", first_number_part, second_number_part)
            .parse()
            .expect("You haven't entered a valid number, please enter a valid number!");
    } else {
        player_selected_number = first_number_part
            .to_digit(10)
            .expect("You haven't entered a valid number, please enter a valid number!")
            as i8;
    }

    Position(letter_part.to_string(), player_selected_number)
}

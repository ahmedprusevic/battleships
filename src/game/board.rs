use std::fmt::Error;
use crate::game::error::ShipInputError;
use crate::game::ship::{Position, Ship};

const LETTERS: [&str; 14] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N"];

pub struct Board {
    pub player_1_fields: [[BoardState; 14]; 14],
    pub player_2_fields: [[BoardState; 14]; 14],
}

#[derive(Clone, Copy, Debug)]
pub enum BoardState {
    Free = 0,
    Shot = 1,
    Miss = 2,
    Ship = 3
}

impl Board {
    pub fn new() -> Board {
        Board {
            player_1_fields: [[BoardState::Free; 14]; 14],
            player_2_fields: [[BoardState::Free; 14]; 14],
        }
    }
    pub fn display_board(&self) {
        println!("  1   2   3   4   5   6   7   8   9   10  11  12  13  14");
        self.display_players_board(&self.player_1_fields);
        println!("--------------------------------------------------------------------");
        self.display_players_board(&self.player_2_fields);
        println!("  1   2   3   4   5   6   7   8   9   10  11  12  13  14");
    }

    fn display_players_board(&self, &board: &[[BoardState; 14]; 14]) {
        for (i, arr) in board.iter().enumerate() {
            for (j, field) in arr.iter().enumerate() {
                let mut first_letter = String::new();
                let mut last_letter = String::new();

                if j == 0 {
                    first_letter += LETTERS[i]
                }
                if j == 13 {
                    last_letter += LETTERS[i]
                }

                match field {
                    BoardState::Free => print!("{} 🟩 {}", first_letter, last_letter),
                    BoardState::Miss => print!("{} ❎ {}", first_letter, last_letter),
                    BoardState::Shot => print!("{} 💢 {}", first_letter, last_letter),
                    BoardState::Ship => print!("{} 🚢 {}", first_letter, last_letter),
                }
            }
            println!();
        }
    }

    fn place_ship_on_board(&mut self, ship: &Ship) -> Result<bool, ShipInputError> {
        let starting_point = &ship.position.0;
        let end_point = &ship.position.1;

        let is_position_valid = check_if_position_is_valid(&starting_point, &end_point);

    }
}

fn check_if_position_is_valid(position1: &Position, position2: &Position) -> Result<bool, ShipInputError> {
    if position1.0 == position2.0 {
        return Ok(true);
    }
    if position1.1 == position2.1 {
        return Ok(true);
    }

    Err(ShipInputError::InvalidInput)
}


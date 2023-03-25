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

    pub fn place_ship_on_board(&mut self, ship: &Ship) -> Result<(), ShipInputError> {
        let starting_point = &ship.position.0;
        let end_point = &ship.position.1;

        check_if_position_is_valid(&starting_point, &end_point)?;

        let starting_point_letter_to_idx = LETTERS.iter().position(|&x| x == starting_point.0).unwrap();
        let end_point_letter_to_idx = LETTERS.iter().position(|&x| x == end_point.0).unwrap();

        let starting_point_num_to_idx = starting_point.1 - 1;
        let end_point_num_to_idx = end_point.1 - 1;

        match (starting_point_letter_to_idx == end_point_letter_to_idx, starting_point_num_to_idx == end_point_num_to_idx) {
            (true, false) => {
                let mut placing_idx = starting_point_num_to_idx as usize;
                let mut step_num = 1;
                while step_num <= ship.length as usize {
                    self.player_1_fields[starting_point_letter_to_idx][placing_idx] = BoardState::Ship;
                    placing_idx += 1;
                    step_num += 1;
                }
            },
            (false, true) => {
                let mut placing_idx = starting_point_letter_to_idx;
                let mut step_num = 1;
                while step_num <= ship.length as usize {
                    self.player_1_fields[placing_idx][starting_point_num_to_idx as usize] = BoardState::Ship;
                    placing_idx += 1;
                    step_num += 1;
                }
            },
            _ => unreachable!(),
        }

        Ok(())
    }
}

fn check_if_position_is_valid(position1: &Position, position2: &Position) -> Result<(), ShipInputError> {
    match (position1.0 == position2.0, position1.1 == position2.1) {
        (true, false) | (false, true) => Ok(()),
        _ => Err(ShipInputError::InvalidInput),
    }
}


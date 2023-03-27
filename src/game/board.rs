use crate::game::error::ShipInputError;
use crate::game::game_controller::CurrentPlayer;
use crate::game::ship::{Position, Ship};
use std::collections::HashMap;

const LETTERS: [&str; 14] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N",
];

const X_POSITIVE: &str = "x-positive";
const X_NEGATIVE: &str = "x-negative";
const Y_POSITIVE: &str = "y-positive";
const Y_NEGATIVE: &str = "y-negative";

pub struct Board {
    pub player_1_fields: [[BoardState; 14]; 14],
    pub player_2_fields: [[BoardState; 14]; 14],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardState {
    Free = 0,
    Shot = 1,
    Miss = 2,
    Ship = 3,
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

        let starting_point_letter_to_idx =
            LETTERS.iter().position(|&x| x == starting_point.0).unwrap();
        let end_point_letter_to_idx = LETTERS.iter().position(|&x| x == end_point.0).unwrap();

        let starting_point_num_to_idx = starting_point.1 - 1;
        let end_point_num_to_idx = end_point.1 - 1;

        match (
            starting_point_letter_to_idx == end_point_letter_to_idx,
            starting_point_num_to_idx == end_point_num_to_idx,
        ) {
            (true, false) => {
                let mut placing_idx = starting_point_num_to_idx as usize;
                let mut step_num = 1;
                while step_num <= ship.length as usize {
                    self.player_1_fields[starting_point_letter_to_idx][placing_idx] =
                        BoardState::Ship;
                    placing_idx += 1;
                    step_num += 1;
                }
            }
            (false, true) => {
                let mut placing_idx = starting_point_letter_to_idx;
                let mut step_num = 1;
                while step_num <= ship.length as usize {
                    self.player_1_fields[placing_idx][starting_point_num_to_idx as usize] =
                        BoardState::Ship;
                    placing_idx += 1;
                    step_num += 1;
                }
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn calculate_available_positions(
        &self,
        position: &Position,
        ship_length: u8,
        current_player: &CurrentPlayer,
    ) {
        match current_player {
            CurrentPlayer(1) => {
                display_available_fields(&self.player_1_fields, position, ship_length)
            }
            CurrentPlayer(2) => {
                display_available_fields(&self.player_2_fields, position, ship_length)
            }
            _ => unreachable!(),
        }
    }
}

fn display_available_fields(fields: &[[BoardState; 14]; 14], position: &Position, ship_length: u8) {
    let starting_point_letter_to_idx = LETTERS.iter().position(|&x| x == position.0).unwrap() as i8;
    let starting_point_num_to_idx = (position.1 - 1) as i8;
    let mut available_placements = HashMap::new();

    available_placements.insert(
        X_POSITIVE,
        format!(
            "{}{} - {}{}",
            position.0,
            position.1,
            position.0,
            position.1 + ship_length
        ),
    );

    available_placements.insert(
        X_NEGATIVE,
        format!(
            "{}{} - {}{}",
            position.0,
            position.1,
            position.0,
            position.1 - ship_length
        ),
    );

    available_placements.insert(
        Y_POSITIVE,
        format!(
            "{}{} - {}{}",
            position.0,
            position.1,
            LETTERS[(starting_point_letter_to_idx + ship_length as i8) as usize],
            position.1
        ),
    );

    available_placements.insert(
        Y_NEGATIVE,
        format!(
            "{}{} - {}{}",
            position.0,
            position.1,
            LETTERS[(starting_point_letter_to_idx - ship_length as i8) as usize],
            position.1
        ),
    );

    {
        //     x-axis positive
        let mut i = starting_point_num_to_idx.clone();
        let possible_last_place = starting_point_num_to_idx + ship_length as i8;

        if possible_last_place > 14 {
            available_placements.insert(X_POSITIVE, String::new());
        }

        while (i <= possible_last_place) && (possible_last_place < 14) {
            if fields[starting_point_letter_to_idx as usize][i as usize] != BoardState::Free {
                available_placements.insert(X_POSITIVE, String::new());
                break;
            }
            i += 1;
        }
    }
    {
        //     x-axis negative
        let mut i = starting_point_num_to_idx.clone();
        let possible_last_place: i8 = starting_point_num_to_idx - ship_length as i8;

        if possible_last_place < 0 {
            available_placements.insert(X_NEGATIVE, String::new());
        }

        while (i >= 0) && (possible_last_place >= 0) {
            if fields[starting_point_letter_to_idx as usize][i as usize] != BoardState::Free {
                available_placements.insert(X_NEGATIVE, String::new());
                break;
            }
            i -= 1;
        }
    }
    {
        //     y-axis positive
        let mut i = starting_point_letter_to_idx.clone();
        let possible_last_place = starting_point_letter_to_idx + ship_length as i8;

        if possible_last_place > 14 {
            available_placements.insert(Y_POSITIVE, String::new());
        }

        while (i <= possible_last_place) && (possible_last_place > 0) {
            if fields[i as usize][starting_point_num_to_idx as usize] != BoardState::Free {
                available_placements.insert(Y_POSITIVE, String::new());
                break;
            }
            i += 1;
        }
    }
    {
        //     y-axis negative
        let mut i = starting_point_letter_to_idx.clone();
        let possible_last_place = starting_point_letter_to_idx + ship_length as i8;

        if possible_last_place < 0 {
            available_placements.insert(Y_NEGATIVE, String::new());
        }

        while (i >= 0) && (possible_last_place >= 0) {
            if fields[i as usize][starting_point_num_to_idx as usize] != BoardState::Free {
                available_placements.insert(Y_NEGATIVE, String::new());
                break;
            }
            i -= 1;
        }
    }
}

fn check_if_position_is_valid(
    position1: &Position,
    position2: &Position,
) -> Result<(), ShipInputError> {
    match (position1.0 == position2.0, position1.1 == position2.1) {
        (true, false) | (false, true) => Ok(()),
        _ => Err(ShipInputError::InvalidInput),
    }
}

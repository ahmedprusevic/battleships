use crate::game::error::ShipInputError;
use crate::game::game_controller::CurrentPlayer;
use crate::game::ship::{Position, Ship};
use crate::utils::conversions::convert_string_to_number;
use dialoguer::{theme::ColorfulTheme, Select};
use std::collections::HashMap;

const LETTERS: [&str; 14] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N",
];

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

    pub fn check_shot(
        &mut self,
        position: Position,
        current_player: &CurrentPlayer,
    ) -> Result<BoardState, ShipInputError> {
        let position_letter_to_idx = LETTERS.iter().position(|&x| x == position.0).unwrap();
        let position_num_to_idx = position.1 - 1;

        let field = &mut self.get_fields_based_on_current_player(current_player)
            [position_letter_to_idx][position_num_to_idx as usize];

        match *field {
            BoardState::Free => {
                self.set_field(
                    current_player,
                    position_letter_to_idx,
                    position_num_to_idx as usize,
                    BoardState::Miss,
                );
                Ok(BoardState::Miss)
            }
            BoardState::Miss => Err(ShipInputError::AlreadyShotError),
            BoardState::Ship => {
                self.set_field(
                    current_player,
                    position_letter_to_idx,
                    position_num_to_idx as usize,
                    BoardState::Shot,
                );
                Ok(BoardState::Shot)
            }
            BoardState::Shot => Err(ShipInputError::AlreadyShotError),
        }
    }

    fn set_field(
        &mut self,
        current_player: &CurrentPlayer,
        letter_idx: usize,
        num_idx: usize,
        state: BoardState,
    ) {
        match current_player {
            CurrentPlayer::One => self.player_1_fields[letter_idx][num_idx] = state,
            CurrentPlayer::Two => self.player_2_fields[letter_idx][num_idx] = state,
        }
    }

    fn get_fields_based_on_current_player(
        &self,
        current_player: &CurrentPlayer,
    ) -> [[BoardState; 14]; 14] {
        match current_player {
            CurrentPlayer::One => self.player_1_fields,
            CurrentPlayer::Two => self.player_2_fields,
        }
    }

    pub fn place_ship_on_board(
        &mut self,
        ship: &Ship,
        current_player: &CurrentPlayer,
    ) -> Result<(), ShipInputError> {
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
                let mut placing_idx = if starting_point_num_to_idx > end_point_num_to_idx {
                    end_point_num_to_idx as usize
                } else {
                    starting_point_num_to_idx as usize
                };
                let mut step_num = 1;

                while step_num <= ship.length as usize {
                    self.get_fields_based_on_current_player(current_player)
                        [starting_point_letter_to_idx][placing_idx] = BoardState::Ship;

                    placing_idx += 1;
                    step_num += 1;
                }
            }
            (false, true) => {
                let mut placing_idx = if starting_point_letter_to_idx > end_point_letter_to_idx {
                    end_point_letter_to_idx
                } else {
                    starting_point_letter_to_idx
                };
                let mut step_num = 1;
                while step_num <= ship.length as usize {
                    match current_player {
                        CurrentPlayer::One => {
                            self.player_1_fields[placing_idx][starting_point_num_to_idx as usize] =
                                BoardState::Ship
                        }
                        CurrentPlayer::Two => {
                            self.player_2_fields[placing_idx][starting_point_num_to_idx as usize] =
                                BoardState::Ship
                        }
                    }

                    placing_idx += 1;
                    step_num += 1;
                }
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn calculate_available_positions_and_take_position(
        &self,
        position: &Position,
        ship_length: i8,
        current_player: &CurrentPlayer,
    ) -> Result<Position, ShipInputError> {
        match current_player {
            CurrentPlayer::One => {
                take_second_position(&self.player_1_fields, position, ship_length - 1)
            }
            CurrentPlayer::Two => {
                take_second_position(&self.player_2_fields, position, ship_length - 1)
            }
        }
    }
}

fn take_second_position(
    fields: &[[BoardState; 14]; 14],
    position: &Position,
    ship_length: i8,
) -> Result<Position, ShipInputError> {
    let starting_point_letter_to_idx = LETTERS.iter().position(|&x| x == position.0).unwrap() as i8;
    let starting_point_num_to_idx = (position.1 - 1) as i8;
    let mut available_placements = HashMap::new();

    available_placements.insert(
        0,
        format!(
            "{}{} - {}{}",
            position.0,
            position.1,
            position.0,
            position.1 + ship_length
        ),
    );

    available_placements.insert(
        1,
        format!(
            "{}{} - {}{}",
            position.0,
            position.1,
            position.0,
            position.1 - ship_length
        ),
    );

    available_placements.insert(
        2,
        format!(
            "{}{} - {}{}",
            position.0,
            position.1,
            LETTERS
                .get((starting_point_letter_to_idx + ship_length) as usize)
                .unwrap_or(&""),
            position.1
        ),
    );

    available_placements.insert(
        3,
        format!(
            "{}{} - {}{}",
            position.0,
            position.1,
            LETTERS
                .get((starting_point_letter_to_idx - ship_length) as usize)
                .unwrap_or(&""),
            position.1
        ),
    );

    {
        //     x-axis positive
        let mut i = starting_point_num_to_idx.clone();
        let possible_last_place = starting_point_num_to_idx + ship_length as i8;

        if possible_last_place > 13 {
            available_placements.remove(&0);
        }

        while (i <= possible_last_place) && (possible_last_place < 14) {
            if fields[starting_point_letter_to_idx as usize][i as usize] != BoardState::Free {
                available_placements.remove(&0);
                break;
            }
            i += 1;
        }
    }
    {
        //     x-axis negative
        let mut i = starting_point_num_to_idx.clone();
        let possible_last_place = starting_point_num_to_idx - ship_length as i8;

        if possible_last_place < 0 {
            available_placements.remove(&1);
        }

        while (i >= possible_last_place) && (possible_last_place >= 0) {
            if fields[starting_point_letter_to_idx as usize][i as usize] != BoardState::Free {
                available_placements.remove(&1);
                break;
            }
            i -= 1;
        }
    }
    {
        //     y-axis positive
        let mut i = starting_point_letter_to_idx.clone();
        let possible_last_place = starting_point_letter_to_idx + ship_length as i8;

        if possible_last_place > 13 {
            available_placements.remove(&2);
        }

        while (i <= possible_last_place) && (possible_last_place < 14) {
            if fields[i as usize][starting_point_num_to_idx as usize] != BoardState::Free {
                available_placements.remove(&2);
                break;
            }
            i += 1;
        }
    }
    {
        //     y-axis negative
        let mut i = starting_point_letter_to_idx.clone();
        let possible_last_place = starting_point_letter_to_idx - ship_length as i8;

        if possible_last_place < 0 {
            available_placements.remove(&3);
        }

        while (i >= possible_last_place) && (possible_last_place >= 0) {
            if fields[i as usize][starting_point_num_to_idx as usize] != BoardState::Free {
                available_placements.remove(&3);
                break;
            }
            i -= 1;
        }
    }

    let vector_of_available_values = Vec::from_iter(available_placements.values());

    if vector_of_available_values.len() == 0 {
        return Err(ShipInputError::NoFreeFields);
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&vector_of_available_values)
        .default(0)
        .interact()
        .unwrap();

    let selected_position_pair: &String = vector_of_available_values[selection];

    let position_tuple = convert_string_to_number(selected_position_pair);

    Ok(Position(
        position_tuple.1.to_string(),
        position_tuple.0 as i8,
    ))
}

fn check_if_position_is_valid(
    position1: &Position,
    position2: &Position,
) -> Result<(), ShipInputError> {
    println!(
        "Position letters {:?} , position numbers {:?}",
        position1, position2
    );
    match (position1.0 == position2.0, position1.1 == position2.1) {
        (true, false) | (false, true) => Ok(()),
        _ => Err(ShipInputError::SameFieldError),
    }
}

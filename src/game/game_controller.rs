use crate::game::board::{Board, BoardState};
use crate::game::fleet::{Fleet, FLEET_SIZE};
use crate::game::player::Player;
use crate::game::ship::{BoardPosition, Position};

use crate::utils::inputs::{parse_input, read_player_input};

#[derive(Debug)]
pub enum CurrentPlayer {
    One,
    Two,
}

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
            current_player: CurrentPlayer::One,
        }
    }

    pub fn start_game(&mut self) {
        let finishing_number = self.player1.fleet.get_fleet_size();
        self.board.display_board();
        self.place_players_fleet();
        self.change_player();
        self.place_players_fleet();

        while self.player1.num_of_hits != finishing_number as u8
            || self.player2.num_of_hits != finishing_number as u8
        {
            println!("Player {} turn", self.get_current_player().name);
            self.take_shot();
            self.board.display_board();
        }
    }
    fn change_player(&mut self) {
        self.current_player = match self.current_player {
            CurrentPlayer::One => CurrentPlayer::Two,
            CurrentPlayer::Two => CurrentPlayer::One,
        }
    }

    fn take_shot(&mut self) {
        println!("Please take a shot (eg A1)");
        let position_to_shoot = take_position();
        let shot_result = self
            .board
            .check_shot(position_to_shoot, &self.current_player);

        match shot_result {
            Ok(BoardState::Shot) => {
                self.get_current_player_mut().num_of_hits += 1;
                self.change_player()
            }
            Ok(BoardState::Miss) => self.change_player(),
            Err(e) => {
                println!("{}", e);
                self.take_shot()
            }
            _ => unreachable!(),
        }
    }

    fn get_current_player_mut(&mut self) -> &mut Player {
        match self.current_player {
            CurrentPlayer::One => &mut self.player1,
            CurrentPlayer::Two => &mut self.player2,
        }
    }

    fn get_current_player(&self) -> &Player {
        match self.current_player {
            CurrentPlayer::One => &self.player1,
            CurrentPlayer::Two => &self.player2,
        }
    }

    fn place_players_fleet(&mut self) {
        let mut current_ship: usize = 0;

        while current_ship < FLEET_SIZE {
            println!("Please enter a ship 🚢 position please take note that you are currently placing {:?} , who has length of {}", self.get_current_player().fleet.ships[current_ship].name, self.get_current_player().fleet.ships[current_ship].length);

            let first_position = take_position();
            let second_position_result =
                self.board.calculate_available_positions_and_take_position(
                    &first_position,
                    self.get_current_player().fleet.ships[current_ship].length,
                    &self.current_player,
                );

            match second_position_result {
                Ok(second_position) => self.get_current_player_mut().fleet.ships[current_ship]
                    .set_ship_position(BoardPosition(first_position, second_position)),
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            let is_ship_placed = match self.current_player {
                CurrentPlayer::One => self.board.place_ship_on_board(
                    &self.player1.fleet.ships[current_ship],
                    &self.current_player,
                ),
                CurrentPlayer::Two => self.board.place_ship_on_board(
                    &self.player2.fleet.ships[current_ship],
                    &self.current_player,
                ),
            };

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

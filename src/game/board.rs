

pub struct Board {
    pub player_1_fields: [[BoardState; 14]; 14],
    pub player_2_fields: [[BoardState; 14]; 14],
}

#[derive(Clone, Copy, Debug)]
pub enum BoardState {
    Free = 0,
    Shot = 1,
    Miss = 2,
}

impl Board {
    pub fn new() -> Board {
        Board {
            player_1_fields: [[BoardState::Free; 14]; 14],
            player_2_fields: [[BoardState::Free; 14]; 14],
        }
    }
    pub fn display_board(&self) {
        for arr in self.player_1_fields {
            for field in arr {
                match field {
                    BoardState::Free => print!("🟩"),
                    BoardState::Miss => print!("❎"),
                    BoardState::Shot => print!("💢"),
                }
            }
            println!();
        }
        for arr in self.player_2_fields {
            for field in arr {
                match field {
                    BoardState::Free => print!("🟩"),
                    BoardState::Miss => print!("❎"),
                    BoardState::Shot => print!("💢"),
                }
            }
            println!();
        }
    }

    // fn display_field(&self) {
    //     match self.player_1_fields { }
    // }
}


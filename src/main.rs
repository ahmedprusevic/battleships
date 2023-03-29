use crate::game::game_controller::GameController;

mod game;
mod utils;

fn main() {
    let mut game_controller = GameController::new();
    game_controller.start_game()
}

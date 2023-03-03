mod game;

use game::board::Board;

fn main() {
    let board = Board::new();

    board.display_board();
}

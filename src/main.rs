// main function 
// orchestrates the main conduction of the things.
mod board;
mod constants;
mod display;
mod log_move;
mod pieces;


fn main() {
    let b = board::Board::new_board();
    display::display_board(&b); 
}
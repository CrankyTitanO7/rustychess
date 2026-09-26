// mod constants;
use crate::constants as C;
use crate::board as B;

// displays an 8x8 array of strings
pub fn display_board (b : &B::Board) {
    // board.board is indexed [y][x] (row-major); print rank 8 first.
    for y in (0..C::YWIDTH).rev() {
        for x in 0.. C::XWIDTH {
            match &b.board[y][x] {
                Some (p) => print!("{} ", p.symbol),
                None => print!(". ")
            };
        }
        print!("\n");
    }
}



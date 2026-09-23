// mod constants;
use crate::constants as C;
use crate::board as B;

// displays an 8x8 array of strings
pub fn display_board (b : B::Board) {
    let board_array = b.board; 
    
    for i in 0 .. C::XWIDTH {
        for j in 0.. C::YWIDTH {
            match &board_array[i][j] {
                Some (p) => print!("{}", p.symbol),
                None => print!(".")
            };
        }
        print!("\n");
    }
}



// mod constants;
use crate::constants as C;

// displays an 8x8 array of strings
fn display_board (board_array: &[[&str; C::XWIDTH]; C::YWIDTH]) {
    for i in 0 .. C::XWIDTH {
        for j in 0.. C::YWIDTH {
            print!("{}", board_array[i][j]);
        }
        print!("\n");
    }
}



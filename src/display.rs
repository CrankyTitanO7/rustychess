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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants as C;

    #[test]
    fn board_dimensions_match_constants() {
        let board: [[&str; C::XWIDTH]; C::YWIDTH] = [["."; C::XWIDTH]; C::YWIDTH];
        assert_eq!(board.len(), C::YWIDTH);
        assert_eq!(board[0].len(), C::XWIDTH);
    }

    #[test]
    fn display_board_does_not_panic_on_empty_board() {
        let board: [[&str; C::XWIDTH]; C::YWIDTH] = [["."; C::XWIDTH]; C::YWIDTH];
        display_board(&board);
    }

    #[test]
    fn board_corners_are_indexable() {
        let mut board: [[&str; C::XWIDTH]; C::YWIDTH] = [["."; C::XWIDTH]; C::YWIDTH];
        board[0][0] = "R";
        board[7][7] = "r";
        assert_eq!(board[0][0], "R");
        assert_eq!(board[7][7], "r");
    }
}


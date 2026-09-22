// file that defines the board struct, and its methods

use crate::constants as C;
use crate::log_move as L;
use crate::pieces as P;

struct Board { 
    board : [[Option<P::Piece>; C::XWIDTH]; C::YWIDTH], 
    log : L::Log
}

fn fresh_constructor() -> [[Option<P::Piece>; C::XWIDTH]; C::YWIDTH] {
    let mut m: [[Option<P::Piece>; C::XWIDTH]; C::YWIDTH] =
        std::array::from_fn(|_| std::array::from_fn(|_| None));
    for y in [1, C::YWIDTH - 1] {
        let is_white = y == 1; // `constants.rs:4`: `false=white`
        for x in 0..C::XWIDTH {
            m[y][x] = Some(P::Piece::inst("P", x as u8, y as u8, is_white));
        }
    }
    for x in 0..C::XWIDTH/2 {
        let ptm = match x%4 {
            0 => "R",
            1 => "N",
            2 => "B", 
            3 => "Q",
            _ => "you win a fields medal"
        }; 
        for y in [0, C::YWIDTH]{
            let is_white = y == 0;
            for x1 in [0 + x, C::XWIDTH - x] {
                m[y][x1] = Some(P::Piece::inst(ptm, x as u8, y as u8, is_white));
            } 
        }
        
    }

    // TODO: replace 1 queen on each side with one king
    
    m
}

impl Board {
    fn new_board () -> Board{
        Board {
            board : fresh_constructor(), 
            log : L::Log::new_log()
        }
    }
}
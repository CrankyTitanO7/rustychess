// file that defines the board struct, and its methods

use crate::constants as C;
use crate::log_move as L;
use crate::pieces as P;

pub struct Board { 
    pub board : [[Option<P::Piece>; C::XWIDTH]; C::YWIDTH], 
    pub log : L::Log
}

fn fresh_constructor() -> [[Option<P::Piece>; C::XWIDTH]; C::YWIDTH] {
    let mut m: [[Option<P::Piece>; C::XWIDTH]; C::YWIDTH] =
        std::array::from_fn(|_| std::array::from_fn(|_| None));
    // Pawns: white (false) on rank y=1, black (true) on rank y=YWIDTH-2.
    // Note `constants.rs:4`: `false=white`, `true=black`.
    for y in [1, C::YWIDTH - 2] {
        let is_black = y == C::YWIDTH - 2;
        for x in 0..C::XWIDTH {
            m[y][x] = Some(P::Piece::inst("P", x as u8, y as u8, is_black));
        }
    }
    // Back rank: R N B Q K B N R on y=0 (white) and y=YWIDTH-1 (black).
    let back_rank = ["R", "N", "B", "Q", "K", "B", "N", "R"];
    for y in [0, C::YWIDTH - 1] {
        let is_black = y != 0;
        for x in 0..C::XWIDTH {
            m[y][x] = Some(P::Piece::inst(back_rank[x], x as u8, y as u8, is_black));
        }
    }

    m
}

impl Board {
    pub fn new_board () -> Board{
        Board {
            board : fresh_constructor(), 
            log : L::Log::new_log()
        }
    }
}
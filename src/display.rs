// mod constants;
use crate::constants as C;
use crate::board as B;

// Render the board grid as plain text: one rank per line, rank 8 first.
// Empty squares render as `.`, pieces as their unicode `symbol`.
// Shared by stdout (`display_board`) and the ratatui PvP loop.
pub fn board_text (b : &B::Board) -> String {
    let mut out = String::new();
    for y in (0..C::YWIDTH).rev() {
        for x in 0.. C::XWIDTH {
            match &b.board[y][x] {
                Some (p) => {
                    out.push_str(&p.symbol);
                    out.push(' ');
                },
                None => out.push_str(". ")
            };
        }
        // Trim trailing space; newline between ranks (none after last).
        out.truncate(out.len().saturating_sub(1));
        if y != 0 {
            out.push('\n');
        }
    }
    out
}

// displays an 8x8 array of strings
pub fn display_board (b : &B::Board) {
    // board.board is indexed [y][x] (row-major); print rank 8 first.
    println!("{}", board_text(b));
}



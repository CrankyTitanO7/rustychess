// Integration-style tests for rustychess (learning placeholders).
// Note: src/ is currently a binary crate (src/main.rs, no src/lib.rs),
// so `use rustychess::...` won't resolve yet. These tests are intentionally
// self-contained so you can see the `tests/` layout without depending on the crate.
// Once you add src/lib.rs, you can change these to real integration tests.

const XWIDTH: usize = 8;
const YWIDTH: usize = 8;

fn empty_board() -> [[&'static str; XWIDTH]; YWIDTH] {
    [["."; XWIDTH]; YWIDTH]
}

#[test]
fn board_is_8x8() {
    let board = empty_board();
    assert_eq!(board.len(), 8);
    assert_eq!(board[0].len(), 8);
}

#[test]
fn board_area_is_64() {
    assert_eq!(XWIDTH * YWIDTH, 64);
}

#[test]
fn board_corners_are_indexable() {
    let mut board = empty_board();
    board[0][0] = "R";
    board[0][7] = "R";
    board[7][0] = "r";
    board[7][7] = "r";
    assert_eq!(board[0][0], "R");
    assert_eq!(board[7][7], "r");
}

#[test]
fn empty_board_has_no_pieces() {
    let board = empty_board();
    for row in board.iter() {
        for square in row.iter() {
            assert_eq!(*square, ".");
        }
    }
}

#[test]
fn white_is_false_convention() {
    // mirrors comment in src/constants.rs: bool: 0 = white
    let white: bool = false;
    let black: bool = true;
    assert_ne!(white, black);
    assert!(!white);
}

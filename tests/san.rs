// Integration tests for `san` stubs.

use rustychess::board::Board;
use rustychess::mv::{Coord, Move};
use rustychess::san;

fn fresh() -> Board {
    Board::new_board()
}

#[test]
fn san_stubs_callable_without_panic() {
    let b = fresh();
    let _ = san::sq_name(4, 1); // stub "" for now
    let _ = san::parse_square("e2"); // stub None for now
    assert!(san::parse_coordinate("e2e4").is_err()); // stub always errs
    assert_eq!(san::normalize_castle_input("e4"), None);
    let (core, _, _) = san::strip_check_mate_suffix("Nf3");
    assert_eq!(core, "Nf3");
    assert_eq!(san::promotion_piece_from_char('x'), None);
    let m = Move::quiet(Coord::new(4, 1), Coord::new(4, 3));
    let _ = san::needs_disambiguation(&b, &m);
    assert!(san::parse_san(&b, "e4").is_err()); // stub always errs
    let _ = san::to_san(&b, &m);
    let _ = san::move_to_coordinate(&m);
}

#[test]
#[ignore = "TODO: implement san coordinate + SAN parsing"]
fn coordinate_and_san_parse_startpos() {
    let b = fresh();
    assert!(san::parse_coordinate("e2e4").is_ok());
    assert!(san::parse_san(&b, "e4").is_ok());
    assert!(san::parse_san(&b, "Nf3").is_ok());
}

#[test]
#[ignore = "TODO: implement san::to_san"]
fn to_san_e2e4_is_e4() {
    let b = fresh();
    let m = san::parse_coordinate("e2e4").unwrap();
    assert_eq!(san::to_san(&b, &m), "e4");
}

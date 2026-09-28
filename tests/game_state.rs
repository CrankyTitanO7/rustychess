// Integration tests for `game_state` via the library crate.

use rustychess::game_state::{CastlingRights, GameState};

#[test]
fn rights_constructors() {
    assert_eq!(CastlingRights::none(), CastlingRights::none());
    assert_eq!(CastlingRights::initial(), CastlingRights::initial());
    assert_ne!(CastlingRights::none(), CastlingRights::initial());
}

#[test]
fn state_initial_is_white_to_move() {
    let s = GameState::initial();
    assert!(!s.turn_is_black());
    assert_eq!(s.turn_name(), "White");
    assert_eq!(s.en_passant, None);
    assert_eq!(s.fullmove_number, 1);
}

#[test]
#[ignore = "TODO: implement CastlingRights mutators"]
fn rights_mutators() {
    let mut r = CastlingRights::initial();
    r.clear_kingside(false);
    assert!(!r.white_kingside);
    r.clear_queenside(true);
    assert!(!r.black_queenside);
    r.clear_all_for(false);
    assert!(!r.white_queenside);
    assert!(r.black_kingside); // other color untouched
}

#[test]
#[ignore = "TODO: implement GameState clocks/turn"]
fn turn_and_clocks() {
    let mut s = GameState::initial();
    s.advance_turn();
    assert!(s.turn_is_black());
    s.set_en_passant((4, 2));
    assert_eq!(s.en_passant, Some((4, 2)));
    s.clear_en_passant();
    assert_eq!(s.en_passant, None);
    s.bump_halfmove();
    assert_eq!(s.halfmove_clock, 1);
    s.reset_halfmove();
    assert_eq!(s.halfmove_clock, 0);
}

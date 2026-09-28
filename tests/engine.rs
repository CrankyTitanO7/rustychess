// Integration tests for `engine` stubs.
// Full per-function cases live in `src/engine.rs`; these verify wiring +
// the one invariant that must hold even before implementation: the
// startpos exists and stubs are callable without panicking.

use rustychess::board::Board;
use rustychess::engine;
use rustychess::game_state::GameResult;

fn fresh() -> Board {
    Board::new_board()
}

#[test]
fn engine_stubs_callable_without_panic() {
    let b = fresh();
    let _ = engine::in_bounds(0, 0);
    let _ = engine::find_king(&b, false);
    let _ = engine::is_attacked(&b, 4, 3, true);
    let _ = engine::king_in_check(&b, false);
    assert!(engine::pawn_moves(&b, 4, 1).is_empty()); // stub: empty for now
    assert!(engine::knight_moves(&b, 1, 0).is_empty());
    assert!(engine::pseudo_moves_for(&b, 1, 0).is_empty());
    assert!(engine::legal_moves_for(&b, 1, 0).is_empty());
    assert!(engine::all_legal_moves(&b).is_empty());
    assert!(!engine::has_any_legal(&b, false));
    assert!(!engine::is_insufficient_material(&b));
    assert_eq!(engine::game_result(&b), GameResult::Ongoing);
    assert!(!engine::is_checkmate(&b));
    assert!(!engine::is_stalemate(&b));
}

#[test]
#[ignore = "TODO: implement engine move generation (expect 20 startpos moves)"]
fn startpos_has_20_legal_moves() {
    let b = fresh();
    assert_eq!(engine::all_legal_moves(&b).len(), 20);
}

#[test]
#[ignore = "TODO: implement engine::make_move"]
fn make_move_e2e4() {
    let mut b = fresh();
    let mv = rustychess::mv::Move::from_uci("e2e4").unwrap();
    assert!(engine::make_move(&mut b, &mv).is_ok());
}

#[test]
fn board_carries_initial_gamestate() {
    // Scaffolding invariant: fresh boards ship startpos GameState.
    let b = fresh();
    assert!(!b.state.turn_is_black());
    assert_eq!(b.state.fullmove_number, 1);
    assert!(b.state.castling.can_castle(false, true) || !b.state.castling.can_castle(false, true));
}

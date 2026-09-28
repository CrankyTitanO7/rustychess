// Integration tests for `ai` stubs.

use rustychess::ai;
use rustychess::board::Board;

fn fresh() -> Board {
    Board::new_board()
}

#[test]
fn ai_stubs_callable_without_panic() {
    let b = fresh();
    let _ = ai::piece_value("P"); // stub 0 for now
    let _ = ai::evaluate_material(&b); // stub 0 for now
    assert!(ai::random_move(&b).is_none()); // stub None for now
    assert!(ai::greedy_move(&b).is_none());
    assert!(ai::minimax_move(&b, 1).is_none());
    assert!(ai::choose_move(&b).is_none());
}

#[test]
#[ignore = "TODO: implement ai evaluation + search"]
fn startpos_even_and_has_move() {
    let b = fresh();
    assert_eq!(ai::evaluate_material(&b), 0);
    assert!(ai::random_move(&b).is_some());
    assert!(ai::greedy_move(&b).is_some());
    assert!(ai::choose_move(&b).is_some());
}

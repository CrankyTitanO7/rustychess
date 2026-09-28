// Computer opponent stubs (PvC / CvC backing for `menu.rs` placeholders). Learning path: see `src/TODO.md` Phase 11.
// Teaches composition, max_by, Option chaining — do last.
//
// Difficulty ladder hint:
//   1. `random_move` — uniform random legal move (baseline, always works).
//   2. `greedy_move` — 1-ply: maximize `evaluate_material` after the move.
//   3. `minimax_move(depth)` — alpha-beta over `engine::all_legal_moves`
//      + `engine::make_move`/`unmake` with `evaluate_material` leaves.
// `choose_move` is the dispatcher the game loops will call.
//
// All functions are compilable stubs; TODOs describe inputs/outputs.
// Requires `rand`? Not yet in `Cargo.toml` — v1 `random_move` can use a
// deterministic first-move or `std::time` nanos; add `rand` dep later.

use crate::board::Board;
use crate::mv::Move;

fn _touch_board(b: &Board) {
    let _ = b;
}

/// Inputs: piece short name (`"P"/"N"/"B"/"R"/"Q"/"K"`).
/// Output: centipawn-ish value, e.g. P=100 N=320 B=330 R=500 Q=900 K=0.
/// TODO: match on `short`; unknown -> 0.
/// Hint: king is 0 (infinite in search, but evaluation treats it as 0 since
///   checkmate is handled by `engine::game_result`, not material).
pub fn piece_value(short: &str) -> i32 {
    // TODO: implement match; stub returns garbage so it compiles.
    let _ = short;
    0
}

/// Inputs: board. Output: material balance from White's perspective
///   (`>0` favors White, `<0` favors Black, `0` even).
/// TODO: sum `+value` for white pieces, `-value` for black pieces by
///   scanning `board.board`. Ignore position/activity in v1.
/// Hint: iterate 8x8, match `name_short` + `color`.
pub fn evaluate_material(board: &Board) -> i32 {
    // TODO: implement scan; stub returns garbage so it compiles.
    _touch_board(board);
    0
}

/// Inputs: board (side to move from `board.state` / `turn_is_black`).
/// Output: one random legal move, or `None` if no legal moves (mate/stale).
/// TODO: `engine::all_legal_moves`, pick via RNG; v1 stub may return
///   `.first().copied()` deterministically until `rand` is added.
/// Hint: keep pure (no mutation); game loop applies via `engine::make_move`.
pub fn random_move(board: &Board) -> Option<Move> {
    // TODO: implement; stub returns None so it compiles.
    _touch_board(board);
    None
}

/// Inputs: board. Output: the 1-ply greedy capture-maximizing move, or
///   `None` if no legal moves.
/// TODO: for each legal move, apply on a scratch copy (or save/unmake),
///   score with `evaluate_material` from the mover's perspective, keep max;
///   tie-break with first-seen (or random among equals once RNG lands).
/// Hint: White maximizes, Black minimizes `evaluate_material`.
pub fn greedy_move(board: &Board) -> Option<Move> {
    // TODO: implement; stub returns None so it compiles.
    _touch_board(board);
    None
}

/// Inputs: board + search depth (plies). Output: best move found, or `None`
///   if no legal moves or depth 0.
/// TODO: alpha-beta minimax: `depth==0` or terminal (`engine::game_result`
///   != Ongoing) returns static eval; else recurse over legal moves with
///   make/unmake. Return `None` on depth 0 (caller falls back to greedy).
/// Hint: needs an `unmake`/clone story first — see
///   `engine::would_leave_king_in_check` TODO about `Piece: Clone`.
pub fn minimax_move(board: &Board, depth: u32) -> Option<Move> {
    // TODO: implement; stub returns None so it compiles.
    _touch_board(board);
    let _ = depth;
    None
}

/// Inputs: board. Output: the engine's chosen move (`None` = game over).
/// TODO: v1 dispatcher: `minimax_move(depth=2)` if implemented, else
///   `greedy_move`, else `random_move`. Document the chosen default here.
/// Hint: this is what `gameloops::pvc/cvc` will call per computer turn.
pub fn choose_move(board: &Board) -> Option<Move> {
    // TODO: implement dispatcher; stub returns None so it compiles.
    _touch_board(board);
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    fn fresh() -> Board {
        Board::new_board()
    }

    #[test]
    #[ignore = "TODO: implement ai::piece_value"]
    fn piece_values_sensible() {
        assert_eq!(piece_value("P"), 100);
        assert!(piece_value("N") >= 300 && piece_value("N") <= 350);
        assert!(piece_value("B") >= 300 && piece_value("B") <= 350);
        assert_eq!(piece_value("R"), 500);
        assert_eq!(piece_value("Q"), 900);
        assert_eq!(piece_value("K"), 0);
        assert_eq!(piece_value("?"), 0);
    }

    #[test]
    #[ignore = "TODO: implement ai::evaluate_material"]
    fn material_startpos_even() {
        let b = fresh();
        assert_eq!(evaluate_material(&b), 0);
        // TODO: remove a black pawn, assert >0; remove white queen, assert <0.
    }

    #[test]
    #[ignore = "TODO: implement ai::random_move"]
    fn random_move_startpos_some() {
        let b = fresh();
        assert!(random_move(&b).is_some());
        // TODO: empty/stalemate board -> None.
    }

    #[test]
    #[ignore = "TODO: implement ai::greedy_move"]
    fn greedy_takes_free_piece() {
        let _b = fresh();
        // TODO: build a position with a hanging piece, assert greedy captures.
        // Startpos smoke: some move exists.
        // let b = fresh(); assert!(greedy_move(&b).is_some());
    }

    #[test]
    #[ignore = "TODO: implement ai::minimax_move"]
    fn minimax_depth0_none_and_depth1_some() {
        let b = fresh();
        assert_eq!(minimax_move(&b, 0), None);
        // TODO: once implemented: assert!(minimax_move(&b, 1).is_some());
    }

    #[test]
    fn choose_move_stub_callable() {
        let b = fresh();
        let _ = choose_move(&b); // stub returns None for now; just must compile
    }
}

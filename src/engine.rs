// Legal chess rules engine (move generation + validation + game end). Learning path: see `src/TODO.md` Phases 2-7, 9-10.
// Teaches borrowing, Vec/iterators, &mut, Result — do after `mv`/`game_state`.
//
// Operates on `Board` (piece grid in `board.rs`) plus `Move`/`GameState`.
// All functions are stubs returning harmless garbage so the crate compiles;
// each TODO describes inputs, outputs, and the algorithm to fill in.
//
// Implementation order hint:
//   1. `in_bounds` + `find_king`
//   2. `is_attacked` (needed by everything)
//   3. per-piece `*_moves` + `pseudo_moves_for`
//   4. `would_leave_king_in_check` + `legal_moves_for` / `all_legal_moves`
//   5. `make_move` (updates grid + `GameState`)
//   6. `has_any_legal`, `is_insufficient_material`, `game_result`
//
// Piece identity hint: `board.board[y][x]` is `Option<Piece>` where
// `Piece.name_short` in `{"P","N","B","R","Q","K"}` and
// `Piece.color: bool` (`false`=White, `true`=Black).

use crate::board::Board;
use crate::game_state::{DrawReason, GameResult};
use crate::mv::Move;

fn _touch_board(b: &Board) {
    let _ = b;
}

/// Inputs: candidate `x`,`y` as `i32` (may be off-board after offsets).
/// Output: `true` iff `0 <= x < 8 && 0 <= y < 8`.
/// TODO: compare against `constants::XWIDTH/YWIDTH` as `i32`.
/// Hint: all sliding/jump loops must call this before indexing.
pub fn in_bounds(x: i32, y: i32) -> bool {
    if 0 <= x&& x < 8 && 0 <= y && y < 8 {
        true
    } else {false}
    // TODO: implement; stub returns garbage so it compiles.
    // let _ = (x, y);
    // false
}

/// Inputs: board + side (`black: false`=White king, `true`=Black king).
/// Output: `(x,y)` of that side's king, or `None` if missing (tests may
///   use partial boards).
/// TODO: scan 8x8 for `name_short == "K"` with matching `color`.
pub fn find_king(board: &Board, black: bool) -> Option<(usize, usize)> {
    // TODO: implement scan; stub returns None so it compiles.
    // use crate::pieces::Piece;
    use crate::constants::XWIDTH as xmax; use crate::constants::YWIDTH as ymax;
    for x in 0..xmax {
        for y in 0..ymax {
            // let ktemp = Some (Piece {
            //     color: !black, 
            //     name_long: "king".to_string(), 
            //     name_short: "K".to_string(), 
            //     symbol: if !black {"♚".to_string()} else {"♔".to_string()}, 
            //     locx : x as u8, 
            //     locy : y as u8
            // }); 
            if let Some(cur) = &board.board[x.clone()][y.clone()] {
                if cur.name_short == "K" {
                    return Some((x, y));
                }
            }
            
        }
    }

    None
}

/// Inputs: board, target square, attacker color.
/// Output: `true` if any piece of `by_black` attacks `(x,y)`.
/// TODO: check pawn attacks (direction depends on attacker color: white
///   pawns attack `y+1`, black attack `y-1`), knight jumps, king ring,
///   sliding rays for B/R/Q (stop at first blocker; attacker only if the
///   first piece on the ray is a matching slider/queen).
/// Hint: white pawns on `(x-1,y-1)/(x+1,y-1)` attack `(x,y)`; black pawns
///   attack from `(x-1,y+1)/(x+1,y+1)`. Use `in_bounds` for every step.
pub fn is_attacked(board: &Board, x: usize, y: usize, by_black: bool) -> bool {
    // TODO: implement; stub returns garbage so it compiles.

    // define tuple iterator;
    // let mut xy = (0, 0); 
    // let b = &board.board;

    // calculates every possible reversible move (aka not pawns)
    let atk = [
        king_moves(board, x, y)     , 
        queen_moves(board, x, y)    ,
        bishop_moves(board, x, y)   , 
        knight_moves(board, x, y)   , 
        rook_moves(board, x, y)     , 
    ];

    // wasteful data. you should delete this if you end up not using it
    let atkmvs: Vec<(Move, String)> = atk.into_iter().flatten()
  .filter(|m| m.to.x as usize == x && m.to.y as usize == y)
  .map(|m| {
    let short = board.board[m.from.y as usize][m.from.x as usize]
      .as_ref()
      .map(|p| p.name_short.clone())
      .unwrap_or_default(); // `Piece::name_short: String` in `src/pieces.rs:5`
    (m, short)
  })
  .collect();

    // let atkbool = atk.clone().map(|moves| moves.iter().any(|m| m.to.x as usize == x && m.to.y as usize ==y)); 

    for mv in atkmvs {
        if board.board[mv.0.to.x][mv.0.to.y].name_short == mv.1 {
            return true;
        }
    }

    // non-reversible (pawn) case:
    if (by_black && (board.board[x-1][y-1].unwrap().name_short == "P" 
                    || board.board[x+1][y-1].unwrap().name_short == "P")) {
        true; 
    }
    if (!by_black && (board.board[x-1][y+1].unwrap().name_short == "P" 
                        || board.board[x+1][y+1].unwrap().name_short == "P")) {
        true;
    }
    
    false
}

/// Inputs: board + side. Output: `true` if that side's king is in check.
/// TODO: `find_king` then `is_attacked(king, !black)`. `None` king -> `false`.
/// Hint: used by move filtering and `game_result`.
pub fn king_in_check(board: &Board, black: bool) -> bool {
    // TODO: implement; stub returns garbage so it compiles.
    _touch_board(board);
    let _ = black;
    false
}

/// Inputs: board + pawn square. Output: pseudo-legal pawn pushes/captures
///   (ignores self-check). Includes double push from start rank, captures,
///   en-passant (if `board.state.en_passant` matches a diagonal), and
///   promotion expansion (one `Move` per `N/B/R/Q` when reaching last rank).
/// TODO: direction `+1` for white, `-1` for black; start rank y=1 white,
///   y=6 black; last rank y=7 white, y=0 black. Captures require enemy
///   piece OR e.p. target (with `is_en_passant=true`, dest empty).
/// Hint: needs `board.state.en_passant`; see `game_state.rs` docs.
pub fn pawn_moves(board: &Board, x: usize, y: usize) -> Vec<Move> {
    // TODO: implement; stub returns empty so it compiles.
    _touch_board(board);
    let _ = (x, y);
    Vec::new()
}

/// Inputs: board + knight square. Output: up-to-8 L-jumps landing on empty
///   or enemy squares (ignores self-check).
/// TODO: offsets `[(±1,±2),(±2,±1)]` + `in_bounds` + own-piece rejection.
pub fn knight_moves(board: &Board, x: usize, y: usize) -> Vec<Move> {
    // TODO: implement; stub returns empty so it compiles.
    _touch_board(board);
    let _ = (x, y);
    Vec::new()
}

/// Inputs: board + bishop square. Output: diagonal slides (ignores self-check).
/// TODO: 4 rays `[(1,1),(1,-1),(-1,1),(-1,-1)]`; walk until blocker;
///   empty -> push quiet; enemy -> push capture then stop; own -> stop.
pub fn bishop_moves(board: &Board, x: usize, y: usize) -> Vec<Move> {
    // TODO: implement; stub returns empty so it compiles.
    _touch_board(board);
    let _ = (x, y);
    Vec::new()
}

/// Inputs: board + rook square. Output: orthogonal slides (ignores self-check).
/// TODO: 4 rays `[(1,0),(-1,0),(0,1),(0,-1)]`, same blocking rules as bishop.
pub fn rook_moves(board: &Board, x: usize, y: usize) -> Vec<Move> {
    // TODO: implement; stub returns empty so it compiles.
    _touch_board(board);
    let _ = (x, y);
    Vec::new()
}

/// Inputs: board + queen square. Output: bishop + rook rays combined.
/// TODO: delegate to `bishop_moves` + `rook_moves` and concat.
pub fn queen_moves(board: &Board, x: usize, y: usize) -> Vec<Move> {
    // TODO: implement; stub returns empty so it compiles.
    _touch_board(board);
    let _ = (x, y);
    Vec::new()
}

/// Inputs: board + king square. Output: 1-step ring + castling (ignores
///   self-check for the ring; castling pre-checks rights/emptiness/safety).
/// TODO: 8 neighbours with own-piece rejection; castling only if
///   `board.state.castling.can_castle(...)`, squares between K and R empty,
///   king not in check and transit squares not attacked. Emit with
///   `is_castle=true`, from e1/e8 to g1/c1 (white) or g8/c8 (black).
/// Hint: corners a1=(0,0) h1=(7,0) a8=(0,7) h8=(7,7); e1=(4,0) e8=(4,7).
pub fn king_moves(board: &Board, x: usize, y: usize) -> Vec<Move> {
    // TODO: implement; stub returns empty so it compiles.
    _touch_board(board);
    let _ = (x, y);
    Vec::new()
}

/// Inputs: board + source square. Output: pseudo-legal moves for whatever
///   piece sits there (`[]` if empty or unknown type). Ignores self-check.
/// TODO: dispatch on `name_short` (`P/N/B/R/Q/K`) to the helper above.
/// Hint: central switch the UI and SAN layers call first.
pub fn pseudo_moves_for(board: &Board, x: usize, y: usize) -> Vec<Move> {
    // TODO: implement dispatch; stub returns empty so it compiles.
    _touch_board(board);
    let _ = (x, y);
    Vec::new()
}

/// Inputs: board + move. Output: `true` if playing it would expose the
///   mover's own king to check (i.e. the move is illegal).
/// TODO: clone-ish make/unmake: apply `mv` to a scratch board (or save +
///   restore the 2-4 touched squares + `GameState`), then `king_in_check`.
///   Must handle e.p. captured-pawn square and castling rook squares.
/// Hint: simplest correct version copies the 8x8 `Option<Piece>` grid via a
///   helper; optimize later. `Piece` is not `Clone`, so move pieces with
///   `Option::take` + restore, or add `Clone` to `Piece` first.
pub fn would_leave_king_in_check(board: &Board, mv: &Move) -> bool {
    // TODO: implement; stub returns garbage so it compiles.
    _touch_board(board);
    let _ = mv;
    false
}

/// Inputs: board + source square. Output: fully legal moves (pseudo minus
///   self-check exposures).
/// TODO: `pseudo_moves_for` + `filter(!would_leave_king_in_check)`.
pub fn legal_moves_for(board: &Board, x: usize, y: usize) -> Vec<Move> {
    // TODO: implement; stub returns empty so it compiles.
    _touch_board(board);
    let _ = (x, y);
    Vec::new()
}

/// Inputs: board. Output: every legal move for the side to move.
/// TODO: iterate own pieces (`color == board.state.turn_black` — or
///   `board.turn_is_black()` until `GameState` wiring lands), collect
///   `legal_moves_for`.
/// Hint: used by SAN disambiguation, checkmate/stalemate, and AI.
pub fn all_legal_moves(board: &Board) -> Vec<Move> {
    // TODO: implement; stub returns empty so it compiles.
    _touch_board(board);
    Vec::new()
}

/// Inputs: `&mut board` + legal `Move`.
/// Output: `Ok(log_string)` on success, `Err(reason)` if illegal.
/// Effect: relocates the piece (updating `locx/locy`), removes captured
///   material (including e.p. pawn), moves the rook on castling, promotes
///   via `Piece::pawn_to_x`, updates `GameState` (turn, castling rights via
///   `CastlingRights::update_on_move`, e.p. set/clear, half/fullmove clocks),
///   and appends to `Log` via `add_move_to_log`.
/// TODO: validate membership in `legal_moves_for(from)` first so the board
///   is untouched on error; then mutate.
/// Hint: mirror `Board::try_move` relocation code, then add state updates.
pub fn make_move(board: &mut Board, mv: &Move) -> Result<String, String> {
    // TODO: implement; stub returns Err so it compiles without mutating.
    let _ = (board, mv);
    Err("TODO: engine::make_move not implemented".to_string())
}

/// Inputs: board + side. Output: `true` if that side has any legal move.
/// TODO: early-exit scan (faster than `all_legal_moves` when only existence
///   matters). Used by `game_result`.
pub fn has_any_legal(board: &Board, black: bool) -> bool {
    // TODO: implement; stub returns garbage so it compiles.
    _touch_board(board);
    let _ = black;
    false
}

/// Inputs: board. Output: `true` for K-vs-K, K+minor-vs-K, K+minor-vs-K+minor
///   same-color-bishops, etc. (FIDE dead positions).
/// TODO: collect non-king material; simplest v1: `true` iff no pawns/rooks/
///   queens and total minor count <= 1 (document stricter cases as follow-up).
pub fn is_insufficient_material(board: &Board) -> bool {
    // TODO: implement; stub returns garbage so it compiles.
    _touch_board(board);
    false
}

/// Inputs: board. Output: game termination status.
/// TODO: if `all_legal_moves` empty -> `in_check ? Checkmate : Stalemate`;
///   else if `is_insufficient_material` -> `Draw(InsufficientMaterial)`;
///   else if `halfmove_clock >= 100` -> `Draw(FiftyMove)`; threefold needs
///   position-history hashing (leave `TODO` + return `Ongoing` for now).
/// Hint: uses `king_in_check`, `has_any_legal`, `board.state.halfmove_clock`.
pub fn game_result(board: &Board) -> GameResult {
    // TODO: implement; stub returns Ongoing so it compiles.
    _touch_board(board);
    GameResult::Ongoing
}

/// Inputs: board. Output: `true` iff side to move is checkmated.
/// TODO: `king_in_check(side) && !has_any_legal(side)` (one-liner over helpers).
pub fn is_checkmate(board: &Board) -> bool {
    // TODO: implement; stub returns garbage so it compiles.
    _touch_board(board);
    let _ = DrawReason::FiftyMove; // keep import used in scaffold
    false
}

/// Inputs: board. Output: `true` iff side to move is stalemated.
/// TODO: `!king_in_check(side) && !has_any_legal(side)`.
pub fn is_stalemate(board: &Board) -> bool {
    // TODO: implement; stub returns garbage so it compiles.
    _touch_board(board);
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    fn fresh() -> Board {
        Board::new_board()
    }

    #[test]
    #[ignore = "TODO: implement engine::in_bounds"]
    fn in_bounds_corners_and_outside() {
        assert!(in_bounds(0, 0));
        assert!(in_bounds(7, 7));
        assert!(!in_bounds(-1, 0));
        assert!(!in_bounds(0, 8));
        assert!(!in_bounds(8, 8));
    }

    #[test]
    #[ignore = "TODO: implement engine::find_king"]
    fn find_king_startpos() {
        let b = fresh();
        assert_eq!(find_king(&b, false), Some((4, 0)));
        assert_eq!(find_king(&b, true), Some((4, 7)));
    }

    #[test]
    #[ignore = "TODO: implement engine::is_attacked"]
    fn is_attacked_startpos() {
        let b = fresh();
        // White knight on b1 attacks a3 and c3.
        assert!(is_attacked(&b, 0, 2, false));
        assert!(is_attacked(&b, 2, 2, false));
        // Empty center is not attacked by black at start.
        assert!(!is_attacked(&b, 4, 3, true));
    }

    #[test]
    #[ignore = "TODO: implement engine::king_in_check"]
    fn king_in_check_startpos_false() {
        let b = fresh();
        assert!(!king_in_check(&b, false));
        assert!(!king_in_check(&b, true));
    }

    #[test]
    #[ignore = "TODO: implement engine::pawn_moves"]
    fn pawn_moves_startpos() {
        let b = fresh();
        // White e-pawn has e3 + e4.
        let mut ucis: Vec<String> = pawn_moves(&b, 4, 1).iter().map(|m| m.to_uci()).collect();
        ucis.sort();
        assert_eq!(ucis, vec!["e2e3".to_string(), "e2e4".to_string()]);
        // Blocked / empty squares yield nothing.
        assert!(pawn_moves(&b, 4, 3).is_empty());
    }

    #[test]
    #[ignore = "TODO: implement engine::knight_moves"]
    fn knight_moves_startpos() {
        let b = fresh();
        let mut ucis: Vec<String> = knight_moves(&b, 1, 0).iter().map(|m| m.to_uci()).collect();
        ucis.sort();
        assert_eq!(ucis, vec!["b1a3".to_string(), "b1c3".to_string()]);
    }

    #[test]
    #[ignore = "TODO: implement engine::bishop_moves"]
    fn bishop_moves_blocked_startpos() {
        let b = fresh();
        assert!(bishop_moves(&b, 2, 0).is_empty()); // c1 shut in by pawns
    }

    #[test]
    #[ignore = "TODO: implement engine::rook_moves"]
    fn rook_moves_blocked_startpos() {
        let b = fresh();
        assert!(rook_moves(&b, 0, 0).is_empty()); // a1 shut in
    }

    #[test]
    #[ignore = "TODO: implement engine::queen_moves"]
    fn queen_moves_blocked_startpos() {
        let b = fresh();
        assert!(queen_moves(&b, 3, 0).is_empty()); // d1 shut in
    }

    #[test]
    #[ignore = "TODO: implement engine::king_moves"]
    fn king_moves_blocked_startpos() {
        let b = fresh();
        assert!(king_moves(&b, 4, 0).is_empty()); // e1 shut in, no castling yet
    }

    #[test]
    #[ignore = "TODO: implement engine::pseudo_moves_for"]
    fn pseudo_dispatches_by_piece() {
        let b = fresh();
        assert_eq!(pseudo_moves_for(&b, 1, 0).len(), 2); // b1 knight
        assert!(pseudo_moves_for(&b, 4, 3).is_empty()); // empty e4
    }

    #[test]
    #[ignore = "TODO: implement engine::legal_moves_for"]
    fn legal_filters_pinned_piece() {
        // Classic pin: white e2 pawn pinned by black rook on e8 after
        // 1.e4 e5 2.Qh5? — exact setup is a follow-up; here just check the
        // startpos knight has 2 legal moves like its pseudo moves.
        let b = fresh();
        assert_eq!(legal_moves_for(&b, 1, 0).len(), 2);
    }

    #[test]
    #[ignore = "TODO: implement engine::all_legal_moves"]
    fn all_legal_moves_startpos_is_20() {
        let b = fresh();
        assert_eq!(all_legal_moves(&b).len(), 20); // 16 pawn + 4 knight
    }

    #[test]
    #[ignore = "TODO: implement engine::would_leave_king_in_check"]
    fn pinned_move_exposes_king() {
        let _b = fresh();
        // TODO: build a pinned position, assert true for the exposing move
        // and false for a safe king-adjacent move.
    }

    #[test]
    #[ignore = "TODO: implement engine::make_move"]
    fn make_move_applies_and_logs() {
        let mut b = fresh();
        let mv = crate::mv::Move::from_uci("e2e4").unwrap();
        assert!(make_move(&mut b, &mv).is_ok());
        assert!(b.board[3][4].is_some()); // e4 now occupied
        assert!(b.board[1][4].is_none()); // e2 now empty
    }

    #[test]
    #[ignore = "TODO: implement engine::has_any_legal"]
    fn has_any_legal_startpos_true() {
        let b = fresh();
        assert!(has_any_legal(&b, false));
    }

    #[test]
    #[ignore = "TODO: implement engine::is_insufficient_material"]
    fn insufficient_k_vs_k() {
        let _b = fresh();
        // TODO: clear to K vs K, assert true; startpos assert false.
    }

    #[test]
    #[ignore = "TODO: implement engine::game_result"]
    fn game_result_startpos_ongoing() {
        let b = fresh();
        assert_eq!(game_result(&b), GameResult::Ongoing);
    }

    #[test]
    #[ignore = "TODO: implement checkmate/stalemate helpers"]
    fn mate_and_stalemate_helpers() {
        let b = fresh();
        assert!(!is_checkmate(&b));
        assert!(!is_stalemate(&b));
        // TODO: add Fool's-mate and classic stalemate fixtures.
    }
}

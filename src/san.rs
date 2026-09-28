// Algebraic notation: coordinate I/O plus Standard Algebraic Notation (SAN). Learning path: see `src/TODO.md` Phase 8, 10.
// Teaches lifetimes (&str->&str), Result, string parsing — do after engine basics.
//
// Coordinate (already supported in `board.rs`): `e2e4`, `e2 e4`, `e2-e4`,
//   `e2xe4`, promotion `e7e8=Q` / `e7e8Q`.
// SAN (new): `e4`, `Nf3`, `exd5`, `Nbd7`, `R1e2`, `O-O`, `O-O-O`, `e8=Q`,
//   with `+` / `#` check/mate suffixes on output (`to_san`) and optional
//   on input (`parse_san`).
//
// All functions are compilable stubs; TODOs describe parsing/generation.
//
// Hint: SAN resolution always goes through `engine::all_legal_moves` —
//   generate the legal set, then match by destination/piece/disambiguation
//   instead of hand-deriving movement rules in this module.

use crate::board::Board;
use crate::mv::Move;

/// Inputs: 0-based `x`,`y`. Output: square name like `"e2"`.
/// TODO: `format!("{}{}", (b'a'+x) as char, y+1)`.
/// Hint: duplicate of private `board::sq_name`; centralize here later.
pub fn sq_name(x: usize, y: usize) -> String {
    // TODO: implement; stub returns empty so it compiles.
    let _ = (x, y);
    String::new()
}

/// Inputs: text like `"e2"`. Output: `Some((x,y))` 0-based or `None`.
/// TODO: 2-char file a-h + rank 1-8 check, map to 0-based.
/// Hint: duplicate of private `board::parse_square`; centralize here later.
pub fn parse_square(s: &str) -> Option<(usize, usize)> {
    // TODO: implement; stub returns None so it compiles.
    let _ = s;
    None
}

/// Inputs: coordinate text (`e2e4` family, see module docs).
/// Output: `Ok(Move)` with castle/e.p. flags `false` (resolved by engine
///   at apply time), or `Err(reason)` with user-facing message.
/// TODO: normalize (lowercase, strip whitespace/`-`, `x` at index 2),
///   split `=promo`, accept len 4/5 core, validate squares + promo `NBRQ`.
/// Hint: mirror `board::parse_move_input`, but return `Move` instead of tuple.
pub fn parse_coordinate(raw: &str) -> Result<Move, String> {
    // TODO: implement; stub always errs so it compiles.
    Err(format!("TODO: san::parse_coordinate not implemented (got `{raw}`)"))
}

/// Inputs: raw castle text (trimmed). Output: `Some(true)` = kingside
///   `O-O`, `Some(false)` = queenside `O-O-O`, `None` = not castling.
/// TODO: uppercase, accept `O-O`, `0-0`, `O-O-O`, `0-0-0` with `-`/`.`/space
///   separators stripped; reject anything else.
/// Hint: normalize by removing `-`, `.`, spaces then matching `OO` / `OOO`.
pub fn normalize_castle_input(s: &str) -> Option<bool> {
    // TODO: implement; stub returns None so it compiles.
    let _ = s;
    None
}

/// Inputs: full SAN string. Output: `(core_without_suffix, is_check, is_mate)`.
/// TODO: strip one trailing `+` (check) or `#` (mate); `++` counts as mate
///   in old notation — accept and map to mate. Return borrowed `core`.
/// Hint: pure string helper; no board needed. Used by both parse + generate.
pub fn strip_check_mate_suffix(s: &str) -> (&str, bool, bool) {
    // TODO: implement; stub returns input untouched so it compiles.
    (s, false, false)
}

/// Inputs: promotion char (any case). Output: canonical piece short name
///   `Some("N"/"B"/"R"/"Q")` or `None` for `P`/`K`/other.
/// TODO: uppercase then match; `None` for invalid.
/// Hint: shares the `NBRQ` allow-list with `board::parse_move_input`.
pub fn promotion_piece_from_char(c: char) -> Option<&'static str> {
    // TODO: implement; stub returns None so it compiles.
    let _ = c;
    None
}

/// Inputs: board *before* the move + the move. Output: disambiguation text
///   `""/file/rank/both` for `to_san` (e.g. two knights can reach e2 ->
///   `"Nbd2"` vs `"Nfd2"`).
/// TODO: find other same-type pieces with a legal move to the same `to`
///   (via `engine::legal_moves_for`); if none -> `""`; else if no other
///   shares the file -> file letter; else if no other shares rank -> rank;
///   else both.
/// Hint: pure `to_san` helper; keep I/O-free for easy testing.
pub fn needs_disambiguation(board_before: &Board, mv: &Move) -> String {
    // TODO: implement; stub returns empty so it compiles.
    let _ = (board_before, mv);
    String::new()
}

/// Inputs: board (side to move) + SAN text like `"Nf3"`, `"exd5"`, `"O-O"`.
/// Output: the unique matching legal `Move`, or `Err` describing ambiguity
///   / no-match.
/// TODO: strip `+`/`#` suffix; castle branch via `normalize_castle_input`
///   (match `all_legal_moves` with `is_castle` + kingside flag); else parse
///   `[Piece]?[disamb]?[x?][square][=Promo]?`, filter `all_legal_moves` by
///   piece type + destination + capture flag + disambiguation + promotion,
///   error on 0 (`"no legal move matches ..."`) or 2+ (`"ambiguous ..."`) .
/// Hint: pawn moves start with file `a-h` and have no leading piece letter;
///   `x` means capture (pawns must include origin file, e.g. `exd5`).
pub fn parse_san(board: &Board, raw: &str) -> Result<Move, String> {
    // TODO: implement; stub always errs so it compiles.
    let _ = board;
    Err(format!("TODO: san::parse_san not implemented (got `{raw}`)"))
}

/// Inputs: board *before* the move + the applied `Move`.
/// Output: SAN string like `"e4"`, `"Nf3"`, `"exd8=Q+"`, `"O-O"`, `"Qxe7#"`.
/// TODO: castle -> `O-O`/`O-O-O`; else piece letter (pawns: `` + capture file
///   + `x` if capture), + `needs_disambiguation`, destination, `=PROMO`,
///   then simulate the move and append `+`/`#` via
///   `engine::king_in_check` + `engine::has_any_legal` on the opponent.
/// Hint: capture = destination occupied OR `is_en_passant`; pawns show file
///   only on captures (`exd5` vs `e4`).
pub fn to_san(board_before: &Board, mv: &Move) -> String {
    // TODO: implement; stub returns empty so it compiles.
    let _ = (board_before, mv);
    String::new()
}

/// Inputs: move. Output: coordinate log form `"e2-e4"` / `"e2xe4"`-family
///   (capture uses `x`, quiet uses `-`, plus `=Q` suffix if promotion).
/// TODO: needs destination occupancy — take a `&Board` too, or accept a
///   `captured: bool` param in the real signature (note the tension in TODO).
///   For now the stub ignores capture detection.
/// Hint: mirrors the `format!` in `Board::try_move`; keep log format stable.
pub fn move_to_coordinate(mv: &Move) -> String {
    // TODO: implement capture-aware formatting; stub returns empty.
    let _ = mv;
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    fn fresh() -> Board {
        Board::new_board()
    }

    #[test]
    #[ignore = "TODO: implement san::sq_name"]
    fn sq_name_maps() {
        assert_eq!(sq_name(4, 1), "e2");
        assert_eq!(sq_name(0, 0), "a1");
        assert_eq!(sq_name(7, 7), "h8");
    }

    #[test]
    #[ignore = "TODO: implement san::parse_square"]
    fn parse_square_accepts_rejects() {
        assert_eq!(parse_square("e2"), Some((4, 1)));
        assert_eq!(parse_square("a1"), Some((0, 0)));
        assert_eq!(parse_square("i9"), None);
        assert_eq!(parse_square("e"), None);
    }

    #[test]
    #[ignore = "TODO: implement san::parse_coordinate"]
    fn parse_coordinate_family() {
        assert!(parse_coordinate("e2e4").is_ok());
        assert!(parse_coordinate("e2 e4").is_ok());
        assert!(parse_coordinate("e2-e4").is_ok());
        assert!(parse_coordinate("e2xe4").is_ok());
        assert!(parse_coordinate("e7e8=Q").is_ok());
        assert!(parse_coordinate("junk").is_err());
    }

    #[test]
    #[ignore = "TODO: implement san::normalize_castle_input"]
    fn castle_normalization() {
        assert_eq!(normalize_castle_input("O-O"), Some(true));
        assert_eq!(normalize_castle_input("O-O-O"), Some(false));
        assert_eq!(normalize_castle_input("0-0"), Some(true));
        assert_eq!(normalize_castle_input("0-0-0"), Some(false));
        assert_eq!(normalize_castle_input("e4"), None);
    }

    #[test]
    fn strip_suffix_smoke() {
        // Stub passes trivially; real test below is the ignored one.
        let (core, _, _) = strip_check_mate_suffix("Nf3");
        assert_eq!(core, "Nf3");
    }

    #[test]
    #[ignore = "TODO: implement san::strip_check_mate_suffix"]
    fn strip_suffix_real() {
        assert_eq!(strip_check_mate_suffix("Qxe7+"), ("Qxe7", true, false));
        assert_eq!(strip_check_mate_suffix("Qxe7#"), ("Qxe7", false, true));
        assert_eq!(strip_check_mate_suffix("e4"), ("e4", false, false));
    }

    #[test]
    #[ignore = "TODO: implement san::promotion_piece_from_char"]
    fn promo_char_map() {
        assert_eq!(promotion_piece_from_char('Q'), Some("Q"));
        assert_eq!(promotion_piece_from_char('n'), Some("N"));
        assert_eq!(promotion_piece_from_char('P'), None);
        assert_eq!(promotion_piece_from_char('K'), None);
        assert_eq!(promotion_piece_from_char('x'), None);
    }

    #[test]
    #[ignore = "TODO: implement san::parse_san"]
    fn parse_san_startpos() {
        let b = fresh();
        assert!(parse_san(&b, "e4").is_ok());
        assert!(parse_san(&b, "Nf3").is_ok());
        assert!(parse_san(&b, "O-O").is_err()); // blocked startpos
        assert!(parse_san(&b, "junk").is_err());
    }

    #[test]
    #[ignore = "TODO: implement san::to_san"]
    fn to_san_shapes() {
        let b = fresh();
        let e4 = parse_coordinate("e2e4").unwrap();
        assert_eq!(to_san(&b, &e4), "e4");
        let nf3 = parse_san(&b, "Nf3").unwrap();
        // After e4 played the SAN is still Nf3 from Black's perspective? Keep
        // fixtures single-ply here: from startpos Nf3 is just "Nf3".
        let _ = nf3;
    }

    #[test]
    #[ignore = "TODO: implement san::to_san disambiguation"]
    fn disambiguation_ngf3_vs_nbd2() {
        let _b = fresh();
        // TODO: build a position where two knights reach the same square,
        // assert needs_disambiguation returns file/rank.
    }

    #[test]
    fn move_to_coordinate_stub_callable() {
        let m = crate::mv::Move::quiet(
            crate::mv::Coord::new(4, 1),
            crate::mv::Coord::new(4, 3),
        );
        let _ = move_to_coordinate(&m); // stub returns "" for now
    }
}

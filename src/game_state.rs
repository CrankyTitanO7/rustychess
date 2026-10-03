// Mutable chess state that is NOT the piece grid itself. Learning path: see `src/TODO.md` Phase 0.
// Teaches `&self` vs `&mut self`, enums, match, Option — start here after `mv.rs`.
//
// `Board.board` holds *where* pieces are; `GameState` holds the extra
// rules state needed for legal play: side to move, castling rights,
// en-passant target, halfmove clock (50-move rule) and fullmove number.
//
// Hints:
// - Long-term `Board::turn_is_black` (derived from `log.num_moves`) should
//   delegate to `GameState.turn_black` so undos / loaded positions work.
// - `en_passant` is the *empty* square behind a double-pushed pawn,
//   e.g. White `e2-e4` sets `Some((4,2))` == `"e3"`.
// - Halfmove resets on pawn move or capture, else +1. Fullmove +1 after Black.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrawReason {
    FiftyMove,
    ThreefoldRepetition,
    InsufficientMaterial,
    Agreement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameResult {
    Ongoing,
    Checkmate { black_wins: bool },
    Stalemate,
    Draw { reason: DrawReason },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameState {
    /// `false` = White to move, `true` = Black to move.
    pub turn_black: bool,
    pub castling: CastlingRights,
    /// En-passant target square `(x,y)` 0-based, if the last move was a
    /// double pawn push. `None` otherwise (clears after every move).
    pub en_passant: Option<(u8, u8)>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

impl CastlingRights {
    /// Inputs: none. Output: all rights `false`.
    /// TODO: construct with all `false` (already implemented).
    pub fn none() -> Self {
        Self {
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
        }
    }

    /// Inputs: none. Output: standard startpos rights (all `true`).
    /// TODO: construct with all `true` (already implemented).
    pub fn initial() -> Self {
        Self {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }

    /// Inputs: `black` side, `kingside` side selector.
    /// Output: whether that side may castle that way.
    /// TODO: match on `(black, kingside)` to the right field.
    /// Hint: pure getter; 4-way branch.
    pub fn can_castle(&self, black: bool, kingside: bool) -> bool {
        // TODO: return the matching field; stub returns garbage.
        match black {
            true => {
                match kingside {
                    true => {
                        if self.black_kingside {true}
                        else {false}
                    }
                    false => {
                        if self.black_queenside {true}
                        else {false}
                    }
                }
            }
            false => {
                match kingside {
                    true => {
                        if self.black_kingside {true}
                        else {false}
                    }
                    false => {
                        if self.black_queenside {true}
                        else {false}
                    }
                }
            }
        }
    }

    /// Inputs: `black` side. Effect: clears ONLY that side's kingside flag.
    /// TODO: `if black { self.black_kingside = false } else { ... }`.
    pub fn clear_kingside(&mut self, black: bool) {
        // TODO: implement; stub does nothing so it compiles.
        // let _ = black;
        if black {self.black_kingside = false} else {self.white_kingside = false}
    }

    /// Inputs: `black` side. Effect: clears ONLY that side's queenside flag.
    /// TODO: mirror `clear_kingside`.
    pub fn clear_queenside(&mut self, black: bool) {
        // TODO: implement; stub does nothing so it compiles.
        if black {self.black_queenside = false} else {self.white_queenside = false}
        // let _ = black;
    }

    /// Inputs: `black` side. Effect: clears BOTH flags for that color.
    /// TODO: call `clear_kingside` + `clear_queenside`, or set directly.
    /// Hint: used when that side's king moves.
    pub fn clear_all_for(&mut self, black: bool) {
        // TODO: implement; stub does nothing so it compiles.
        if black {(self.black_kingside , self.black_queenside) = (false, false)}
        else {(self.white_kingside , self.white_queenside) = (false, false)}
        // let _ = black;
    }

    /// Inputs: moving piece short name (`"K"`/`"R"`), from-square, captured
    ///   piece (if any) + its square. Effect: update rights.
    /// Output: nothing.
    /// TODO: king move -> `clear_all_for(mover)`; rook from a1/h1/a8/h8 ->
    ///   clear matching side; capture on those squares -> clear victim's side.
    /// Hint: needs startpos corner constants: a1=(0,0) h1=(7,0) a8=(0,7) h8=(7,7).
    pub fn update_on_move(
        &mut self,
        mover_short: &str,
        mover_black: bool,
        from: (u8, u8),
        captured_short: Option<&str>,
        capture_square: Option<(u8, u8)>,
    ) {
        // TODO: implement rights stripping; stub does nothing so it compiles.
        // let _ = (mover_short, mover_black, from, captured_short, capture_square);

        if mover_short == "K" {
            // immediate disqualify if your king pieces move.
            {self.clear_all_for(mover_black);}
        } else {
            // define rook squares
            const A1:(u8, u8) = (0, 0); 
            const A8:(u8, u8) = (7, 0);
            const H1:(u8, u8)= (0, 7);
            const H8:(u8, u8) = (7, 7);

            if mover_short == "R" {
                // if you moved your rook from a starting position
                match from {
                    A1 | H1=>{ self.clear_queenside(mover_black) }, 
                    A8 | H8=>{ self.clear_kingside(mover_black) }, 
                    _ => {} // otherwise do nothing
                }
            }

            if captured_short == Some("R") {
                // if your rook has been captured
                match capture_square.unwrap() {
                    A1 | H1=>{ self.clear_queenside(mover_black) }, 
                    A8 | H8=>{ self.clear_kingside(mover_black) }, 
                    _ => {} // otherwise do nothing
                }
            }
        }
    }
}

impl GameState {
    /// Inputs: none. Output: fresh state, White to move, no rights, clocks 0/1.
    /// TODO: already implemented (baseline for custom positions/tests).
    pub fn new() -> Self {
        Self {
            turn_black: false,
            castling: CastlingRights::none(),
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    /// Inputs: none. Output: standard startpos state (White, all castling,
    ///   no e.p., halfmove 0, fullmove 1).
    /// TODO: already implemented; keep in sync with `board.rs` startpos.
    pub fn initial() -> Self {
        Self {
            turn_black: false,
            castling: CastlingRights::initial(),
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    /// Inputs: `&self`. Output: `true` if Black to move.
    /// TODO: return `self.turn_black` (already implemented).
    pub fn turn_is_black(&self) -> bool {
        self.turn_black
    }

    /// Inputs: `&self`. Output: `"White"` or `"Black"`.
    /// TODO: branch on `turn_black` (already implemented).
    pub fn turn_name(&self) -> &'static str {
        if self.turn_black {
            "Black"
        } else {
            "White"
        }
    }

    /// Inputs: `&mut self`. Effect: flip side to move; if Black just moved
    ///   (i.e. flipping back to White), increment `fullmove_number`.
    /// TODO: `self.turn_black = !self.turn_black; if !self.turn_black { fullmove += 1 }`
    ///   plus clear `en_passant`? (Decide: e.p. clears here OR in `set_en_passant`
    ///   caller — document choice in engine.)
    /// Hint: fullmove starts at 1 and increments after Black's move.
    pub fn advance_turn(&mut self) {
        if self.turn_is_black() {self.fullmove_number += 1}
        self.turn_black = !self.turn_black;
    }

    /// Inputs: `&mut self`, target `(x,y)`. Effect: set e.p. square.
    /// TODO: `self.en_passant = Some(target)` (one-liner, already done shape).
    pub fn set_en_passant(&mut self, target: (u8, u8)) {
        // let _ = target;
        self.en_passant = Some(target);
        // TODO: store target; stub does nothing so it compiles.
    }

    /// Inputs: `&mut self`. Effect: clear e.p. square.
    /// TODO: `self.en_passant = None`.
    pub fn clear_en_passant(&mut self) {
        self.en_passant = None;
        // TODO: implement; stub does nothing so it compiles.
    }

    /// Inputs: `&mut self`. Effect: reset 50-move clock to 0.
    /// TODO: called on pawn move or capture.
    pub fn reset_halfmove(&mut self) {
        self.halfmove_clock = 0;
        // TODO: implement; stub does nothing so it compiles.
    }

    /// Inputs: `&mut self`. Effect: increment 50-move clock by 1.
    /// TODO: called on quiet non-pawn moves.
    pub fn bump_halfmove(&mut self) {
        self.halfmove_clock += 1;
        // TODO: implement; stub does nothing so it compiles.
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn castling_none_is_all_false() {
        let r = CastlingRights::none();
        assert!(!r.white_kingside && !r.white_queenside && !r.black_kingside && !r.black_queenside);
    }

    #[test]
    fn castling_initial_is_all_true() {
        let r = CastlingRights::initial();
        assert!(r.white_kingside && r.white_queenside && r.black_kingside && r.black_queenside);
    }

    #[test]
    // #[ignore = "TODO: implement CastlingRights::can_castle"]
    fn can_castle_selects_field() {
        let r = CastlingRights::initial();
        assert!(r.can_castle(false, true));
        assert!(r.can_castle(false, false));
        assert!(r.can_castle(true, true));
        assert!(r.can_castle(true, false));
        let n = CastlingRights::none();
        assert!(!n.can_castle(false, true));
    }

    #[test]
    // #[ignore = "TODO: implement CastlingRights::clear_kingside"]
    fn clear_kingside_only_that_side() {
        let mut r = CastlingRights::initial();
        r.clear_kingside(false);
        assert!(!r.white_kingside);
        assert!(r.white_queenside);
        assert!(r.black_kingside);
    }

    #[test]
    // #[ignore = "TODO: implement CastlingRights::clear_queenside"]
    fn clear_queenside_only_that_side() {
        let mut r = CastlingRights::initial();
        r.clear_queenside(true);
        assert!(!r.black_queenside);
        assert!(r.black_kingside);
        assert!(r.white_queenside);
    }

    #[test]
    // #[ignore = "TODO: implement CastlingRights::clear_all_for"]
    fn clear_all_for_one_color() {
        let mut r = CastlingRights::initial();
        r.clear_all_for(false);
        assert!(!r.white_kingside && !r.white_queenside);
        assert!(r.black_kingside && r.black_queenside);
    }

    #[test]
    // #[ignore = "TODO: implement CastlingRights::update_on_move"]
    fn update_on_move_strips_rights() {
        let mut r = CastlingRights::initial();
        // White king moves -> white loses both.
        r.update_on_move("K", false, (4, 0), None, None);
        assert!(!r.white_kingside && !r.white_queenside);
        // Black queenside rook captured on a8 -> black loses queenside only.
        let mut r2 = CastlingRights::initial();
        r2.update_on_move("Q", false, (0, 1), Some("R"), Some((0, 7)));
        assert!(!r2.black_queenside);
        assert!(r2.black_kingside);
    }

    #[test]
    fn game_state_new_and_initial() {
        let s = GameState::new();
        assert!(!s.turn_black);
        assert_eq!(s.fullmove_number, 1);
        let i = GameState::initial();
        assert_eq!(i.castling, CastlingRights::initial());
        assert_eq!(i.en_passant, None);
    }

    #[test]
    fn turn_name_matches_flag() {
        let mut s = GameState::initial();
        assert_eq!(s.turn_name(), "White");
        s.turn_black = true;
        assert!(s.turn_is_black());
        assert_eq!(s.turn_name(), "Black");
    }

    #[test]
    // #[ignore = "TODO: implement GameState::advance_turn"]
    fn advance_turn_flips_and_counts_fullmove() {
        let mut s = GameState::initial();
        s.advance_turn(); // White -> Black, still move 1
        assert!(s.turn_black);
        assert_eq!(s.fullmove_number, 1);
        s.advance_turn(); // Black -> White, now move 2
        assert!(!s.turn_black);
        assert_eq!(s.fullmove_number, 2);
    }

    #[test]
    // #[ignore = "TODO: implement en-passant setters"]
    fn en_passant_set_clear() {
        let mut s = GameState::initial();
        s.set_en_passant((4, 2));
        assert_eq!(s.en_passant, Some((4, 2)));
        s.clear_en_passant();
        assert_eq!(s.en_passant, None);
    }

    #[test]
    // #[ignore = "TODO: implement halfmove clock"]
    fn halfmove_reset_bump() {
        let mut s = GameState::initial();
        s.bump_halfmove();
        assert_eq!(s.halfmove_clock, 1);
        s.reset_halfmove();
        assert_eq!(s.halfmove_clock, 0);
    }
}

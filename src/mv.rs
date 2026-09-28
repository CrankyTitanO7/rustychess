// Core move types for rustychess. Learning path: see `src/TODO.md` Phases 0-1.
// Each function below teaches one Rust idea (structs, Option, String, match);
// implement in TODO.md order, one ignored test at a time.
//
// `Coord` is a single square (0-based `x` file a-h, `y` rank 1-8).
// `Move` is a from/to pair plus flags the engine/SAN layers need.
//
// Hints:
// - Keep `Move: Copy` so engine search can pass moves by value cheaply.
// - `color: bool` convention (`false=white`, `true=black`) lives in
//   `constants.rs:4`; move types stay color-agnostic.
// - UCI shape is `e2e4` / `e7e8q` (promotion lowercase trailing char).

use crate::constants as C;
use crate::board as B;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub from: Coord,
    pub to: Coord,
    /// Promotion piece as uppercase `N`/`B`/`R`/`Q`, `None` = no promotion.
    pub promotion: Option<char>,
    /// `true` for O-O / O-O-O king moves (rook move is implied).
    pub is_castle: bool,
    /// `true` when the pawn captures en passant (destination is empty).
    pub is_en_passant: bool,
}

impl Coord {
    /// Inputs: 0-based file `x`, rank `y`.
    /// Output: the coordinate.
    /// TODO: nothing to do (trivial constructor, already implemented).
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    /// Inputs: `self`.
    /// Output: `true` if on an 8x8 board.
    /// TODO: check `x < 8 && y < 8` (uses `constants::XWIDTH/YWIDTH`).
    /// Hint: one-liner; used by engine to guard array indexing.
    pub fn is_valid(&self) -> bool {
        // TODO: implement bounds check; stub returns garbage so it compiles.
        if self.x as usize >= C::XWIDTH || self.y as usize >= C::YWIDTH  { false } else { true }
    }

    /// Inputs: `self`.
    /// Output: algebraic square name, e.g. `(4,1)` -> `"e2"`.
    /// TODO: `(b'a' + x) as char` + `(y+1)` string.
    /// Hint: see `board.rs` private `sq_name` for reference logic.
    pub fn to_algebraic(&self) -> String {
        // TODO: format file/rank; stub returns empty so it compiles.
        // String::new()
        B::sq_name(self.x.into(), self.y.into())
    }

    /// Inputs: square text like `"e2"`.
    /// Output: `Some(Coord)` on files a-h + ranks 1-8, else `None`.
    /// TODO: parse 2 chars, validate ranges, map to 0-based.
    /// Hint: see `board.rs` private `parse_square` for reference logic.
    pub fn from_algebraic(s: &str) -> Option<Self> {
        // TODO: implement; stub returns None so it compiles.
        match B::parse_square(s) {
            Some ((x, y)) => {
                Some (Coord {
                    x: x.try_into().unwrap(), y: y.try_into().unwrap()
                    // panics if the number is too big
                })
            }, 
            _ => None
        }
        
    }
}

impl Move {
    /// Inputs: from/to coords, promotion (`Some('Q')` etc.), castle/e.p. flags.
    /// Output: the move value.
    /// TODO: nothing to do (trivial constructor, already implemented).
    pub fn new(
        from: Coord,
        to: Coord,
        promotion: Option<char>,
        is_castle: bool,
        is_en_passant: bool,
    ) -> Self {
        Self {
            from,
            to,
            promotion,
            is_castle,
            is_en_passant,
        }
    }

    /// Inputs: from/to coords.
    /// Output: a quiet non-promotion, non-special move.
    /// TODO: delegate to `Move::new` with `None, false, false` (already done).
    pub fn quiet(from: Coord, to: Coord) -> Self {
        Self::new(from, to, None, false, false)
    }

    /// Inputs: `self`.
    /// Output: `true` if `promotion.is_some()`.
    /// TODO: one-liner (already implemented, keep).
    pub fn is_promotion(&self) -> bool {
        self.promotion.is_some()
    }

    /// Inputs: `self`.
    /// Output: UCI string like `"e2e4"` or `"e7e8q"` (promo lowercase).
    /// TODO: `from.to_algebraic() + to.to_algebraic() + promo.to_lowercase()`.
    /// Hint: depends on `Coord::to_algebraic`.
    pub fn to_uci(&self) -> String {
        // TODO: implement; stub returns empty so it compiles.
        String::new()
    }

    /// Inputs: UCI text (`"e2e4"`, `"e7e8q"`, `"e7e8=Q"` variant optional).
    /// Output: `Some(Move)` on success, `None` on malformed input.
    /// TODO: accept len 4/5, parse squares via `Coord::from_algebraic`,
    ///   validate promo char in `NBRQ` (case-insensitive), set flags false
    ///   (castling/e.p. are resolved by engine/SAN layers, not UCI).
    /// Hint: mirror `board.rs::parse_move_input` core-4-chars logic.
    pub fn from_uci(s: &str) -> Option<Self> {
        // TODO: implement; stub returns None so it compiles.
        let _ = s;
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coord_new_stores_xy() {
        let c = Coord::new(4, 1);
        assert_eq!((c.x, c.y), (4, 1));
    }

    #[test]
    // #[ignore = "TODO: implement Coord::is_valid"]
    fn coord_is_valid_startpos_corners() {
        assert!(Coord::new(0, 0).is_valid());
        assert!(Coord::new(7, 7).is_valid());
        assert!(!Coord::new(8, 0).is_valid());
        assert!(!Coord::new(0, 8).is_valid());
    }

    #[test]
    // #[ignore = "TODO: implement Coord::to_algebraic"]
    fn coord_to_algebraic_e2() {
        assert_eq!(Coord::new(4, 1).to_algebraic(), "e2");
        assert_eq!(Coord::new(0, 0).to_algebraic(), "a1");
        assert_eq!(Coord::new(7, 7).to_algebraic(), "h8");
    }

    #[test]
    // #[ignore = "TODO: implement Coord::from_algebraic"]
    fn coord_from_algebraic_roundtrip() {
        assert_eq!(Coord::from_algebraic("e2"), Some(Coord::new(4, 1)));
        assert_eq!(Coord::from_algebraic("a1"), Some(Coord::new(0, 0)));
        assert_eq!(Coord::from_algebraic("h8"), Some(Coord::new(7, 7)));
        assert_eq!(Coord::from_algebraic("i9"), None);
        assert_eq!(Coord::from_algebraic("e"), None);
        assert_eq!(Coord::from_algebraic(""), None);
    }

    #[test]
    fn move_new_and_quiet_flags() {
        let a = Coord::new(4, 1);
        let b = Coord::new(4, 3);
        let m = Move::new(a, b, None, false, false);
        assert_eq!(m.from, a);
        assert_eq!(m.to, b);
        assert!(!m.is_promotion());
        let q = Move::quiet(a, b);
        assert_eq!(q, m);
    }

    #[test]
    fn move_is_promotion_detects_flag() {
        let m = Move::new(Coord::new(4, 6), Coord::new(4, 7), Some('Q'), false, false);
        assert!(m.is_promotion());
    }

    #[test]
    #[ignore = "TODO: implement Move::to_uci"]
    fn move_to_uci_shapes() {
        let m = Move::quiet(Coord::new(4, 1), Coord::new(4, 3));
        assert_eq!(m.to_uci(), "e2e4");
        let p = Move::new(Coord::new(4, 6), Coord::new(4, 7), Some('Q'), false, false);
        assert_eq!(p.to_uci(), "e7e8q");
    }

    #[test]
    #[ignore = "TODO: implement Move::from_uci"]
    fn move_from_uci_parses() {
        assert_eq!(
            Move::from_uci("e2e4"),
            Some(Move::quiet(Coord::new(4, 1), Coord::new(4, 3)))
        );
        let p = Move::from_uci("e7e8q").unwrap();
        assert_eq!(p.promotion, Some('Q'));
        assert_eq!(Move::from_uci("junk"), None);
        assert_eq!(Move::from_uci("e9e4"), None);
    }
}

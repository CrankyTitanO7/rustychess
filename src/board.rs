// file that defines the board struct, and its methods

use crate::constants as C;
use crate::game_state as GS;
use crate::log_move as L;
use crate::pieces as P;

pub struct Board {
    pub board : [[Option<P::Piece>; C::XWIDTH]; C::YWIDTH],
    pub log : L::Log,
    /// Rules state (side to move, castling, e.p., clocks).
    /// TODO(engine): migrate `turn_is_black` to `state.turn_black` so loaded
    /// positions/undos work; `log.num_moves` stays as move history only.
    pub state : GS::GameState,
}

fn fresh_constructor() -> [[Option<P::Piece>; C::XWIDTH]; C::YWIDTH] {
    let mut m: [[Option<P::Piece>; C::XWIDTH]; C::YWIDTH] =
        std::array::from_fn(|_| std::array::from_fn(|_| None));
    // Pawns: white (false) on rank y=1, black (true) on rank y=YWIDTH-2.
    // Note `constants.rs:4`: `false=white`, `true=black`.
    for y in [1, C::YWIDTH - 2] {
        let is_black = y == C::YWIDTH - 2;
        for x in 0..C::XWIDTH {
            m[y][x] = Some(P::Piece::inst("P", x as u8, y as u8, is_black));
        }
    }
    // Back rank: R N B Q K B N R on y=0 (white) and y=YWIDTH-1 (black).
    let back_rank = ["R", "N", "B", "Q", "K", "B", "N", "R"];
    for y in [0, C::YWIDTH - 1] {
        let is_black = y != 0;
        for x in 0..C::XWIDTH {
            m[y][x] = Some(P::Piece::inst(back_rank[x], x as u8, y as u8, is_black));
        }
    }

    m
}

impl Board {
    pub fn new_board () -> Board{
        Board {
            board : fresh_constructor(),
            log : L::Log::new_log(),
            state : GS::GameState::initial(),
        }
    }

    /// Whose turn is it? White moves first: even `num_moves` = White.
    /// Uses `Log::num_moves` so the move log is the source of truth.
    pub fn turn_is_black(&self) -> bool {
        self.log.num_moves % 2 == 1
    }

    pub fn turn_name(&self) -> &'static str {
        if self.turn_is_black() { "Black" } else { "White" }
    }

    /// Attempt a PvP move written as `e2 e4` / `e2e4` / `e2-e4` / `e2xe4`,
    /// with optional promotion suffix (`e7e8=Q`, `e7e8Q`).
    ///
    /// On success the piece is relocated (capturing any opponent piece),
    /// its `locx`/`locy` are updated, pawn promotion goes through
    /// `Piece::pawn_to_x`, and the move is recorded with
    /// `Log::add_move_to_log`. Returns the logged algebraic string.
    pub fn try_move(&mut self, raw: &str) -> Result<String, String> {
        let ((fx, fy), (tx, ty), promo) = parse_move_input(raw)?;

        if fx == tx && fy == ty {
            return Err("source and destination are the same square".to_string());
        }

        if self.board[fy][fx].is_none() {
            return Err(format!("no piece on {}", sq_name(fx, fy)));
        }

        // Enforce alternating turns from the log.
        let expect_black = self.turn_is_black();
        let mover_color = self.board[fy][fx].as_ref().unwrap().color;
        if mover_color != expect_black {
            return Err(format!(
                "it's {} to move ({} holds a {} piece)",
                self.turn_name(),
                sq_name(fx, fy),
                if mover_color { "Black" } else { "White" },
            ));
        }

        // Reject capturing your own piece.
        if let Some(dest) = self.board[ty][tx].as_ref() {
            if dest.color == mover_color {
                return Err(format!(
                    "own piece on {} ({})",
                    sq_name(tx, ty),
                    dest.name_long
                ));
            }
        }

        let captured = self.board[ty][tx].is_some();

        // Promotion validity is checked before touching the board so a
        // rejected input leaves the position unchanged.
        let is_pawn = self.board[fy][fx].as_ref().unwrap().name_short == "P";
        let mover_color = self.board[fy][fx].as_ref().unwrap().color;
        let reaches_last_rank =
            (!mover_color && ty == C::YWIDTH - 1) || (mover_color && ty == 0);
        if promo.is_some() && !(is_pawn && reaches_last_rank) {
            return Err(format!(
                "promotion {} given but {} is not a pawn reaching the last rank",
                promo.unwrap(),
                sq_name(fx, fy)
            ));
        }

        // Relocate: `take` the source, retarget its tracked square, place it.
        // Captures simply overwrite the destination.
        let mut moving = self.board[fy][fx].take().unwrap();
        moving.locx = tx as u8;
        moving.locy = ty as u8;

        // Pawn promotion uses `Piece::pawn_to_x` (defaults to queen).
        let promo_suffix = if is_pawn && reaches_last_rank {
            let p = promo.unwrap_or_else(|| "Q".to_string());
            moving.pawn_to_x(&p);
            format!("={}", p)
        } else {
            String::new()
        };

        self.board[ty][tx] = Some(moving);

        let alg = format!(
            "{}{}{}{}",
            sq_name(fx, fy),
            if captured { "x" } else { "-" },
            sq_name(tx, ty),
            promo_suffix,
        );
        self.log.add_move_to_log(&alg);
        Ok(alg)
    }

    /// Most recent logged move via `Log::movesearch`, if any.
    pub fn last_move(&self) -> Option<&String> {
        self.log.num_moves.checked_sub(1).and_then(|i| self.log.movesearch(i))
    }
}

/// `x,y` (0-based) -> `"e2"`.
pub fn sq_name(x: usize, y: usize) -> String {
    format!("{}{}", (b'a' + x as u8) as char, y + 1)
}

/// `"e2"` -> `(x,y)`. File `a-h` maps to `x 0-7`, rank `1-8` to `y 0-7`.
pub fn parse_square(s: &str) -> Option<(usize, usize)> {
    let mut chars = s.chars();
    let file = chars.next()?;
    let rank = chars.next()?;
    if chars.next().is_some() {
        return None;
    }
    if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank) {
        return None;
    }
    let x = (file as usize) - ('a' as usize);
    let y = (rank as usize) - ('1' as usize);
    if x < C::XWIDTH && y < C::YWIDTH {
        Some((x, y))
    } else {
        None
    }
}

/// Parse `e2 e4`-style input into `(from, to, promotion)`.
///
/// Accepts `e2e4`, `e2 e4`, `e2-e4`, `e2xe4`, plus `=Q` / trailing `Q`
/// promotion (`e7e8=Q`, `e7e8Q`). Promotion must be one of `N,B,R,Q`.
fn parse_move_input(raw: &str) -> Result<((usize, usize), (usize, usize), Option<String>), String> {
    // Normalise: lowercase, drop whitespace and `-`/`x` separators.
    // (`x`/`-` never occur inside a square name, so they must be separators.)
    let mut s: String = raw
        .trim()
        .to_ascii_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .collect();
    if s.contains('x') && (s.len() == 5 || s.len() == 7) {
        // Only strip the separator `x` in `e2xe4` / `e2xe4=q` shapes.
        let mut stripped = String::with_capacity(s.len());
        for (i, c) in s.chars().enumerate() {
            if c == 'x' && i == 2 {
                continue;
            }
            stripped.push(c);
        }
        s = stripped;
    }

    // Split optional `=P` promotion suffix.
    let (core, promo_eq) = match s.split_once('=') {
        Some((c, p)) => (c.to_string(), Some(p.to_string())),
        None => (s.clone(), None),
    };

    let (from_s, to_s, promo_trail): (String, String, Option<String>) = match core.len() {
        4 => (core[0..2].to_string(), core[2..4].to_string(), None),
        5 => (
            core[0..2].to_string(),
            core[2..4].to_string(),
            Some(core[4..5].to_string()),
        ),
        _ => {
            return Err(format!(
                "expected a move like `e2 e4` or `e2e4` (got `{raw}`)"
            ))
        }
    };

    if promo_eq.is_some() && promo_trail.is_some() {
        return Err("promotion given twice (use `e7e8=Q` or `e7e8Q`)".to_string());
    }
    let promo_raw = promo_eq.or(promo_trail);

    let from = parse_square(&from_s)
        .ok_or_else(|| format!("bad source square `{from_s}` (files a-h, ranks 1-8)"))?;
    let to = parse_square(&to_s)
        .ok_or_else(|| format!("bad destination square `{to_s}` (files a-h, ranks 1-8)"))?;

    let promo = match promo_raw {
        None => None,
        Some(p) => {
            let u = p.to_ascii_uppercase();
            // Allowed promotions reference `constants::ALG` subset (no P/K).
            if !["N", "B", "R", "Q"].contains(&u.as_str()) {
                return Err(format!(
                    "bad promotion `{p}` (choose one of N, B, R, Q, e.g. `e7e8=Q`)"
                ));
            }
            Some(u)
        }
    };

    Ok((from, to, promo))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_moves_first_then_black() {
        let mut b = Board::new_board();
        assert_eq!(b.turn_name(), "White");
        assert!(b.try_move("e2 e4").is_ok());
        assert_eq!(b.turn_name(), "Black");
        assert_eq!(b.log.num_moves, 1);
        assert_eq!(b.log.movesearch(0).unwrap(), "e2-e4");
        // White piece on black's turn is rejected.
        assert!(b.try_move("d2 d4").is_err());
        // Black replies.
        assert!(b.try_move("e7e5").is_ok());
        assert_eq!(b.log.num_moves, 2);
    }

    #[test]
    fn rejects_empty_source_and_own_capture() {
        let mut b = Board::new_board();
        assert!(b.try_move("e4 e5").is_err()); // empty square
        assert!(b.try_move("b1 b2").is_err()); // own pawn on b2
    }

    #[test]
    fn accepts_separators_and_promotes() {
        let mut b = Board::new_board();
        assert!(b.try_move("b1-c3").is_ok()); // knight, dash separator
        assert!(b.try_move("b8xc6").is_ok()); // black knight, x separator
        // Set up a white pawn one step from promotion and force white to move.
        b.board[6][0] = Some(P::Piece::inst("P", 0, 6, false));
        b.board[7][0] = None;
        b.board[7][1] = None;
        b.log.num_moves = 4; // even => White to move
        assert!(b.try_move("a7a8=Q").is_ok());
        assert_eq!(b.board[7][0].as_ref().unwrap().name_short, "Q");
    }
}
# RustyChess TODO — learn Rust by building the engine

> You had not mentioned learning Rust before — this file now re-orders
> everything so each function teaches one Rust idea at a time.
> Start at Phase 0 and go in order. Chess logic is secondary; Rust is primary.

## How the repo fits together

- `src/board.rs` — `Board { board: [[Option<Piece>; 8]; 8], log: Log, state: GameState }`.
  Grid is `[y][x]`, `y=0` is rank 1 (White home). `Piece.color: false=White`.
- `src/pieces.rs` — `Piece { name_short: "P/N/B/R/Q/K", color, locx, locy }`.
- `src/mv.rs` — `Coord {x,y}`, `Move {from,to,promotion,is_castle,is_en_passant}`. Color-agnostic.
- `src/game_state.rs` — side-to-move, castling rights, en-passant square, clocks.
- `src/engine.rs` — legal move generation + `make_move` + `game_result`.
- `src/san.rs` — coordinate parsing (`e2e4`) + SAN parsing/generation (`Nf3`, `O-O`).
- `src/ai.rs` — `evaluate_material` + random/greedy/minimax `choose_move`.
- `src/lib.rs` — re-exports core modules so `tests/*.rs` can `use rustychess::...`.

## How to work

1. Pick ONE ignored test: `cargo test <name> -- --ignored --nocapture`.
   Example: `cargo test coord_is_valid -- --ignored`.
2. Read that function's `TODO:` + `Learn:` + `Hint:` in `src/*.rs`, then this file's detail section.
3. Implement just that function (keep stubs for the rest returning garbage).
4. `cargo test` must stay green (ignored tests don't fail). `cargo build` must pass.
5. Remove the `#[ignore]` line only for tests you just made pass. Commit.

Full status: `cargo test 2>&1 | grep -E "ignored|ok$" | head`.

## Learning path (do in this order, not chess order)

| Phase | Rust idea you learn | Functions | Tests to un-ignore |
|-------|---------------------|-----------|--------------------|
| 0 | structs, `impl`, `Copy`, `Option`, `match`, `&self` vs `&mut self` | `mv::Coord::is_valid`, `ai::piece_value`, `san::promotion_piece_from_char`, `game_state::can_castle/clear_*` | `coord_is_valid`, `piece_values_sensible`, `promo_char_map`, `can_castle/clear_*` |
| 1 | `String`/`char`, `format!`, byte math, slicing | `mv::Coord::to_algebraic`, `san::sq_name`, `san::parse_square`, `mv::Coord::from_algebraic` | `coord_to_algebraic`, `sq_name_maps`, `parse_square_*`, `coord_from_algebraic` |
| 2 | borrowing `&Board`, `for` loops, indexing `Option`, `if let` | `engine::in_bounds`, `engine::find_king`, `ai::evaluate_material` | `in_bounds_*`, `find_king_*`, `material_startpos_even` |
| 3 | `Vec`, `push`, fixed arrays, offsets, early `continue` | `engine::knight_moves`, `engine::king_moves` (ring part only) | `knight_moves_startpos`, `king_moves_blocked` |
| 4 | `while`, rays, `break`, blocker logic | `engine::bishop_moves`, `rook_moves`, `queen_moves` | `bishop/rook/queen_*_blocked` |
| 5 | branching, multi-rule functions | `engine::pawn_moves` (push/double/capture/e.p./promo) | `pawn_moves_startpos` |
| 6 | iterators: `any`, `iter().filter().map()`, composition | `engine::is_attacked`, `king_in_check`, `pseudo_moves_for` | `is_attacked_startpos`, `king_in_check_*`, `pseudo_dispatches` |
| 7 | closures, `filter`, `collect`, ownership of `Vec<Move>` | `engine::legal_moves_for`, `all_legal_moves`, `has_any_legal` | `legal_filters_*`, `all_legal_moves_is_20`, `has_any_legal` |
| 8 | lifetimes (`&str -> &str`), `Result`, `strip_suffix`, error strings | `san::strip_check_mate_suffix`, `san::normalize_castle_input`, `san::parse_coordinate`, `mv::Move::to_uci/from_uci` | `strip_suffix_real`, `castle_normalization`, `parse_coordinate_family`, `move_to_uci/from_uci` |
| 9 | `&mut`, `Option::take`, save/restore, `Result` mutation | `engine::would_leave_king_in_check`, `engine::make_move`, `game_state::advance_turn/clocks/e.p.` | `pinned_move_*`, `make_move_applies`, `advance_turn/en_passant/halfmove` |
| 10 | enums with data, `match` exhaustiveness, `format!` generation | `engine::game_result/is_checkmate/is_stalemate/is_insufficient_material`, `san::needs_disambiguation/parse_san/to_san`, `game_state::update_on_move` | `game_result_*`, `mate_and_stalemate`, `insufficient_*`, `parse_san/to_san/disambiguation`, `update_on_move` |
| 11 | search, `max_by`, `sort`, deterministic fallback | `ai::random_move`, `greedy_move`, `minimax_move`, `choose_move` | `random/greedy/minimax/choose` |

## All tasks in detail

### `src/mv.rs` — pure value types (start here)

- [x] `Coord::new`, `Move::new/quiet/is_promotion` — done. Learn: `Self`, struct init shorthand.
- [x] `Coord::is_valid() -> bool`
  What: `x < 8 && y < 8` via `constants::XWIDTH/YWIDTH`.
  Learn: `&self` borrow (no move), boolean expr as return (no `return` needed).
  Hint: `self.x < C::XWIDTH as u8`.
- [x] `Coord::to_algebraic() -> String`
  What: `(4,1)->"e2"`. Formula: `(b'a'+x) as char` + `(y+1).to_string()`.
  Learn: `format!`, byte→char cast, owned `String` vs borrowed `&str`.
- [x] `Coord::from_algebraic(s: &str) -> Option<Self>`
  What: `"e2"->Some((4,1))`, else `None`. Check len==2, file `a-h`, rank `1-8`.
  Learn: `Option`, `chars().nth()`, early `return None`, `?` on `Option`.
  Pitfall: `s.len()` is bytes — fine for ASCII, note UTF-8 caveat.
- [ ] `Move::to_uci() -> String`
  What: `"e2e4"`, promo lowercased (`Some('Q')->"q"`).
  Learn: method calls on fields (`self.from.to_algebraic()`), `match promotion`.
- [ ] `Move::from_uci(s) -> Option<Self>`
  What: len 4/5, two squares + optional `NBRQ`. Flags `false`.
  Learn: string slicing `[0..2]` (ASCII-safe), `to_ascii_uppercase`, validation.

### `src/game_state.rs` — small state, `&mut self`

- [x] `CastlingRights::none/initial`, `GameState::new/initial/turn_is_black/turn_name` — done.
- [ ] `can_castle(black, kingside) -> bool`
  What: 4-way getter. Learn: `match (black, kingside)` tuple match.
- [ ] `clear_kingside/clear_queenside/clear_all_for(&mut self, black)`
  What: set one/both flags false. Learn: `&mut self`, `if black {} else {}` mutation.
- [ ] `advance_turn/set_en_passant/clear_en_passant/reset_halfmove/bump_halfmove`
  What: flip `turn_black`, fullmove+1 after Black; e.p. set/clear; clock reset/bump.
  Learn: mutation one-liners, `Option` assignment, `+= 1`. Decide e.p. clearing policy here vs `engine::make_move` and document it.
- [ ] `update_on_move(mover_short, mover_black, from, captured_short, capture_square)`
  What: king move clears mover; rook from/to a1/h1/a8/h8 clears that corner.
  Learn: `&str` comparison, `Option<&str>`, tuple equality `(x,y) == (0,0)`.
  Hint: corners `a1=(0,0) h1=(7,0) a8=(0,7) h8=(7,7)`.

### `src/engine.rs` — borrowing + loops + Vec

- [ ] `in_bounds(x: i32, y: i32) -> bool`
  What: `0<=x<8 && 0<=y<8`. Learn: `i32` for pre-check offsets (so `-1` is representable), comparisons.
- [ ] `find_king(board: &Board, black: bool) -> Option<(usize,usize)>`
  What: nested `for y in 0..8 { for x ... }`, match `board.board[y][x]` with `Some(p) if p.name_short=="K" && p.color==black`.
  Learn: shared borrow `&Board` (no mutation), `as_ref()` on `Option`, `if let Some(p) = ...`.
- [ ] `knight_moves / king ring`
  What: offset tables `[(1,2),(2,1),...]`, `in_bounds` guard, reject own piece.
  Learn: arrays, `for (dx,dy) in OFFSETS`, `Vec::push`, `as usize` after bounds check.
- [ ] `bishop_moves / rook_moves / queen_moves`
  What: ray walk: `let (mut cx, mut cy) = (x as i32 + dx, ...); while in_bounds { ... }`.
  Learn: `mut`, `while`, `break`, empty→push+continue / enemy→push+break / own→break. `queen = bishop + rook` via `extend`.
- [ ] `pawn_moves`
  What: dir `+1` white / `-1` black; 1-push if empty; 2-push from y=1/6 if both empty; captures incl. `board.state.en_passant`; promo expands to 4 moves.
  Learn: branching depth, constructing `Move` variants. Hardest generator — do last among pieces.
- [ ] `is_attacked(board,x,y,by_black) -> bool`
  What: pawn offsets (white attacks from `y-1`, black from `y+1`), knight jumps, king ring, 4 diagonal + 4 orthogonal rays (first blocker decides).
  Learn: reusing helpers vs raw scan, `||` short-circuit. Test with startpos knights.
- [ ] `king_in_check -> bool`
  What: `find_king(...).map(|(kx,ky)| is_attacked(..., !black)).unwrap_or(false)`.
  Learn: `Option::map`, `unwrap_or`, function composition.
- [ ] `pseudo_moves_for -> Vec<Move>`
  What: `match piece.name_short { "P"=>pawn_moves, "N"=>..., _=>vec![] }`.
  Learn: `match` on `&str` (`as_str()`), dispatch. Return `vec![]` on empty square.
- [ ] `legal_moves_for / all_legal_moves / has_any_legal`
  What: `pseudo.into_iter().filter(|m| !would_leave...).collect()`; `all_` loops own pieces (`p.color == board.state.turn_black`); `has_any` early-returns.
  Learn: iterators, closures `|m|`, `collect::<Vec<_>>()`, `any()`.
- [ ] `would_leave_king_in_check(board: &Board, mv: &Move) -> bool`
  What: apply `mv` on scratch state, test `king_in_check`, undo. Must handle e.p. + castling squares.
  Learn: THE ownership lesson. `Piece` is not `Clone` — either `#[derive(Clone)]` on `Piece` first, or `Option::take` + manual restore. Document choice.
- [ ] `make_move(board: &mut Board, mv: &Move) -> Result<String,String>`
  What: validate `mv ∈ legal_moves_for(from)` first (no mutation on `Err`), then relocate (`take`, update `locx/locy`), remove e.p. pawn, move rook if castle, `pawn_to_x` if promo, update `GameState` + `Log::add_move_to_log`.
  Learn: `&mut Board`, `Result::Err` with message, mutation ordering. Mirror `Board::try_move` relocation, then add state.
- [ ] `is_insufficient_material / game_result / is_checkmate / is_stalemate`
  What: v1 `insufficient` = no P/R/Q and minors ≤1; `game_result` = no-legals→mate/stale, else draws, else `Ongoing`.
  Learn: `enum` with data (`Checkmate{black_wins}`), exhaustive `match`, `if/else` chains.

### `src/san.rs` — strings + `Result` + lifetimes

- [ ] `sq_name / parse_square` — same as `mv` versions; centralize here later (delete `board.rs` privates, `pub use`).
  Learn: deduplication, `pub use`, module paths.
- [ ] `strip_check_mate_suffix(s: &str) -> (&str, bool, bool)`
  What: strip trailing `+`/`#` (`++`→mate). Returns borrowed slice of input.
  Learn: LIFETIMES — output borrows input (`fn f<'a>(s: &'a str) -> &'a str`). `strip_suffix`, tuple return.
- [ ] `normalize_castle_input -> Option<bool>`
  What: strip `- . space`, uppercase, `OO`→kingside, `OOO`→queenside.
  Learn: `to_ascii_uppercase`, `retain`/`filter`, `Option`.
- [ ] `promotion_piece_from_char -> Option<&'static str>`
  What: `NBRQ`→`Some`, else `None`. Learn: `match c.to_ascii_uppercase()`, `&'static str` (string lives forever).
- [ ] `parse_coordinate(raw) -> Result<Move,String>`
  What: port `board::parse_move_input` but return `Move`. Learn: `Result::Ok/Err`, `format!` errors, `?` on `Result`.
- [ ] `parse_san / to_san / needs_disambiguation / move_to_coordinate`
  What: all via `engine::all_legal_moves` filtering (never hand-roll movement here).
  Learn: `iter().find()`, counting matches (`0→Err(no match)`, `2+→Err(ambiguous)`), `format!` generation, check/mate suffix via simulated `king_in_check + has_any_legal`.

### `src/ai.rs` — composition + fix the `rand` question

- [ ] `piece_value(&str) -> i32` — `match "P"=>100, "N"=>320, ...`. Learn: `match` with fallback `_ => 0`.
- [ ] `evaluate_material(&Board) -> i32` — scan 64, `+v` white / `-v` black. Learn: nested loops + accumulator `let mut score = 0`.
- [ ] `random_move` — `all_legal_moves` + pick. v1 `.first().copied()` deterministic; v2 add `rand` dep. Learn: `Option`, `.copied()` on `Option<Move>` (needs `Copy`).
- [ ] `greedy_move` — 1-ply: best `evaluate_material` from mover's view. Learn: `max_by_key` / `min_by_key`, `White=max, Black=min`.
- [ ] `minimax_move(depth)` — needs clone/unmake story from engine. Learn: recursion, `u32` depth, base case.
- [ ] `choose_move` — dispatcher. Learn: fallback chain with `or_else`.

### Cross-cutting (do when blocked)

- [ ] `Piece: Clone` — derive it so engine search can clone boards. Learn: `#[derive]`; why `Copy` is wrong here (`String` fields).
- [ ] `Board::turn_is_black` → delegate to `state.turn_black`. Learn: single source of truth.
- [ ] `src/lib.rs` ↔ `src/main.rs` module sync. Learn: binary vs library crates sharing files.
- [ ] `move_to_coordinate` signature tension: needs `captured: bool` or `&Board`. Learn: API design — when a stub's signature is insufficient, change it + fix callers.

## Definition of done

- `cargo test` green; each implemented function has its `#[ignore]` removed.
- `cargo test -- --ignored` shows only genuinely unstarted work.
- `parse_san`/`to_san` round-trip on startpos + Scholar's/Fool's mate.
- `all_legal_moves` startpos == 20. `game_result` detects mate + stalemate fixtures.
- `choose_move` plays a legal game via `gameloops` without panic.

// Library root for integration tests (`tests/` uses `rustychess::...`).
//
// The binary (`src/main.rs`) declares the same modules privately; this file
// re-declares the core (non-TUI) modules publicly so `cargo test` can import
// them without pulling the ratatui game loops.
//
// Kept in sync with `src/main.rs` module list by hand.

pub mod ai;
pub mod board;
pub mod constants;
pub mod engine;
pub mod game_state;
pub mod log_move;
pub mod mv;
pub mod pieces;
pub mod san;

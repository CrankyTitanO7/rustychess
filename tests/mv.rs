// Integration smoke tests for `mv` via the library crate.
// Unit tests in `src/mv.rs` cover each function in depth; these prove the
// `src/lib.rs` wiring and give end-to-end entry points.
// Run: `cargo test` (ignored TODOs run with `cargo test -- --ignored`).

use rustychess::mv::{Coord, Move};

#[test]
fn coord_constructs() {
    let c = Coord::new(4, 1);
    assert_eq!((c.x, c.y), (4, 1));
}

#[test]
fn move_quiet_and_promotion_flag() {
    let a = Coord::new(4, 1);
    let b = Coord::new(4, 3);
    let m = Move::quiet(a, b);
    assert_eq!(m.from, a);
    assert!(!m.is_promotion());
    let p = Move::new(a, b, Some('Q'), false, false);
    assert!(p.is_promotion());
}

#[test]
#[ignore = "TODO: implement Coord::to_algebraic / from_algebraic"]
fn coord_algebraic_roundtrip() {
    assert_eq!(Coord::new(4, 1).to_algebraic(), "e2");
    assert_eq!(Coord::from_algebraic("e2"), Some(Coord::new(4, 1)));
}

#[test]
#[ignore = "TODO: implement Move::to_uci / from_uci"]
fn move_uci_roundtrip() {
    let m = Move::quiet(Coord::new(4, 1), Coord::new(4, 3));
    assert_eq!(m.to_uci(), "e2e4");
    assert_eq!(Move::from_uci("e2e4"), Some(m));
}

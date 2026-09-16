pub const XWIDTH:usize = 8;
pub const YWIDTH:usize = 8;

// bool:  0 = white
pub struct piece {
    name_long: String, 
    name_short: String, 
    symbol : char, 
    color: bool, 
    locx : u8, 
    locy: u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xwidth_is_8() {
        assert_eq!(XWIDTH, 8);
    }

    #[test]
    fn ywidth_is_8() {
        assert_eq!(YWIDTH, 8);
    }

    #[test]
    fn board_area_is_64() {
        assert_eq!(XWIDTH * YWIDTH, 64);
    }

    #[test]
    fn piece_stores_fields() {
        let p = piece {
            name_long: String::from("pawn"),
            name_short: String::from("p"),
            symbol: 'p',
            color: false, // 0 = white per comment above
            locx: 0,
            locy: 1,
        };
        assert_eq!(p.name_long, "pawn");
        assert_eq!(p.name_short, "p");
        assert_eq!(p.symbol, 'p');
        assert!(!p.color);
        assert_eq!(p.locx, 0);
        assert_eq!(p.locy, 1);
    }
}
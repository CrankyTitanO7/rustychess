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
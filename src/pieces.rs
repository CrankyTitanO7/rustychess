use crate::constants as C;

pub struct Piece {
    pub name_long: String, 
    pub name_short: String, 
    pub symbol : String, 
    pub color: bool, // false = white, true = black
    pub locx : u8, 
    pub locy: u8
}

impl Piece {
    pub fn piecebegin() {
        C::KvTable::create_kv();
    }

    // a function that instantiates a new piece at x y 
    pub fn inst (acr:&str, x:u8, y: u8, color:bool) -> Piece{

        let tmppiece = acr.to_string();

        Piece {
            name_long: C::KvTable::find_by_key(&tmppiece, &PIECE_LONG_LOOKUP_TABLE).unwrap().clone(),
            name_short: tmppiece.clone(),
            symbol: C::KvTable::find_by_key(&tmppiece,if color {&PIECE_SYM_BL_LOOKUP_TABLE} else {&PIECE_SYM_WH_LOOKUP_TABLE}).unwrap().clone(),
            color, 
            locx: x,
            locy: y,
        }
    }

    // a function that upgrades pawns into other objects
    pub fn pawn_to_x(&mut self, acr: &str) {
        self.name_long = C::KvTable::find_by_key(&acr.to_string(), &PIECE_LONG_LOOKUP_TABLE).unwrap().clone();
        self.name_short = acr.to_string();
        // Select table based on color, then look up the symbol
        let table = if self.color {
            &*PIECE_SYM_BL_LOOKUP_TABLE
        } else {
            &*PIECE_SYM_WH_LOOKUP_TABLE
        };
        self.symbol = C::KvTable::find_by_key(&acr.to_string(), table).unwrap().clone();
    }
}



use once_cell::sync::Lazy; // Alternative: use std::sync::OnceLock;

// 1. Define static lookup table globally
pub static PIECE_LONG_LOOKUP_TABLE: Lazy<C::KvTable> = Lazy::new(|| {
    let mut table = C::KvTable::create_kv(); 
    for i in 0..C::UNIQUE_PIECES {
        // Note: ALG[i].to_string() allocates a String on the heap, 
        // which requires a lazy/runtime static initializer like this.
        C::KvTable::push_new(C::ALG[i].to_string(), C::FUL[i].to_string(), &mut table); 
    }
    table // Return the initialized table
});

pub static PIECE_SHORT_LOOKUP_TABLE: Lazy<C::KvTable> = Lazy::new(|| {
    let mut table = C::KvTable::create_kv(); 
    for i in 0..C::UNIQUE_PIECES {
        C::KvTable::push_new(C::FUL[i].to_string(), C::ALG[i].to_string(), &mut table); 
    }
    table 
});

pub static PIECE_SYM_BL_LOOKUP_TABLE: Lazy<C::KvTable> = Lazy::new(|| {
    let mut table = C::KvTable::create_kv(); 
    for i in 0..C::UNIQUE_PIECES {
        // Note: ALG[i].to_string() allocates a String on the heap, 
        // which requires a lazy/runtime static initializer like this.
        C::KvTable::push_new(C::ALG[i].to_string(), C::SYM_BL[i].to_string(), &mut table); 
    }
    table // Return the initialized table
});

pub static PIECE_SYM_WH_LOOKUP_TABLE: Lazy<C::KvTable> = Lazy::new(|| {
    let mut table = C::KvTable::create_kv(); 
    for i in 0..C::UNIQUE_PIECES {
        // Note: ALG[i].to_string() allocates a String on the heap, 
        // which requires a lazy/runtime static initializer like this.
        C::KvTable::push_new(C::ALG[i].to_string(), C::SYM_WH[i].to_string(), &mut table); 
    }
    table // Return the initialized table
});
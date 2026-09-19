pub const XWIDTH:usize = 8;
pub const YWIDTH:usize = 8;

// bool:  0 = white
pub struct Piece {
    pub name_long: String, 
    pub name_short: String, 
    pub symbol : char, 
    pub color: bool, 
    pub locx : u8, 
    pub locy: u8
}

pub const UNIQUE_PIECES:usize = 5; 
pub static ALG : [&'static str; 5] = ["P", "N", "B", "Q", "K"]; 
pub static SYM_BL: [&str; 5] = ["♟", "♞", "♝", "♜", "♛"]; 
pub static SYM_WH: [&str; 5] = ["♙", "♘", "♗", "♖", "♕"];

pub static FUL : [&'static str; 5] = ["pawn", "knight", "bishop", "queen", "king"]; 

use std::hash::{BuildHasher, Hash, Hasher, RandomState};

struct Kv {
    pub key : u64, 
    pub value : String 
}

pub struct KvTable <B: BuildHasher = RandomState> {
    hash_builder : B,
    pub tab : Vec<Kv>
}

fn create_kv () -> KvTable<RandomState>{
    KvTable {
        hash_builder: RandomState::default(), 
        tab : Vec::new()
    }
}

fn push_new <B: BuildHasher> (key:String, val:String, k: &mut KvTable<B>) {
    let mut hasher = k.hash_builder.build_hasher();
    key.hash(&mut hasher); 

    let newkey = hasher.finish();
    let newent = Kv {key:newkey, value: val} ;
    k.tab.push(newent);

}

fn find_by_key<'a, B: BuildHasher>(key: String, table: &'a KvTable<B>) -> Option<&'a String> {
    let mut hasher = table.hash_builder.build_hasher();
    key.hash(&mut hasher);
    let hashed_key = hasher.finish();

    // Search the linear vector for the matching numeric hash
    for item in &table.tab {
        if item.key == hashed_key {
            return Some(&item.value);
        }
    }
    None
}

use once_cell::sync::Lazy; // Alternative: use std::sync::OnceLock;

// 1. Define your static lookup table globally
pub static PIECE_LONG_LOOKUP_TABLE: Lazy<KvTable> = Lazy::new(|| {
    let mut table = create_kv(); 
    for i in 0..UNIQUE_PIECES {
        // Note: ALG[i].to_string() allocates a String on the heap, 
        // which requires a lazy/runtime static initializer like this.
        push_new(ALG[i].to_string(), FUL[i].to_string(), &mut table); 
    }
    table // Return the initialized table
});

pub static PIECE_short_LOOKUP_TABLE: Lazy<KvTable> = Lazy::new(|| {
    let mut table = create_kv(); 
    for i in 0..UNIQUE_PIECES {
        push_new(FUL[i].to_string(), ALG[i].to_string(), &mut table); 
    }
    table 
});

pub static PIECE_SYM_BL_LOOKUP_TABLE: Lazy<KvTable> = Lazy::new(|| {
    let mut table = create_kv(); 
    for i in 0..UNIQUE_PIECES {
        // Note: ALG[i].to_string() allocates a String on the heap, 
        // which requires a lazy/runtime static initializer like this.
        push_new(ALG[i].to_string(), SYM_BL[i].to_string(), &mut table); 
    }
    table // Return the initialized table
});

pub static PIECE_SYM_WH_LOOKUP_TABLE: Lazy<KvTable> = Lazy::new(|| {
    let mut table = create_kv(); 
    for i in 0..UNIQUE_PIECES {
        // Note: ALG[i].to_string() allocates a String on the heap, 
        // which requires a lazy/runtime static initializer like this.
        push_new(ALG[i].to_string(), SYM_WH[i].to_string(), &mut table); 
    }
    table // Return the initialized table
});
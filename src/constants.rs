pub const XWIDTH:usize = 8;
pub const YWIDTH:usize = 8;

// bool:  0 = white

pub const UNIQUE_PIECES:usize = 5; 
pub static ALG : [&'static str; 5] = ["P", "N", "B", "Q", "K"]; 
pub static SYM_BL: [&str; 5] = ["♟", "♞", "♝", "♜", "♛"]; 
pub static SYM_WH: [&str; 5] = ["♙", "♘", "♗", "♖", "♕"];

pub static FUL : [&'static str; 5] = ["pawn", "knight", "bishop", "queen", "king"]; 

use std::hash::{BuildHasher, Hash, Hasher, RandomState};

pub struct Kv {
    pub key : u64, 
    pub value : String 
}

pub struct KvTable <B: BuildHasher = RandomState> {
    hash_builder : B,
    pub tab : Vec<Kv>
}

impl KvTable {
    pub fn create_kv () -> KvTable<RandomState>{
        KvTable {
            hash_builder: RandomState::default(), 
            tab : Vec::new()
        }
    }

    pub fn push_new <B: BuildHasher> (key:String, val:String, k: &mut KvTable<B>) {
        let mut hasher = k.hash_builder.build_hasher();
        key.hash(&mut hasher); 

        let newkey = hasher.finish();
        let newent = Kv {key:newkey, value: val} ;
        k.tab.push(newent);

    }

    pub fn find_by_key<'a, B: BuildHasher>(key: &String, table: &'a KvTable<B>) -> Option<&'a String> {
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
}








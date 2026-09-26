pub const XWIDTH:usize = 8;
pub const YWIDTH:usize = 8;

// bool: false = white, true = black

pub const UNIQUE_PIECES:usize = 6;
pub static ALG : [&'static str; 6] = ["P", "N", "B", "R", "Q", "K"];
pub static SYM_BL: [&str; 6] = ["♟", "♞", "♝", "♜", "♛", "♚"];
pub static SYM_WH: [&str; 6] = ["♙", "♘", "♗", "♖", "♕", "♔"];

pub static FUL : [&'static str; 6] = ["pawn", "knight", "bishop", "rook", "queen", "king"];

pub struct Kv {
    pub key : String,
    pub value : String
}

pub struct KvTable {
    pub tab : Vec<Kv>
}

impl KvTable {
    pub fn create_kv () -> KvTable{
        KvTable {
            tab : Vec::new()
        }
    }

    pub fn push_new (key:String, val:String, k: &mut KvTable) {
        let newent = Kv {key, value: val} ;
        k.tab.push(newent);

    }

    pub fn find_by_key<'a>(key: &String, table: &'a KvTable) -> Option<&'a String> {
        // Compare string keys directly: the previous implementation compared only
        // truncated u64 hashes produced with a per-table RandomState, which is
        // fragile (random seeds, hash collisions). Direct comparison is exact.
        for item in &table.tab {
            if item.key == *key {
                return Some(&item.value);
            }
        }
        None
    }
}








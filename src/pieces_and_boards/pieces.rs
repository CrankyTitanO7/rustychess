use crate::constants as C;

fn piecebegin() {
    kv_table = create_kv();

    kv_table;
}

// a function that instantiates a new piece at x y 
fn inst (acr:&str, x:usize, y: usize, color:bool) {
    let table = if pawn.color {
        PIECE_SYM_BL_LOOKUP_TABLE
    } else {
        PIECE_SYM_WH_LOOKUP_TABLE
    };

    let tmppiece = "P";

    let tmp=Piece {
        name_long: find_by_key(tmppiece, PIECE_LONG_LOOKUP_TABLE),
        name_short: tmppiece,
        symbol: find_by_key(tmppiece, table),
        color: false, // 0 = white per comment above
        locx: x,
        locy: y,
    };
    
    if (acr.to_ != "P") {
        pawn_to_x(tmp, acr);
    }

    tmp
}

// a function that upgrades pawns into other objects
fn pawn_to_x(pawn: &mut Piece, acr: &str) {
    pawn.name_long = find_by_key(acr, PIECE_LONG_LOOKUP_TABLE);
    pawn.name_short = acr.to_string();
    
    // Select table based on color, then look up the symbol
    let table = if pawn.color {
        PIECE_SYM_BL_LOOKUP_TABLE
    } else {
        PIECE_SYM_WH_LOOKUP_TABLE
    };
    
    pawn.symbol = find_by_key(acr, table);
}

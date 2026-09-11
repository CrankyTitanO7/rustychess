
// displays an 8x8 array of strings
fn displayBoard (boardArray: &[[&str; XWIDTH]; YWIDTH]) {
    for i < XWIDTH {
        for j < YWIDTH {
            print!(boardArray[i][j]);
        }
        print!("\n");
    }
}


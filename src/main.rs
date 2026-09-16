// main function 
// orchestrates the main conduction of the things.
mod display;
mod constants;
fn main() {
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn crate_constants_are_8x8() {
        assert_eq!(crate::constants::XWIDTH, 8);
        assert_eq!(crate::constants::YWIDTH, 8);
    }

    #[test]
    fn placeholder_engine_smoke() {
        // main() currently does nothing; just proves the test harness works
        assert!(true);
    }
}

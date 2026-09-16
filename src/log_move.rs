use crate::constants as C;

struct moveset {
    alg : String, 
}

pub struct log {
    log : Vec<moveset>, 
    num_moves : usize
}

pub fn new_log (&log: moveset) {
    let m : log;
    m.loc = Vec::new();
    m.num_moves = 0; 
    m
}

pub fn add_move_to_log (&log: log, &alg: String) {
    log.log.push(alg);
    log.num_moves += 1
}

pub fn print_log (&log: log) {
    for (index, movement) in log.log.iter.enumerate() {
        println!("move {i}: {}", index, movement);
    }
}

// unit tests 

#[cfg(test)] 

mod tests {
    use super::*; 

    #[test]
    fn test_new() {
        assert_eq!()
    }
}
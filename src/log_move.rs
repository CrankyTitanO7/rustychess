use crate::constants as C;

// struct moveset {
//     alg : String, 
// }

pub struct Log {
    // log : Vec<moveset>, 
    pub log: C::KvTable,
    pub num_moves : usize,
}

impl Log {
    pub fn new_log () -> Self {
        Log {
            log : C::KvTable::create_kv(),
            num_moves : 0,
        }
    }

    pub fn add_move_to_log (&mut self, alg: &str) {
        C::KvTable::push_new(self.num_moves.to_string(), alg.to_string(), &mut self.log);
        self.num_moves += 1
    }

    pub fn movesearch (&self, m: usize) -> Option<&String>{
        C::KvTable::find_by_key(&(m.to_string()), &self.log)
    }

    pub fn print_log (&self) {
        for (index, movement) in self.log.tab.iter().enumerate() {
            println!("move {}: {}", index, movement.value);
        }
    }
}
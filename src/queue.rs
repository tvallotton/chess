
use crate::moves::{Move, self}; 

struct Queue {
    queue: [Option<Move>; 6], 
    start: usize, 
}



impl Queue {
    fn finish(&self) -> bool {
        false
    }
}



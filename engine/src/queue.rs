
use crate::moves::{Move}; 

struct Queue {
    queue: [Option<Move>; 6], 
    start: usize, 
}



impl Queue {
    fn finish(&self) -> bool {
        false
    }
}



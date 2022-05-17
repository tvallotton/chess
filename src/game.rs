use crate::board::Board;
use crate::minimax::Node;
use crate::moves::Move;
use crate::opt::{Opt, Settings};
use crate::parameters::Params;
use crate::piece::Color;

use serde_json::{from_str, from_value};
use std::borrow::Borrow;
use std::cell::Ref;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Display;
use std::fs::read_to_string;
use std::time::Instant;
use structopt::StructOpt;
use tap::Pipe;
#[derive(Clone)]
pub struct Game {
    turn: Color,
    node: Node,
    opt: Settings,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.node.board)?;
        writeln!(f, "white heuristic:    {}", self.white_heuristic())?;
        writeln!(f, "black heuristic:    {}", self.white_heuristic())?;
        writeln!(f, "absolute heuristic: {}", self.absolute_heuristic())
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            turn: Color::White,
            node: Node::default(),
            opt: Opt::from_args()
                .settings
                .map(from_value)
                .unwrap_or_else(|| {
                    
                    let out = include_str!("../settings.json")
                        // .unwrap()
                        .pipe(|x| from_str(&x)); 
                        println!("asd"); 
                        return out; 
                })
                .unwrap(),
        }
    }

    fn params(&self) -> &Params {
        match self.turn {
            Color::White => &self.opt.white_params,
            Color::Black => &self.opt.black_params,
        }
    }
    fn board(&self) -> Board {
        self.node.board
    }
    pub fn play(&mut self) -> bool {
        log::info!("playing for: {:?}", self.node.turn);
        if let Some((mov, val)) = self.clone().a_star() {
            log::info!("move heuristic: {val}");
            self.node.board = self.node.board.apply(mov);
            
            self.turn = self.turn.opposite();
            self.node.turn = self.turn;
            return true;
        }
        false
    }

    fn a_star(&self) -> Option<(Move, f32)> {
        let mut count = 0;
        let time = Instant::now();

        let mut heap = BinaryHeap::with_capacity(self.opt.memory_limit);
        heap.push(&self.node);

        while let Some(mut node) = heap.pop() {
            log::debug!("POP:\n{}\nheuristic: {}", node.board, node.heuristic);
            node.expand(self.params());

            // # SAFETY
            // ## Aliasing
            //     The node is not mutated after this,
            //     since it is never returned to the heap
            //     any mutation on its children occurs behind
            //     a cell, ensuring aliasing safety.
            //
            // ## Lifetimes
            //     the lifetime of node is determined by self.
            //     given that self lives through the entire function call
            //     this should be safe.
            let children = unsafe { &*node.children.as_ptr() };

            for child in children.iter() {
                heap.push(child);
            }

            // If memory limit is reached
            // drop the worse half of nodes.
            if heap.len() > self.opt.memory_limit / 8 * 7 {
                log::error!("DROPPING: {}", heap.len());
                let mut sum = 0.0;
                let mut count = 0.0;
                heap.retain(|node| {
                    sum += node.heuristic;
                    count += 1.0;
                    node.heuristic >= sum / count
                });
                log::error!("DROPPED: {}", heap.len());
            }

            if count == self.opt.max_iter
                || (count % 500 == 0 && time.elapsed().as_millis() >= self.opt.time_limit)
            {
                log::error!("{count}");
                return self.minimax();
            }
            count += 1;
        }
        None
    }

    fn minimax(&self) -> Option<(Move, f32)> {
        self.node.get_move(self.turn)
    }
    pub fn winner(&self) {
        let h = self.absolute_heuristic();
        if h == 0.0 {
            println!("tie")
        } else if h > 0.0 {
            println!("white")
        } else {
            println!("black")
        }
    }
    fn white_heuristic(&self) -> f32 {
        self.node
            .board
            .heuristic(Color::White, self.params())
    }
    fn black_heuristic(&self) -> f32 {
        self.node
            .board
            .heuristic(Color::Black, self.params())
    }
    fn absolute_heuristic(&self) -> f32 {
        self.node
            .board
            .heuristic(Color::White, &self.opt.absolute_params)
    }
}

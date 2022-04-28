use serde_json::from_str;
use structopt::StructOpt;
use tap::Pipe;

use crate::board::Board;
use crate::minimax::MiniMaxNode;
use crate::moves::Move;
use crate::opt::Opt;
use crate::parameters::Params;
use crate::piece::Color;
use std::collections::BinaryHeap;
use std::fmt::Display;
use std::fs::read_to_string;

struct Game {
    turn: Color,
    node: MiniMaxNode,
    opt: Opt,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.node.board)?;
        writeln!(f, "node heuristic: {}", self.node_heuristic())?;
        writeln!(f, "board heuristic: {}", self.board_heuristic())
    }
}

impl Game {
    fn new() -> Self {
        Game {
            turn: Color::White,
            node: MiniMaxNode::default(),
            opt: Opt::from_args_safe()
                .or_else(|_| {
                    read_to_string("settings.json")
                        .as_deref()
                        .map(from_str)
                        .unwrap()
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
    fn play(&mut self) -> bool {
        log::info!("playing for: {:?}", self.node.turn);
        if let Some(mov) = self.minimax() {
            self.node.board.apply(mov);
            self.turn = self.turn.opposite();
            self.node.turn = self.turn;
            return true;
        }
        false
    }

    fn minimax(&self) -> Option<Move> {
        let time = std::time::Instant::now();
        let mut heap = BinaryHeap::with_capacity(self.opt.memory_limit);
        heap.push(self.node.clone());
        let mut count = 0;
        while let Some(mut node) = heap.pop() {
            log::debug!("POP:\n{}\nheuristic: {}", node.board, node.heuristic);
            for child in node.children(self.params()) {
                heap.push(child);
            }

            // drop unused nodes
            if heap.len() > self.opt.memory_limit / 8 * 7 {
                log::info!("DROPPING: {}", heap.len());
                let mut sum = 0.0;
                let mut count = 0.0;
                heap.retain(|node| {
                    sum += node.heuristic;
                    count += 1.0;
                    node.heuristic >= sum / count
                });
                log::info!("DROPPED: {}", heap.len());
            }
            count += 1;
            if count % 100 == 0 || time.elapsed().as_millis() >= self.opt.time_limit {
                return node.history;
            }
        }
        None
    }

    fn board_heuristic(&self) -> f32 {
        todo!()
    }
    fn node_heuristic(&self) -> f32 {
        todo!()
    }
}

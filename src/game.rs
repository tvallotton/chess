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
        writeln!(f, "black heuristic:    {}", self.black_heuristic())?;
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
                    read_to_string("settings.json")
                        .unwrap()
                        .pipe(|x| from_str(&x))
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

    fn minimax(&mut self) -> Option<(Move, f32)> {
       self.node.minimax(); 
       None
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
            .heuristic(Color::White, &self.opt.white_params)
    }
    fn black_heuristic(&self) -> f32 {
        self.node
            .board
            .heuristic(Color::Black, &self.opt.black_params)
    }
    fn absolute_heuristic(&self) -> f32 {
        self.node
            .board
            .heuristic(Color::White, &self.opt.absolute_params)
    }
}

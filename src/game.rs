use itertools::Itertools;
use serde_json::{from_str, from_value};
use tap::Pipe;

use crate::board::Board;
use crate::minimax::MiniMaxNode;
use crate::moves::Move;
use crate::opt::{Opt, Settings};
use crate::parameters::Params;
use crate::piece::Color;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Display;
use std::fs::read_to_string;
use structopt::StructOpt;

pub struct Game {
    turn: Color,
    node: MiniMaxNode,
    opt: Settings,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.node.board)?;
        writeln!(f, "node heuristic: {}", self.node_heuristic())?;
        writeln!(f, "board heuristic: {}", self.board_heuristic())
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            turn: Color::White,
            node: MiniMaxNode::default(),
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
        if let Some(mov) = self.minimax() {
            self.node.board = self.node.board.apply(mov);
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
            if count % 100 == 0 && time.elapsed().as_millis() >= self.opt.time_limit {
                return get_move(heap, self.turn)
            }
        }
        None
    }
    pub fn winner(&self) {
        let h = self.node_heuristic();
        if h == 0.0 {
            println!("tie")
        } else if h > 0.0 {
            println!("white")
        } else {
            println!("black")
        }
    }
    fn board_heuristic(&self) -> f32 {
        self.node
            .board
            .heuristic(Color::White, self.params())
    }
    fn node_heuristic(&self) -> f32 {
        self.node
            .board
            .heuristic(Color::White, &self.opt.absolute_params)
    }
}

enum MoveTree {
    Branch(HashMap<Move, MoveTree>),
    Leaf(f32),
}

impl MoveTree {
    fn add_node(mut self: &mut Self, node: MiniMaxNode) {
        for mov in node.history {
            match self {
                Leaf(_) => {
                    self = self.into_branch(mov, node.heuristic);
                }
                Branch(map) => {
                    self = map
                        .entry(mov)
                        .or_insert(Leaf(node.heuristic));
                }
            }
        }
    }

    fn into_branch(&mut self, mov: Move, heuristic: f32) -> &mut Self {
        let map = HashMap::from([(mov, Leaf(heuristic))]);
        *self = Branch(map);
        match self {
            Branch(map) => map.get_mut(&mov).unwrap(),
            _ => unreachable!(),
        }
    }

    fn minimax(self, turn: Color) -> f32 {
        match self {
            Branch(children) => children
                .into_values()
                .map(|child| child.minimax(turn.opposite()))
                .pipe(|value| {
                    if turn == Color::White {
                        value.max_by(|x, y| x.partial_cmp(y).unwrap())
                    } else {
                        value.min_by(|x, y| x.partial_cmp(y).unwrap())
                    }
                })
                .unwrap(),
            Leaf(value) => value,
        }
    }
    fn branches(self) -> HashMap<Move, MoveTree> {
        match self {
            Branch(map) => map,
            _ => unreachable!(),
        }
    }
    fn get_move(self, turn: Color) -> Option<Move> {
        self.branches()
            .into_iter()
            .map(|(mov, branch)| (mov, branch.minimax(turn)))
            .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
            .map(|x| x.0)
    }
}

use MoveTree::*;
fn get_move(heap: BinaryHeap<MiniMaxNode>, turn: Color) -> Option<Move> {
    let mut tree = Leaf(0.0);
    for node in heap.into_iter() {
        tree.add_node(node);
    }
    tree.get_move(turn)
}

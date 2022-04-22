use itertools::MinMaxResult;

use crate::moves::{Move, Play};
use crate::settings::{self, *};
use crate::settings::{ATTACKED, DEFENDED};
use crate::{piece::Color, table::Board};

use std::cell::RefCell;
use std::collections::{binary_heap, BinaryHeap};

#[derive(Clone)]
pub struct MiniMaxNode {
    history: Vec<Move>,
    turn: Color,
    heuristic: f32,
   pub board: Board,
    children: Vec<MiniMaxNode>,
}

impl Ord for MiniMaxNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // SAFETY
        // None is not a possible value
        unsafe {
            self.partial_cmp(other)
                .unwrap_unchecked()
        }
    }
}

// if me and me {
//     pick my highest chance
// }
// if me and opponent {
//     pick my hightest change
// }
// if opponent and opponent {
//     pick my lowest change
// }

impl PartialOrd for MiniMaxNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.heuristic
            .partial_cmp(&other.heuristic)
    }
}

impl PartialEq for MiniMaxNode {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic == other.heuristic
    }
}
impl Eq for MiniMaxNode {}
impl Default for MiniMaxNode {
    fn default() -> Self {
        MiniMaxNode {
            history: vec![],
            turn: Color::White,
            heuristic: 0.0,
            board: Board::default(),
            children: Default::default(),
        }
    }
}

impl MiniMaxNode {
    fn reduce(&self) -> Move {
        todo!()
    }

    fn new(board: Board, turn: Color, base_heuristic: f32, history: &[Move], move_: Move) -> Self {
        let player_score: f32 = board
            .colored_pieces(turn)
            .map(value)
            .sum();
        let opponent_score: f32 = board
            .colored_pieces(turn.opposite())
            .map(value)
            .sum();
        let _rough_estimate = player_score - opponent_score;
        let mut history = Vec::with_capacity(history.len() + 1);
        history.push(move_);
        MiniMaxNode {
            history,
            turn,
            heuristic: base_heuristic + player_score - opponent_score,
            board,
            children: Default::default(),
        }
    }
    fn add_children(&mut self) {
        let plays = self.board.moves(self.turn);
        let mut defended_value = 0.0f32;
        let mut attacked_value = 0.0f32;
        let mut children = vec![];
        let mut count = 0;
        for play in plays {
            count += 1;
            let r#move;
            match play {
                Play::Defense(move_, piece) => {
                    defended_value += value((piece, move_.to));
                    continue;
                }
                Play::Capture(move_, piece) => {
                    attacked_value += value((piece, move_.from));
                    r#move = move_;
                }
                Play::Move(move_) => {
                    r#move = move_;
                }
                _ => panic!("castle not implemented"),
            }
            let mut board = self.board;
            board.apply(r#move);

            children.push(Self::new(
                board,
                self.turn.opposite(),
                heuristic(defended_value, attacked_value, count),
                &self.history,
                r#move,
            ));
        }
        self.children = children;
    }

    pub fn build(self) -> Move {
        let mut heap = BinaryHeap::from([self]);
        let mut max = 0;
        while let Some(mut node) = heap.pop() {
            node.add_children();
            for child in node.children {
                heap.push(child);
            }
            // drop unused nodes
            if heap.len() > *MAX_NODES {
                let mut sum = 0.0;
                let mut count = 0.0;
                heap.retain(|node| {
                    sum += node.heuristic;
                    count += 1.0;
                    node.heuristic >= sum / count
                });
            }
            max += 1;
            if max == 100 {
                return node.history[0];
            }
        }
        panic!("no moves available");
    }
}

fn heuristic(defended_value: f32, attacked_value: f32, count: i32) -> f32 {
    *DEFENDED * defended_value + *ATTACKED * attacked_value + *AVAILABLE_MOVES * count as f32
}

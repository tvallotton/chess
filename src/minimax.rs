use crate::moves::{Move, Play};
use crate::parameters::*;
use crate::{board::Board, piece::Color};
use arrayvec::ArrayVec;
use itertools::Itertools;
use std::cell::{Ref, RefCell};
use std::cmp::PartialOrd;
use std::cmp::{Ord, PartialEq};

use structopt::clap::Shell;
use Color::*;

#[derive(Clone)]
pub struct Node {
    pub heuristic: f32,
    pub history: Option<Move>,
    pub turn: Color,
    pub board: Board,
    pub children: RefCell<Vec<Node>>,
}

impl Node {
    pub fn expand(&self, params: &Params) {
        let mut children = self.children.borrow_mut();
        if children.is_empty() {
            *children = self.children(params);
        }
    }

    pub fn children(&self, params: &Params) -> Vec<Self> {
        let Self { turn, board, .. } = self;
        board
            .playable_moves(self.turn)
            .map(|mov| {
                let new_board = board.apply(mov);
                let child_node = Self {
                    history: self.history.or(Some(mov)),
                    board: new_board,
                    children: RefCell::default(),
                    heuristic: new_board.heuristic(*turn, params),
                    turn: turn.opposite(),
                };

                log::debug!(
                    "CHILD:\n{}\nheuristic {}",
                    child_node.board,
                    child_node.heuristic
                );
                child_node
            })
            .collect()
    }

    pub fn get_move(&self, turn: Color) -> Option<(Move, f32)> {
        let (mut black, mut white) = (f32::INFINITY, f32::NEG_INFINITY);
        let children = self.children.borrow();
        let moves = children
            .iter()
            .map(|child| (child, child.minimax(turn, &mut black, &mut white)));

        if turn == Color::White {
            let (node, h) = moves.max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())?;

            Some((node.history?, h))
        } else {
            let (node, h) = moves.min_by(|x, y| x.1.partial_cmp(&y.1).unwrap())?;
            Some((node.history?, h))
        }
    }

    pub fn minimax(&self, turn: Color, black: &mut f32, white: &mut f32) -> f32 {
        let children = self.children.borrow();
        if children.is_empty() {
            return self.heuristic;
        }
        if let Color::White = turn {
            let mut max = f32::NEG_INFINITY;
            for child in &*children {
                let value = child.minimax(turn.opposite(), black, white);
                max = max.max(value);
                *white = white.max(self.heuristic);
                if black < white {
                    break;
                }
            }
            max
        } else {
            let mut min = f32::INFINITY;
            for child in &*children {
                let value = child.minimax(turn.opposite(), black, white);
                min = min.min(value);
                *black = black.min(self.heuristic);
                if white < black {
                    break;
                }
            }
            min
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .unwrap()
    }
}

// // if me and me {
// //     pick my highest chance
// // }
// // if me and opponent {
// //     pick my hightest chance
// // }
// // if opponent and opponent {
// //     pick my lowest chance
// // }
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // let h1 = self
        //     .turn
        //     .opposite()
        //     .pawn_dir() as f32
        //     * self.heuristic;
        // let h2 = other
        //     .turn
        //     .opposite()
        //     .pawn_dir() as f32
        //     * other.heuristic;
        // h1.partial_cmp(&h2)

        match (self.turn, other.turn) {
            (White, White) => self
                .heuristic
                .partial_cmp(&other.heuristic),
            (White, Black) => self
                .heuristic
                .partial_cmp(&-other.heuristic),
            (Black, White) => (-self.heuristic).partial_cmp(&other.heuristic),
            (Black, Black) => (-self.heuristic).partial_cmp(&-other.heuristic),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic == other.heuristic
    }
}
impl Eq for Node {}
impl Default for Node {
    fn default() -> Self {
        Self {
            history: None,
            turn: White,
            heuristic: 0.0,
            board: Board::default(),
            children: Default::default(),
        }
    }
}

// impl MiniMaxNode {
//     fn reduce(&self) -> Move {
//         todo!()
//     }

//     fn new(board: Board, turn: Color, history: Vec<Move>, params: &Params) -> Self {
//         MiniMaxNode {
//             history,
//             turn,
//             heuristic: board.heuristic(turn, params),
//             board,
//         }
//     }

//     // pub fn build(self) -> (Move, f32) {
//     //     let current_turn = self.turn;
//     //     let mut heap = BinaryHeap::with_capacity(*MAX_NODES);
//     //     heap.push(self);
//     //     let mut max = 0;
//     //     while let Some(mut node) = heap.pop() {
//     //         log::debug!("POP:\n{}\nheuristic: {}", node.board, node.heuristic);
//     //         for child in node.get_children() {
//     //             heap.push(child);
//     //         }
//     //         // drop unused nodes
//     //         if heap.len() > *MAX_NODES {
//     //             let mut sum = 0.0;
//     //             let mut count = 0.0;
//     //             heap.retain(|node| {
//     //                 sum += node.heuristic;
//     //                 count += 1.0;
//     //                 node.heuristic >= sum / count
//     //             });
//     //         }
//     //         max += 1;
//     //         if max >= *MAX_ITER && node.turn == current_turn {
//     //             return (node.history.unwrap(), node.heuristic);
//     //         }
//     //     }
//     //     panic!("no moves available");
//     // }

//     // pub fn play(&mut self) {
//     //     log::info!("playing for: {:?}", self.turn);
//     //     let node = self.clone();
//     //     let mov = node.build();
//     //     self.turn = self.turn.opposite();
//     //     self.board = self.board.apply(mov.0);
//     //     self.heuristic = mov.1;
//     // }
// }

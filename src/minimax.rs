use arrayvec::ArrayVec;

use crate::moves::{Move, Play};
use crate::parameters::*;
use crate::parameters::{ATTACKED, DEFENDED};
use crate::{board::Board, piece::Color};

struct Game {
    node: MiniMaxNode,
    opt: crate::opt::Opt,
}

#[derive(Clone)]
pub struct MiniMaxNode {
    pub history: Option<Move>,
    pub turn: Color,
    pub heuristic: f32,
    pub board: Board,
}

impl Ord for MiniMaxNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // SAFETY
        // None is not a possible value

        self.partial_cmp(other)
            .unwrap()
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
            history: None,
            turn: Color::White,
            heuristic: 0.0,
            board: Board::default(),
            // children: Default::default(),
        }
    }
}

impl MiniMaxNode {
    fn reduce(&self) -> Move {
        todo!()
    }

    fn new(board: Board, turn: Color, history: Option<Move>, params: &Params) -> Self {
        MiniMaxNode {
            history,
            turn,
            heuristic: board.heuristic(turn, params),
            board,
        }
    }
    pub fn children(&mut self, params: &Params) -> ArrayVec<Self, 256> {
        let _plays = self.board.moves(self.turn);
        let mut children = arrayvec::ArrayVec::<_, 256>::new();
        self.board
            .moves(self.turn)
            .for_each(|play| {
                let r#move = match play {
                    Play::Defense(_, _) => {
                        return;
                    }
                    Play::Capture(move_, _) => {
                        assert!(self.board[move_.to].is_some());
                        move_
                    }
                    Play::Move(move_) => move_,
                    _ => panic!("castle not implemented"),
                };
                let child_node = Self::new(
                    self.board.apply(r#move),
                    self.turn.opposite(),
                    self.history.or(Some(r#move)),
                    params,
                );
                log::debug!(
                    "CHILD:\n{}\nheuristic {}",
                    child_node.board,
                    child_node.heuristic
                );
                children.push(child_node);
            });
        children
    }

    // pub fn build(self) -> (Move, f32) {
    //     let current_turn = self.turn;
    //     let mut heap = BinaryHeap::with_capacity(*MAX_NODES);
    //     heap.push(self);
    //     let mut max = 0;
    //     while let Some(mut node) = heap.pop() {
    //         log::debug!("POP:\n{}\nheuristic: {}", node.board, node.heuristic);
    //         for child in node.get_children() {
    //             heap.push(child);
    //         }
    //         // drop unused nodes
    //         if heap.len() > *MAX_NODES {
    //             let mut sum = 0.0;
    //             let mut count = 0.0;
    //             heap.retain(|node| {
    //                 sum += node.heuristic;
    //                 count += 1.0;
    //                 node.heuristic >= sum / count
    //             });
    //         }
    //         max += 1;
    //         if max >= *MAX_ITER && node.turn == current_turn {
    //             return (node.history.unwrap(), node.heuristic);
    //         }
    //     }
    //     panic!("no moves available");
    // }

    // pub fn play(&mut self) {
    //     log::info!("playing for: {:?}", self.turn);
    //     let node = self.clone();
    //     let mov = node.build();
    //     self.turn = self.turn.opposite();
    //     self.board = self.board.apply(mov.0);
    //     self.heuristic = mov.1;
    // }
}

fn heuristic(defended_value: f32, attacked_value: f32, count: i32) -> f32 {
    *DEFENDED * defended_value + *ATTACKED * attacked_value + *AVAILABLE_MOVES * count as f32
}

use crate::moves::{Move, Play};
use crate::settings::*;
use crate::settings::{ATTACKED, DEFENDED};
use crate::{piece::Color, table::Board};

use std::cell::RefCell;
use std::collections::BinaryHeap;

use Color::*;

struct MiniMaxNode {
    turn: Color,
    heuristic: f32,
    board: Board,
    children: RefCell<Vec<MiniMaxNode>>,
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

    fn new(board: Board, turn: Color, base_heuristic: f32) -> Self {
        let player_score: f32 = board
            .colored_pieces(turn)
            .map(value)
            .sum();
        let opponent_score: f32 = board
            .colored_pieces(turn.opposite())
            .map(value)
            .sum();
        let rough_estimate = player_score - opponent_score;
        MiniMaxNode {
            turn,
            heuristic: base_heuristic + player_score - opponent_score,
            board,
            children: Default::default(),
        }
    }
    fn add_children(&self) {
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

                    r#move = move_;
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
            ));
        }
        let mut cell = self.children.borrow_mut();
        *cell = children;
    }
}

fn heuristic(defended_value: f32, attacked_value: f32, count: i32) -> f32 {
    return *DEFENDED * defended_value
        + *ATTACKED * attacked_value
        + *AVAILABLE_MOVES * count as f32;
}

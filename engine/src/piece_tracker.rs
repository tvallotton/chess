use std::convert::Infallible;

use crate::moves::Move;
use crate::parameters::Params;
use crate::piece::{Kind, Piece};
use crate::Color::*;
use crate::{board::Board, moves::Position, piece::Color};
use arrayvec::ArrayVec;
use itertools::Itertools;
use tap::Pipe;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PieceTracker {
    pub black: ArrayVec<Position, 16>,
    pub white: ArrayVec<Position, 16>,
}

impl PieceTracker {
    fn pieces_mut(&mut self, color: Color) -> &mut ArrayVec<Position, 16> {
        match color {
            Black => &mut self.black,
            White => &mut self.white,
        }
    }
    fn pieces(&self, color: Color) -> &ArrayVec<Position, 16> {
        match color {
            Black => &self.black,
            White => &self.white,
        }
    }

    fn remove(&mut self, color: Color, target: Position) -> Option<Infallible> {
        let (index, _) = self
            .pieces(color)
            .iter()
            .find_position(|pos| target == **pos)?;
        self.pieces_mut(color)
            .swap_remove(index);
        None
    }
    fn mov(&mut self, color: Color, mov: Move) -> Option<Infallible> {
        let tracker = self.pieces_mut(color);
        let index = tracker
            .iter()
            .find_position(|&from| *from == mov.from)?
            .0;
        tracker[index] = mov.to;
        None
    }
}

impl Board {
    /// Initializes the piece tracker and sorts
    /// the tracked indexes by value.
    pub(crate) fn init_piece_tracker(&mut self) {
        for i in 0..8 {
            for j in 0..8 {
                let position = (i, j).into();
                if let Some(piece) = self[(i, j)] {
                    self.tracker_mut(piece.color)
                        .push(position)
                }
            }
        }
        let board = self.clone();
        let key = |position: &Position| board[*position].unwrap().kind;

        self.pieces
            .black
            .sort_unstable_by_key(key);
        self.pieces
            .white
            .sort_unstable_by_key(key);
    }
    /// Retrieves a mutable reference to the piece tracker
    #[inline]
    fn tracker_mut(&mut self, color: Color) -> &mut ArrayVec<Position, 16> {
        self.pieces.pieces_mut(color)
    }
    /// Retrieves an immutable reference to the peice tracker
    #[inline]
    fn tracker(&self, color: Color) -> &ArrayVec<Position, 16> {
        self.pieces.pieces(color)
    }

    pub fn _move_noncapture(&mut self, mov: Move) -> Piece {
        let piece = self[mov.from]
            .take()
            .unwrap_or_else(|| panic!("invalid move {mov:?}\n{self}"));
        // Move in board
        self[mov.to] = Some(piece);

        // Move in tracker
        self.pieces
            .mov(piece.color, mov);
        piece
    }
    /// Performs a move. If there is no piece to move this function
    /// will panic.
    #[inline]
    pub fn _move_capture(&mut self, mov: Move) -> (Piece, Option<Piece>) {
        let capture = self[mov.to];
        // Remove capture
        for piece in capture {
            self.pieces
                .remove(piece.color, mov.to);
        }

        let piece = self._move_noncapture(mov);

        (piece, capture)
    }
    #[inline]
    pub fn remove_piece(&mut self, pos: Position) -> Option<Piece> {
        let piece = self[pos]?;
        self.pieces
            .remove(piece.color, pos);
        Some(piece)
    }

    pub fn colored_pieces(&self, color: Color) -> impl Iterator<Item = (Piece, Position)> + '_ {
        let map = move |&i| (self[i].unwrap_or_else(move || panic!("{self},{i:?}")), i);
        match color {
            Black => self
                .pieces
                .black
                .iter()
                .map(map),
            White => self
                .pieces
                .white
                .iter()
                .map(map),
        }
    }

    fn _check_pieces(&mut self) -> Result<(), (Color, Position)> {
        for i in 0..8 {
            for j in 0..8 {
                let p = (i, j).into();
                for piece in self[p] {
                    let tracker = self.tracker(piece.color);
                    if !tracker.contains(&p) {
                        return Err((piece.color, p));
                    }
                }
            }
        }
        for color in [White, Black] {
            for pos in self.tracker(color) {
                self[*pos].ok_or((color, *pos))?;
            }
        }
        Ok(())
    }
}

#[test]
fn test_tracker() {
    pretty_env_logger::init();
    let mut params = Params::default();
    params.max_depth = 1;
    let mut board = Board::default();
    let mut parent = board.clone();
    let mut i = 0;
    while board
        .play_with(&params)
        .is_some()
    {
        if let Err((color, pos)) = board._check_pieces() {
            log::error!(
                "\n{pos:?}\n{parent}\n{:#?}\n{board}\n{:#?}",
                parent.tracker(color),
                board.tracker(color)
            );
            panic!("piece tracker out if sync")
        }
        parent = board.clone();
        if i == 500 {
            break;
        }
        i += 1;
    }
}

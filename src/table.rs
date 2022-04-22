use std::panic::UnwindSafe;

use crate::{
    moves::{Move, Play, Position},
    piece::{Color, Kind, Piece},
};
use tap::prelude::*;
use Kind::*;

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub table: [[Option<Piece>; 8]; 8],
    pub black_castle: Castle,
    pub white_castle: Castle,
}
#[derive(Debug, Clone, Copy)]
pub struct Castle {
    pub kingside: bool,
    pub queenside: bool,
}

fn cast<'a, T: Iterator<Item = Position> + 'a>(obj: T) -> Box<dyn Iterator<Item = Position> + 'a> {
    Box::new(obj) as Box<dyn Iterator<Item = Position>>
}
type Positions<'a> = Box<dyn Iterator<Item = Position> + 'a>;
impl UnwindSafe for Board {}
impl Board {
    pub fn apply(&mut self, move_: Move) {
        let piece = self[move_.from].take();
        self[move_.to] = piece;
    }

    // pub fn apply(&mut self, play: Play) -> Option<f32> {
    //     match play {
    //         Play::Capture(move_, piece) => {
    //             let taken_value = value((piece, move_.to));
    //             Some(taken_value)
    //         }
    //         Play::Move(move_) => {
    //             let piece = self[move_.from].take();
    //             self[move_.to] = piece;
    //             Some(0.0)
    //         }
    //         Play::Defense(_, _) => return None,
    //         _ => panic!(),
    //     }
    // }
    pub fn get(&self, pos: Position) -> Option<Option<Piece>> {
        self
            .table
            .get(pos.rank as usize)
            .map(|row| {
                row.get(pos.file as usize)
                    .copied()
            })?
    }

    pub fn colored_pieces<'a>(
        &'a self,
        color: Color,
    ) -> impl Iterator<Item = (Piece, Position)> + 'a {
        (0..8)
            .flat_map(|rank| (0..8).map(move |file| (rank, file)))
            .map(Position::from)
            .filter_map(move |pos| (self[pos]?, pos).pipe(Some))
            .filter(move |(piece, _)| piece.color == color)
    }
    pub fn moves<'a>(&'a self, turn: Color) -> impl Iterator<Item = Play> + 'a {
        self.colored_pieces(turn)
            .map(|(_, pos)| pos)
            .flat_map(|pos| self.moves_for(pos))
    }

    #[allow(unreachable_patterns)]
    pub fn moves_for<'a>(&'a self, pos: Position) -> impl Iterator<Item = Play> + 'a {
        use itertools::Either::*;
        self[pos]
            .into_iter()
            .flat_map(move |piece| match piece.kind {
                Bishop => self
                    .bishop_moves(pos, piece.color)
                    .pipe(Left)
                    .pipe(Left)
                    .pipe(Left),
                King => self
                    .king_moves(pos, piece.color)
                    .pipe(Left)
                    .pipe(Left)
                    .pipe(Right),
                Queen => self
                    .queen_moves(pos, piece.color)
                    .pipe(Left)
                    .pipe(Right)
                    .pipe(Left),
                Knight => self
                    .king_moves(pos, piece.color)
                    .pipe(Left)
                    .pipe(Right)
                    .pipe(Right),
                Rook => self
                    .rook_moves(pos, piece.color)
                    .pipe(Right)
                    .pipe(Left)
                    .pipe(Left),
                Pawn => self
                    .pawn_moves(pos, piece.color)
                    .pipe(Right)
                    .pipe(Left)
                    .pipe(Right),
                // This is unreachable but it is used as a type hint
                _ => None
                    .into_iter()
                    .pipe(Right)
                    .pipe(Right)
                    .pipe(Left),
                _ => None
                    .into_iter()
                    .pipe(Right)
                    .pipe(Right)
                    .pipe(Right),
            })
    }
    /// # Bishop moves
    /// ready
    fn bishop_moves<'a>(&'a self, pos: Position, color: Color) -> impl Iterator<Item = Play> + 'a {
        self.walk(pos, color, -1, -1)
            .chain(self.walk(pos, color, -1, 1))
            .chain(self.walk(pos, color, 1, -1))
    }
    /// # Rook
    /// ready
    fn rook_moves<'a>(&'a self, pos: Position, color: Color) -> impl Iterator<Item = Play> + 'a {
        self.walk(pos, color, -1, 0)
            .chain(self.walk(pos, color, 0, -1))
            .chain(self.walk(pos, color, 0, 1))
            .chain(self.walk(pos, color, 1, 0))
    }
    /// # Queen
    /// ## ready
    /// * diagonal moves
    /// * vertical and horizontal moves

    fn queen_moves<'a>(&'a self, pos: Position, color: Color) -> impl Iterator<Item = Play> + 'a {
        self.bishop_moves(pos, color)
            .chain(self.rook_moves(pos, color))
    }
    /// # King
    /// ## ready
    /// * basic moves
    /// ## missing
    /// * castling
    fn king_moves<'a>(&'a self, pos: Position, color: Color) -> impl Iterator<Item = Play> + 'a {
        self.relative(pos, color, 0, 1)
            .into_iter()
            .chain(self.relative(pos, color, 0, -1))
            .chain(self.relative(pos, color, 1, 0))
            .chain(self.relative(pos, color, 1, 1))
            .chain(self.relative(pos, color, 1, -1))
            .chain(self.relative(pos, color, -1, 0))
            .chain(self.relative(pos, color, -1, 1))
            .chain(self.relative(pos, color, -1, -1))
    }
    /// # Pawns
    /// ## ready
    /// 1. double initial jump
    /// 2. sideways capture
    /// 3. upwards move
    ///
    /// ## missing
    /// 1. pawn passant
    /// 2. promotion
    fn pawn_moves<'a>(&'a self, pos: Position, color: Color) -> impl Iterator<Item = Play> {
        self.capture_only(pos, color, color.pawn_dir(), 1)
            .into_iter()
            .chain(self.capture_only(pos, color, color.pawn_dir(), -1))
            .chain(self.moves_only(pos, color, color.pawn_dir(), 0))
            .chain({
                if pos.rank == color.pawn_start() {
                    self.moves_only(pos, color, 2 * color.pawn_dir(), 0)
                } else {
                    None
                }
            })
    }

    fn is_capture(&self, pos: Position, color: Color) -> bool {
        self[pos]
            .map(|piece| piece.color != color)
            .unwrap_or(false)
    }

    /// This is used to describe the movements of pieces that can move in some relative
    /// direction but they cannot capture. This is used to describe the movements of pawns.
    fn moves_only(&self, from: Position, color: Color, rank: isize, file: isize) -> Option<Play> {
        let move_ = self.relative(from, color, rank, file);
        if let Some(Play::Move(_)) = move_ {
            return move_;
        }
        None
    }

    /// This function is used to specify a relative position to the one presented
    /// and returns a Play to that position only if it the piece can move there
    /// by capturing. This is used to describe how pawns move.
    ///
    /// from: represents the current position of the piece to be moved.
    /// rank: relative rank movement
    /// file: relative file movement
    fn capture_only(&self, from: Position, color: Color, rank: isize, file: isize) -> Option<Play> {
        let move_ = self.relative(from, color, rank, file);
        if let Some(Play::Move(_)) = move_ {
            return None;
        }
        move_
    }

    fn relative(&self, from: Position, color: Color, rank: isize, file: isize) -> Option<Play> {
        let to = Position {
            rank: from.rank + rank,
            file: from.file + file,
        };
        let r#move = Move { to, from };
        match self.get(to)? {
            Some(piece) if piece.color == color => Play::Defense(r#move, piece),
            Some(piece) => Play::Capture(r#move, piece),
            None => Play::Move(r#move),
        }
        .pipe(Some)
    }

    pub fn walk<'a>(
        &'a self,
        pos: Position,
        color: Color,
        rank: isize,
        file: isize,
    ) -> impl Iterator<Item = Play> + 'a {
        (1..8)
            .map(move |i| (pos.rank + rank * i, pos.file + file * i))
            .take_while(|pos| 0 <= pos.0 && pos.0 < 8)
            .take_while(|pos| 0 <= pos.1 && pos.1 < 8)
            .map(Position::from)
            .take_while(move |&iterpos| {
                let prev = Position {
                    rank: iterpos.rank - rank,
                    file: iterpos.file - file,
                };
                pos == iterpos || self[prev].is_some()
            })
            .map(move |to| {
                let r#move = Move { to, from: pos };
                match self[to] {
                    Some(piece) if color == piece.color => Play::Defense(r#move, piece),
                    Some(piece) => Play::Capture(r#move, piece),
                    None => Play::Move(r#move),
                }
            })
    }
}

impl std::ops::Index<Position> for Board {
    type Output = Option<Piece>;
    fn index(&self, index: Position) -> &Self::Output {
        &self.table[index.rank as usize][index.file as usize]
    }
}
impl std::ops::IndexMut<Position> for Board {
    fn index_mut(&mut self, index: Position) -> &mut Option<Piece> {
        &mut self.table[index.rank as usize][index.file as usize]
    }
}

#[test]
fn foo() {}

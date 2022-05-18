use crate::{
    moves::{Move, Play, Position},
    parameters::Params,
    piece::{Color, Kind, Piece},
};
use std::panic::UnwindSafe;
use Color::*;

use tap::prelude::*;
use unicode_segmentation::UnicodeSegmentation;
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
    pub fn empty() -> Board {
        Board {
            table: [[None; 8]; 8],
            black_castle: Castle {
                kingside: true,
                queenside: true,
            },
            white_castle: Castle {
                kingside: true,
                queenside: true,
            },
        }
    }

    pub fn apply(&self, mov: Move) -> Board {
        let mut board = *self;
        let piece = board[mov.from].take();

        board[mov.to] = piece;
        if [0, 7].contains(&mov.to.rank) {
            if let Piece { kind: Pawn, color } = board[mov.to].unwrap() {
                board[mov.to] = Some(Piece { kind: Queen, color });
            }
        }

        board
    }

    /// Positive numbers mean that white is winning. Negative numbers means that black is winning.
    #[inline]
    pub fn heuristic(&self, params: &Params, log: bool) -> f32 {
        let mut score = params.turn_value;

        for i in 0..8 {
            for j in 0..8 {
                let pos = (i, j).into();

                if let Some(piece) = self[pos] {
                    let mut piece_val = params.piece_val((piece, pos));

                    self.plays_for(pos)
                        .for_each(|play| {
                            // we want more available moves, but we don't want to use the queen too early
                            piece_val +=
                                params.available_moves / (1.0 + params.value((piece, pos)));

                            match play {
                                Play::Capture(mov, taken) => {
                                    piece_val += params.attacked(taken, piece, mov);
                                    piece_val += params.mov(piece, mov);
                                }
                                Play::Defense(mov, def) => {
                                    piece_val += params.defended((def, mov.to), (piece, mov.from));
                                    piece_val += params.mov(piece, mov);
                                }
                                Play::Move(mov) => {
                                    piece_val += params.mov(piece, mov);
                                }
                                _ => panic!(),
                            }
                        });
                    let Piece { kind, color } = piece;
                    if piece.color == Color::White {
                        if log {
                            log::debug!("{color:?} {kind:?}: {piece_val}");
                        }
                        score += piece_val;
                    } else {
                        if log {
                            log::debug!("{color:?} {kind:?}: {piece_val}");
                        }
                        score -= piece_val;
                    }
                }
            }
        }

        score
    }
    // pub fn parse(mut input: &str) {
    //     let empty = Board::empty();

    //     if let Some(stripped) = input.strip_prefix(" ║ ") {
    //         input = stripped;
    //         let piece = input.graphemes(true).next();
    //         empty[(0, 0)] = Some(match piece {
    //             Some("♜") => Black | Rook,
    //             Some("♟") => Black | Pawn,
    //             Some("♚") => Black | Rook,
    //             Some("♝") => Black | Bishop,
    //             Some("♞") => Black | Knight,
    //             Some("♙") => White | Pawn,
    //             Some("♔") => White | King,
    //             Some("♗") => White | Bishop,
    //             Some("♖") => White | Rook,
    //             Some("♘") => White | Knight, 
    //             _ => return,
    //         });
    //     }
    // }
    pub fn get(&self, pos: Position) -> Option<Option<Piece>> {
        self.table
            .get(pos.rank as usize)
            .map(|row| {
                row.get(pos.file as usize)
                    .copied()
            })?
    }

    pub fn colored_pieces(&self, color: Color) -> impl Iterator<Item = (Piece, Position)> + '_ {
        (0..8)
            .flat_map(|rank| (0..8).map(move |file| (rank, file)))
            .map(Position::from)
            .filter_map(move |pos| (self[pos]?, pos).pipe(Some))
            .filter(move |(piece, _)| piece.color == color)
    }
    pub fn moves(&self, turn: Color) -> impl Iterator<Item = Play> + '_ {
        self.colored_pieces(turn)
            .map(|(_, pos)| pos)
            .flat_map(|pos| self.plays_for(pos))
    }

    pub fn playable_moves(&self, turn: Color) -> impl Iterator<Item = Move> + '_ {
        self.moves(turn)
            .filter_map(|play| match play {
                Play::Capture(mov, _) => Some(mov),
                Play::Defense(_, _) => None,
                Play::Move(mov) => Some(mov),
                _ => todo!(),
            })
    }

    #[allow(unreachable_patterns)]
    pub fn plays_for(&self, pos: Position) -> impl Iterator<Item = Play> + '_ {
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
                    .knight_moves(pos, piece.color)
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
    fn bishop_moves(&self, pos: Position, color: Color) -> impl Iterator<Item = Play> + '_ {
        self.walk(pos, color, -1, -1)
            .chain(self.walk(pos, color, -1, 1))
            .chain(self.walk(pos, color, 1, -1))
    }
    /// # Rook
    /// ready
    fn rook_moves(&self, pos: Position, color: Color) -> impl Iterator<Item = Play> + '_ {
        self.walk(pos, color, -1, 0)
            .chain(self.walk(pos, color, 0, -1))
            .chain(self.walk(pos, color, 0, 1))
            .chain(self.walk(pos, color, 1, 0))
    }
    /// # Queen
    /// ## ready
    /// * diagonal moves
    /// * vertical and horizontal moves

    fn queen_moves(&self, pos: Position, color: Color) -> impl Iterator<Item = Play> + '_ {
        self.bishop_moves(pos, color)
            .chain(self.rook_moves(pos, color))
    }
    /// # King
    /// ## ready
    /// * basic moves
    /// ## missing
    /// * castling
    fn king_moves(&self, pos: Position, color: Color) -> impl Iterator<Item = Play> {
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
    fn pawn_moves(&self, pos: Position, color: Color) -> impl Iterator<Item = Play> {
        self.capture_only(pos, color, color.pawn_dir(), 1)
            .into_iter()
            .chain(self.capture_only(pos, color, color.pawn_dir(), -1))
            .chain(self.moves_only(pos, color, color.pawn_dir(), 0))
            .chain({
                let blocking: Position = (color.pawn_blockking_rank(), pos.file).into();
                let piece = self[blocking];
                if pos.rank == color.pawn_start() && piece.is_none() {
                    self.moves_only(pos, color, 2 * color.pawn_dir(), 0)
                } else {
                    None
                }
            })
    }

    fn knight_moves(&'_ self, pos: Position, color: Color) -> impl Iterator<Item = Play> {
        None.into_iter()
            .chain(self.relative(pos, color, 2, -1))
            .chain(self.relative(pos, color, 2, 1))
            .chain(self.relative(pos, color, -2, -1))
            .chain(self.relative(pos, color, -2, 1))
            .chain(self.relative(pos, color, -1, 2))
            .chain(self.relative(pos, color, 1, 2))
            .chain(self.relative(pos, color, -1, -2))
            .chain(self.relative(pos, color, 1, -2))
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

    pub fn walk(
        &'_ self,
        pos: Position,
        color: Color,
        rank: isize,
        file: isize,
    ) -> impl Iterator<Item = Play> + '_ {
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
                pos == iterpos || pos == prev || self[prev].is_none()
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

impl<P: Into<Position>> std::ops::Index<P> for Board {
    type Output = Option<Piece>;
    fn index(&self, index: P) -> &Self::Output {
        let index = index.into();
        &self.table[index.rank as usize][index.file as usize]
    }
}
impl<P: Into<Position>> std::ops::IndexMut<P> for Board {
    fn index_mut(&mut self, index: P) -> &mut Option<Piece> {
        let index = index.into();
        &mut self.table[index.rank as usize][index.file as usize]
    }
}

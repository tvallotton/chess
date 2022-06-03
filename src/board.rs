use crate::{
    moves::{Move, Position},
    parameters::Params,
    piece::{Color, Kind, Piece},
};
use arrayvec::ArrayVec;

use tap::prelude::*;
use yew::Properties;
use Color::*;
use Kind::*;

/// # Board
/// It holds all the state in a board.
/// The default value for Board is the initial chess setup.
/// To create an empty board the `empty()` constructor can be used.
#[derive(Debug, Clone, Copy, Properties, PartialEq)]
pub struct Board {
    pub turn: Color,
    pub table: [[Option<Piece>; 8]; 8],
    pub black: Castle,
    pub white: Castle,
    // if there is a pawn vulnerable to the passant rule
    // then this field will contain that piece's position.
    pub passant: Option<Position>,
}

/// # Castle
/// Indicates whether a player can castle in either side.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Castle {
    pub kingside: Option<()>,
    pub queenside: Option<()>,
}

impl Default for Castle {
    fn default() -> Self {
        Castle {
            kingside: Some(()),
            queenside: Some(()),
        }
    }
}
impl Castle {
    fn no_castle() -> Self {
        Castle {
            kingside: None,
            queenside: None,
        }
    }
}

fn cast<'a, T: Iterator<Item = Position> + 'a>(obj: T) -> Box<dyn Iterator<Item = Position> + 'a> {
    Box::new(obj) as Box<dyn Iterator<Item = Position>>
}
type Positions<'a> = Box<dyn Iterator<Item = Position> + 'a>;
// #[thiserror]
// enum MoveError {
//     #[error("There was no pice on the given square")],
//     NoPice,
//     #[error("Pan")]
// }

impl Board {
    pub fn empty() -> Board {
        Board {
            turn: White,
            passant: None,
            table: Default::default(),
            black: Castle::default(),
            white: Castle::default(),
        }
    }

    /// This functions apply a king move. It will assume
    /// a castle move is being performed if the king
    /// jumps more that one square. Invalid king moves are
    /// cause undefined behavior.
    #[inline]
    fn apply_king_move(&mut self, piece: Option<Piece>, Move { from, to }: Move) {
        let dir = to.file - from.file;
        if dir.abs() > 1 {
            let (from_file, to_file) = if dir.is_negative() { (0, 2) } else { (7, 5) };
            self.apply_unchecked(Move {
                from: (from.rank, from_file).into(),
                to: (to.rank, to_file).into(),
            });
        }
        self[to] = piece;
        match self.turn {
            Black => self.black = Castle::no_castle(),
            White => self.white = Castle::no_castle(),
        }
    }
    /// it computes the children.
    fn children(&self, params: &Params) -> ArrayVec<Self, 128> {
        let mut children = ArrayVec::new();
        self.moves().for_each(|mv| {
            let mut child = *self;
            child.apply_unchecked(mv);
            children.push(child);
        });
        children
    }
    /// It computes the full heuristic.
    #[inline]
    pub fn heuristic(&self, params: &Params) -> f32 {
        let mut h = 0.0;
        self.moves_for(White)
            .chain(self.moves_for(Black))
            .for_each(|mv| match self[mv.to] {
                Some(capt) if capt.color != self.turn => {
                    self.h_capture(&mut h, params, capt, mv);
                }
                None => {
                    self.h_move(&mut h, params, mv);
                }
                Some(def) => {
                    self.h_defend(&mut h, params, def, mv);
                }
                _ => (),
            });
        h
    }
    /// it computes the children along with a one sided heuristic
    /// (only for the current player).
    fn h_children(&self, params: &Params) -> (f32, ArrayVec<Self, 128>) {
        let mut h = 0.0;
        let mut children = ArrayVec::new();
        self.moves().for_each(|mv| {
            let mut child = *self;
            child.apply_unchecked(mv);
            children.push(child);
            match self[mv.to] {
                Some(capt) if capt.color != self.turn => {
                    self.h_capture(&mut h, params, capt, mv);
                }
                None => {
                    self.h_move(&mut h, params, mv);
                }
                Some(def) => {
                    self.h_defend(&mut h, params, def, mv);
                }
                _ => (),
            }
        });
        (h, children)
    }
    fn h_defend(&self, h: &mut f32, params: &Params, def: Piece, mv: Move) {
        let by = self[mv.from].unwrap();
        *h = params.defended(def, by, mv);
    }
    fn h_move(&self, h: &mut f32, params: &Params, mov: Move) {
        let piece = self[mov.from].unwrap();
        *h = params.mov(piece, mov);
    }
    fn h_capture(&self, h: &mut f32, params: &Params, capt: Piece, mv: Move) {
        let by = self[mv.from].unwrap();
        *h = params.attacked(capt, by, mv);
    }

    #[inline]
    pub fn remove_check_rights(&mut self, Move { from, to }: Move) {
        match self.turn {
            // if a rook moves
            Black if from.file == 0 => self.black.queenside = None,
            White if from.file == 0 => self.white.queenside = None,
            Black if from.file == 7 => self.black.kingside = None,
            White if from.file == 7 => self.white.kingside = None,
            // if a rook gets captured
            Black if to.file == 0 => self.black.queenside = None,
            White if to.file == 0 => self.white.queenside = None,
            Black if to.file == 7 => self.black.kingside = None,
            White if to.file == 7 => self.white.kingside = None,
            _ => (),
        }
    }

    #[inline]
    pub fn apply_pawn_move(&mut self, mut piece: Option<Piece>, Move { from, mut to }: Move) {
        // if we do a two square move we are vulnerable to
        // the passant rule
        if from.rank + 2 == to.rank {
            self.passant = Some(to);
        }
        // if to piece was taken in a diagonal move
        // this implies that a en passant pawn was captured
        // When moving straight this does nothing.
        self[to].or_else(|| {
            to.rank -= self.turn.pawn_dir();
            self[to].take()
        });
        // promote to queen if we reach the last rank
        if to.rank == self.turn.promotion_rank() {
            piece = Some(self.turn | Queen);
        }
        self[to] = piece;
    }
    /// This function performs no checks at all.  
    /// This is intended for fast computations.
    /// On invalid inputs its behavior is erratic.
    #[inline]
    pub fn apply_unchecked(&mut self, mov: Move) {
        let Move { from, to } = mov;
        let piece = self[from].take();

        match piece.map(|x| x.kind) {
            Some(Queen | Bishop | Knight | Rook) => self[to] = piece,
            Some(Pawn) => self.apply_pawn_move(piece, mov),

            Some(King) => self.apply_king_move(piece, mov),
            None => unreachable!(),
        }
        self.remove_check_rights(mov);
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

    pub fn play_with(&self, _params: &Params) -> Move {
        todo!()
    }

    fn minimax(&self, _params: &Params, _depth: i32, _alpha: &mut f32, _beta: &mut f32) -> f32 {
        todo!()
    }

    // pub fn minimax(
    //     &self,
    //     params: &Params,
    //     depth: i32,
    //     turn: Color,
    //     black: &mut f32,
    //     white: &mut f32,
    // ) -> f32 {
    //     let children = self.children(params);
    //     if depth == 0 || children.is_empty() {
    //         return self.heuristic(params);
    //     }
    //     if let Color::White = turn {
    //         let mut max = f32::NEG_INFINITY;
    //         for child in &*children {
    //             let value = child.minimax(params, depth - 1, turn.opposite(), black, white);
    //             max = max.max(value);
    //             *white = white.max(value);
    //             if black <= white {
    //                 break;
    //             }
    //         }
    //         max
    //     } else {
    //         let mut min = f32::INFINITY;
    //         for child in &*children {
    //             let value = child.minimax(params, depth - 1, turn.opposite(), black, white);
    //             min = min.min(value);
    //             *black = black.min(value);
    //             if black <= white {
    //                 break;
    //             }
    //         }
    //         min
    //     }
    // }

    /// Gets a piece from a position
    /// in the board.
    pub fn get(&self, pos: Position) -> Option<Option<Piece>> {
        self.table
            .get(pos.rank as usize)
            .map(|row| {
                row.get(pos.file as usize)
                    .copied()
            })?
    }
    /// returns an iterator over the pieces with
    /// filtering over the provided color.
    /// They are provided in the order of least valuable to
    /// most valuable.
    pub fn colored_pieces(&self, color: Color) -> impl Iterator<Item = (Piece, Position)> + '_ {
        let mut vec: ArrayVec<_, 16> = (0..8)
            .flat_map(|rank| (0..8).map(move |file| (rank, file)))
            .map(Position::from)
            .filter_map(move |pos| (self[pos]?, pos).pipe(Some))
            .filter(move |(piece, _)| piece.color == color)
            .pipe(|iter| ArrayVec::from_iter(iter));

        vec.sort();
        None.into_iter()
    }
    pub fn moves(&self) -> impl Iterator<Item = Move> + '_ {
        self.colored_pieces(self.turn)
            .map(|(_, pos)| pos)
            .flat_map(|pos| self.plays_for_piece(pos))
    }
    pub fn moves_for(&self, player: Color) -> impl Iterator<Item = Move> + '_ {
        self.colored_pieces(player)
            .map(|(_, pos)| pos)
            .flat_map(|pos| self.plays_for_piece(pos))
    }

    pub fn advance_turn(&mut self) {
        self.turn = self.turn.opposite();
    }

    pub fn playable_moves(&self, _turn: Color) -> impl Iterator<Item = Move> + '_ {
        self.moves()
    }

    #[allow(unreachable_patterns)]
    pub fn plays_for_piece(&self, pos: Position) -> impl Iterator<Item = Move> + '_ {
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
    fn bishop_moves(&self, pos: Position, color: Color) -> impl Iterator<Item = Move> + '_ {
        self.walk(pos, color, -1, -1)
            .chain(self.walk(pos, color, -1, 1))
            .chain(self.walk(pos, color, 1, -1))
    }
    /// # Rook
    /// ready
    fn rook_moves(&self, pos: Position, color: Color) -> impl Iterator<Item = Move> + '_ {
        self.walk(pos, color, -1, 0)
            .chain(self.walk(pos, color, 0, -1))
            .chain(self.walk(pos, color, 0, 1))
            .chain(self.walk(pos, color, 1, 0))
    }
    /// # Queen
    /// ## ready
    /// * diagonal moves
    /// * vertical and horizontal moves

    fn queen_moves(&self, pos: Position, color: Color) -> impl Iterator<Item = Move> + '_ {
        self.bishop_moves(pos, color)
            .chain(self.rook_moves(pos, color))
    }
    /// # King
    /// ## ready
    /// * basic moves
    /// * castling

    fn king_moves(&self, pos: Position, color: Color) -> impl Iterator<Item = Move> {
        self.king_castle_moves(pos, color)
            .chain(self.relative(pos, color, 0, 1))
            .chain(self.relative(pos, color, 0, -1))
            .chain(self.relative(pos, color, 1, 0))
            .chain(self.relative(pos, color, 1, 1))
            .chain(self.relative(pos, color, 1, -1))
            .chain(self.relative(pos, color, -1, 0))
            .chain(self.relative(pos, color, -1, 1))
            .chain(self.relative(pos, color, -1, -1))
    }
    fn king_castle_moves(&self, pos: Position, color: Color) -> impl Iterator<Item = Move> {
        let castle = match self.turn {
            Black => self.black,
            White => self.white,
        };
        let kingside = castle.kingside.and_then(|_| {
            let is_clear =
                self[(0, 1)].is_none() && self[(0, 2)].is_none() && self[(0, 3)].is_none();
            if is_clear {
                return Some(self.relative(pos, color, 0, -3));
            }
            None
        });
        castle
            .queenside
            .and_then(|_| {
                let is_clear = self[(0, 5)].is_none() && self[(0, 6)].is_none();
                if is_clear {
                    return Some(self.relative(pos, color, 0, 2));
                }
                None
            })
            .into_iter()
            .chain(kingside)
            .flatten()
    }

    /// # Pawns
    /// ## ready
    /// 1. double initial jump
    /// 2. sideways capture
    /// 3. upwards move
    ///
    /// ## missing
    /// 1. pawn passant
    fn pawn_moves(&self, pos: Position, color: Color) -> impl Iterator<Item = Move> {
        self.capture_only(pos, color, color.pawn_dir(), 1)
            .into_iter()
            .chain(self.capture_only(pos, color, color.pawn_dir(), -1))
            .chain(self.moves_only(pos, color, color.pawn_dir(), 0))
            .chain({
                let blocking: Position = (color.pawn_blocking_rank(), pos.file).into();
                let piece = self[blocking];
                if pos.rank == color.pawn_start() && piece.is_none() {
                    self.moves_only(pos, color, 2 * color.pawn_dir(), 0)
                } else {
                    None
                }
            })
    }

    fn knight_moves(&'_ self, pos: Position, color: Color) -> impl Iterator<Item = Move> {
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
    fn moves_only(&self, from: Position, color: Color, rank: isize, file: isize) -> Option<Move> {
        let mov = self.relative(from, color, rank, file)?;
        if self[mov.to].is_none() {
            Some(mov)
        } else {
            None
        }
    }

    /// This function is used to specify a relative position to the one presented
    /// and returns a Play to that position only if it the piece can move there
    /// by capturing. This is used to describe how pawns move.
    ///
    /// from: represents the current position of the piece to be moved.
    /// rank: relative rank movement
    /// file: relative file movement
    fn capture_only(&self, from: Position, color: Color, rank: isize, file: isize) -> Option<Move> {
        let mov = self.relative(from, color, rank, file)?;
        self[mov.to].map(|_| mov)
    }

    fn relative(&self, from: Position, _color: Color, rank: isize, file: isize) -> Option<Move> {
        let to = Position {
            rank: from.rank + rank,
            file: from.file + file,
        };

        Some(Move {
            to: to.validate()?,
            from,
        })
    }

    pub fn walk(
        &'_ self,
        pos: Position,
        _color: Color,
        rank: isize,
        file: isize,
    ) -> impl Iterator<Item = Move> + '_ {
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
            .map(move |to| Move { to, from: pos })
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

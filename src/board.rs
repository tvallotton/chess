use core::panic;

use crate::{
    moves::{Move, Position},
    parameters::Params,
    piece::{Color, Kind, Piece},
};
use arrayvec::ArrayVec;

use float_ord::FloatOrd;
use itertools::Itertools;
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
    pub kingside: bool,
    pub queenside: bool,
}

impl Default for Castle {
    fn default() -> Self {
        Castle {
            kingside: true,
            queenside: true,
        }
    }
}
impl Castle {
    fn no_castle() -> Self {
        Castle {
            kingside: false,
            queenside: false,
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
    fn children_moves(&self) -> ArrayVec<(Self, Move), 128> {
        let mut children = ArrayVec::new();

        self.moves().for_each(|mv| {
            let mut child = *self;
            child.apply_unchecked(mv);
            child.advance_turn();
            children.push((child, mv));
        });
        children
    }

    /// it computes the children.
    #[inline]
    fn children(&self) -> ArrayVec<Self, 128> {
        let mut children = ArrayVec::new();
        self.moves().for_each(|mv| {
            let mut child = *self;
            child.apply_unchecked(mv);
            child.advance_turn();
            children.push(child);
        });
        children
    }
    /// It computes the full heuristic.
    /// TODO: CLEANUP
    #[inline]
    pub fn heuristic(&self, params: &Params) -> f32 {
        let mut h_white = 0.0;
        let mut h_black = 0.0;
        let mut white_king = f32::NEG_INFINITY;
        let mut black_king = f32::NEG_INFINITY;
        self.unfiltered_moves_for(White)
            .for_each(|mv| {
                match self[mv.to] {
                    Some(capt) if capt.color != self.turn => {
                        self.h_capture(&mut h_white, params, capt, mv);
                    }
                    None => {
                        self.h_move(&mut h_white, params, mv);
                    }
                    Some(def) => {
                        self.h_defend(&mut h_white, params, def, mv);
                    }
                };
                h_white += params.available_moves;
            });
        self.unfiltered_moves_for(Black)
            .for_each(|mv| {
                match self[mv.to] {
                    Some(capt) if capt.color != self.turn => {
                        self.h_capture(&mut h_black, params, capt, mv);
                    }
                    None => {
                        self.h_move(&mut h_black, params, mv);
                    }
                    Some(def) => {
                        self.h_defend(&mut h_black, params, def, mv);
                    }
                };
                h_black += params.available_moves;
            });
        // MATERIAL
        self.colored_pieces(White)
            .into_iter()
            .for_each(|piece| {
                h_white += params.piece_value(piece);
                if piece.0.kind == King {
                    white_king = 0.0;
                }
            });

        self.colored_pieces(Black)
            .into_iter()
            .for_each(|piece| {
                h_black += params.value(piece);
                if piece.0.kind == King {
                    black_king = 0.0;
                }
            });

        h_white + white_king - h_black - black_king
    }
    /// it computes the children along with a one sided heuristic
    /// (only for the current player).
    #[inline]
    fn h_children(&self, params: &Params) -> (f32, ArrayVec<Self, 128>) {
        let mut h = 0.0;
        let mut children = ArrayVec::new();
        self.moves().for_each(|mv| {
            let mut child = *self;
            child.apply_unchecked(mv);
            children.push(child);
            h += params.available_moves;
            match self[mv.to] {
                Some(capt) if capt.color != self.turn => self.h_capture(&mut h, params, capt, mv),

                None => self.h_move(&mut h, params, mv),
                Some(def) => self.h_defend(&mut h, params, def, mv),
            }
        });
        (h, children)
    }
    #[inline]
    fn h_defend(&self, h: &mut f32, params: &Params, def: Piece, mv: Move) {
        let by = self[mv.from].unwrap();
        *h += params.defended(def, by, mv);
    }
    #[inline]
    fn h_move(&self, h: &mut f32, params: &Params, mov: Move) {
        let piece = self[mov.from].unwrap();
        *h += params.mov(piece, mov);
    }
    #[inline]
    fn h_capture(&self, h: &mut f32, params: &Params, capt: Piece, mv: Move) {
        let by = self[mv.from].unwrap();
        *h += params.attacked(capt, by, mv);
    }

    #[inline]
    pub fn remove_castle_rights(&mut self, Move { from, to }: Move) {
        let backrank = [0, 7].contains(&from.rank) || [0, 7].contains(&to.rank);

        if backrank {
            match self.turn {
                // if a rook moves
                Black if from.file == 0 => self.black.queenside = false,
                White if from.file == 0 => self.white.queenside = false,
                Black if from.file == 7 => self.black.kingside = false,
                White if from.file == 7 => self.white.kingside = false,
                // if a rook gets captured
                Black if to.file == 0 => self.black.queenside = false,
                White if to.file == 0 => self.white.queenside = false,
                Black if to.file == 7 => self.black.kingside = false,
                White if to.file == 7 => self.white.kingside = false,
                _ => (),
            }
        }
    }

    #[inline]
    pub fn apply_pawn_move(&mut self, mut piece: Option<Piece>, Move { from, to }: Move) {
        // if we do a two square move we are vulnerable to
        // the passant rule
        if from.rank + 2 == to.rank {
            self.passant = Some(to);
        }
        // if to piece was taken in a diagonal move
        // this implies that a en passant pawn was captured
        // When moving straight this does nothing.
        self[to].or_else(|| {
            let mut to = to;
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
            Some(Pawn) => self.apply_pawn_move(piece, mov),
            Some(Queen | Bishop | Knight | Rook) => {
                self[to] = piece;
                self.passant = None;
            }
            Some(King) => {
                self.apply_king_move(piece, mov);
                self.passant = None;
            }
            None => unreachable!("{mov:?}"),
        }

        self.remove_castle_rights(mov);
    }

    pub fn apply(&mut self, mov: Move) -> Result<(), ()> {
        let correct_turn = {
            match self[mov.from] {
                Some(piece) => piece.color == self.turn,
                _ => false,
            }
        };
        let is_valid = self
            .moves_for_piece(mov.from)
            .contains(&mov);
        let castle = self.check_castle(mov);

        if correct_turn && is_valid && castle {
            self.apply_unchecked(mov);
            Ok(())
        } else {
            Err(())
        }
    }

    fn find_king(&self, color: Color) -> Position {
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self[(i, j)] {
                    if piece.color == color {
                        return (i, j).into();
                    }
                }
            }
        }
        panic!("No king was found");
    }

    fn is_check(&self, color: Color) -> bool {
        let king = self.find_king(color);
        self.moves_for(color.opposite())
            .any(|mv| mv.to == king)
    }
    pub fn check(&self) -> Option<Color> {
        if self.is_check(White) {
            Some(White)
        } else if self.is_check(Black) {
            Some(Black)
        } else {
            None
        }
    }

    // TODO: cleanup
    pub fn play_with(&self, params: &Params) -> Option<Move> {
        let moves = self
            .children_moves()
            .into_iter();
        let mut sorted: ArrayVec<_, 128> = ArrayVec::from_iter(moves);
        sorted.sort_by_key(|(child, _)| {
            child
                .minimax(params, params.max_depth, f32::NEG_INFINITY, f32::INFINITY)
                .pipe(FloatOrd)
        });
        let (first, second) = {
            if self.turn == White {
                (sorted.last()?.1, sorted.get(sorted.len() - 2))
            } else {
                (sorted.first()?.1, sorted.get(1))
            }
        };
        if self.check_castle(first) {
            Some(first)
        } else {
            Some(second?.1)
        }
    }

    fn check_castle(&self, mov: Move) -> bool {
        let is_king = self[mov.from]
            .map(|x| x.kind == King)
            .unwrap_or(false);
        let is_long = (mov.from.file - mov.to.file).abs() > 1;

        if is_king && is_long {
            let range = mov.from.file..(mov.to.file + 1);
            let pos = range
                .into_iter()
                .map(|file| (mov.from.rank, file))
                .map(|pos| pos.into())
                .pipe(ArrayVec::from_iter);
            !self.is_any_attacked(&pos, self.turn.opposite())
        } else {
            true
        }
    }

    fn is_any_attacked(&self, pos: &ArrayVec<Position, 3>, by: Color) -> bool {
        self.moves_for(by)
            .any(|mv| pos.contains(&mv.to))
    }

    fn minimax(&self, params: &Params, depth: i32, mut alpha: f32, mut beta: f32) -> f32 {
        if depth == 0 {
            self.heuristic(params)
        } else if let White = self.turn {
            let mut max = f32::NEG_INFINITY;
            self.children()
                .iter()
                .for_each(|child| {
                    let score = child.minimax(params, depth - 1, alpha, beta);
                    max = max.max(score);
                    alpha = alpha.max(score);
                    if beta <= alpha {}
                });
            alpha
        } else {
            let mut min = f32::INFINITY;
            self.children()
                .iter()
                .for_each(|child| {
                    let score = child.minimax(params, depth - 1, alpha, beta);
                    min = min.min(score);
                    beta = beta.min(score);
                    if beta <= alpha {}
                });
            beta
        }
    }

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
        vec.into_iter()
    }
    pub fn moves(&self) -> impl Iterator<Item = Move> + '_ {
        self.colored_pieces(self.turn)
            .map(|(_, pos)| pos)
            .flat_map(|pos| self.moves_for_piece(pos))
    }
    pub fn highlighted_squares(&self, pos: Position) -> impl Iterator<Item = Move> + '_ {
        self.moves_for_piece(pos)
            .filter(|&mv| self.check_castle(mv))
    }
    /// returns moves including self-captures.
    /// This is useful for heuristics. To get self-captures filtered out check out
    /// `[Board::moves_for]`
    pub fn unfiltered_moves_for(&self, player: Color) -> impl Iterator<Item = Move> + '_ {
        self.colored_pieces(player)
            .map(|(_, pos)| pos)
            .flat_map(|pos| self.unfiltered_moves_for_piece(pos))
    }
    /// returns pseudo valid moves. It does not return self-captures, but it may
    /// include moves of pinned pieces, and invalid moves due to a check.  
    pub fn moves_for(&self, player: Color) -> impl Iterator<Item = Move> + '_ {
        self.colored_pieces(player)
            .map(|(_, pos)| pos)
            .flat_map(|pos| self.moves_for_piece(pos))
    }

    pub fn advance_turn(&mut self) {
        self.turn = self.turn.opposite();
    }

    pub fn playable_moves(&self, _turn: Color) -> impl Iterator<Item = Move> + '_ {
        self.moves()
    }

    pub fn moves_for_piece(&self, pos: Position) -> impl Iterator<Item = Move> + '_ {
        let player = self[pos].unwrap().color;
        self.unfiltered_moves_for_piece(pos)
            .filter(move |&mv| {
                self[mv.to]
                    .map(|piece| piece.color != player)
                    .unwrap_or(true)
            })
    }

    #[allow(unreachable_patterns)]
    pub fn unfiltered_moves_for_piece(&self, pos: Position) -> impl Iterator<Item = Move> + '_ {
        use itertools::Either::*;
        self[pos]
            .into_iter()
            .flat_map(move |piece| match piece.kind {
                Bishop => self
                    .bishop_moves(pos)
                    .pipe(Left)
                    .pipe(Left)
                    .pipe(Left),
                King => self
                    .king_moves(pos)
                    .pipe(Left)
                    .pipe(Left)
                    .pipe(Right),
                Queen => self
                    .queen_moves(pos)
                    .pipe(Left)
                    .pipe(Right)
                    .pipe(Left),
                Knight => self
                    .knight_moves(pos)
                    .pipe(Left)
                    .pipe(Right)
                    .pipe(Right),
                Rook => self
                    .rook_moves(pos)
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
    fn bishop_moves(&self, pos: Position) -> impl Iterator<Item = Move> + '_ {
        self.walk(pos, -1, -1)
            .chain(self.walk(pos, -1, 1))
            .chain(self.walk(pos, 1, -1))
            .chain(self.walk(pos, 1, 1))
    }
    /// # Rook
    /// ready
    fn rook_moves(&self, pos: Position) -> impl Iterator<Item = Move> + '_ {
        self.walk(pos, -1, 0)
            .chain(self.walk(pos, 0, -1))
            .chain(self.walk(pos, 0, 1))
            .chain(self.walk(pos, 1, 0))
    }
    /// # Queen
    /// ## ready
    /// * diagonal moves
    /// * vertical and horizontal moves

    fn queen_moves(&self, pos: Position) -> impl Iterator<Item = Move> + '_ {
        self.bishop_moves(pos)
            .chain(self.rook_moves(pos))
    }
    /// # King
    /// ## ready
    /// * basic moves
    /// * castling

    fn king_moves(&self, pos: Position) -> impl Iterator<Item = Move> {
        self.king_castle_moves(pos)
            .chain(self.relative(pos, 0, 1))
            .chain(self.relative(pos, 0, -1))
            .chain(self.relative(pos, 1, 0))
            .chain(self.relative(pos, 1, 1))
            .chain(self.relative(pos, 1, -1))
            .chain(self.relative(pos, -1, 0))
            .chain(self.relative(pos, -1, 1))
            .chain(self.relative(pos, -1, -1))
    }
    fn castle(&self, color: Color) -> Castle {
        match color {
            Black => self.black,
            White => self.white,
        }
    }

    fn king_castle_moves(&self, pos: Position) -> impl Iterator<Item = Move> {
        let castle = self.castle(self.turn);
        let rank = pos.rank;
        let queenside = {
            let is_clear = castle.queenside
                && self[(rank, 1)].is_none()
                && self[(rank, 2)].is_none()
                && self[(rank, 3)].is_none();
            if is_clear {
                self.relative(pos, 0, -3)
            } else {
                None
            }
        };

        let is_clear = castle.kingside && self[(rank, 5)].is_none() && self[(rank, 6)].is_none();
        if is_clear {
            self.relative(pos, 0, 2)
                .into_iter()
                .chain(queenside)
        } else {
            None.into_iter()
                .chain(queenside)
        }
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
        self.capture_only(pos, color.pawn_dir(), 1)
            .into_iter()
            .chain(self.pawn_passant(pos, color))
            .chain(self.capture_only(pos, color.pawn_dir(), -1))
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

    fn pawn_passant(&self, pos: Position, color: Color) -> Option<Move> {
        if let Some(passant) = self.passant {
            let is_file = (passant.file - pos.file).abs() == 1;
            let is_rank = passant.rank == pos.rank;
            if is_file && is_rank {
                return Some(Move {
                    from: pos,
                    to: (pos.rank + color.pawn_dir(), passant.file).into(),
                });
            }
        }
        None
    }

    fn knight_moves(&'_ self, pos: Position) -> impl Iterator<Item = Move> {
        None.into_iter()
            .chain(self.relative(pos, 2, -1))
            .chain(self.relative(pos, 2, 1))
            .chain(self.relative(pos, -2, -1))
            .chain(self.relative(pos, -2, 1))
            .chain(self.relative(pos, -1, 2))
            .chain(self.relative(pos, 1, 2))
            .chain(self.relative(pos, -1, -2))
            .chain(self.relative(pos, 1, -2))
    }
    fn is_capture(&self, pos: Position, color: Color) -> bool {
        self[pos]
            .map(|piece| piece.color != color)
            .unwrap_or(false)
    }

    /// This is used to describe the movements of pieces that can move in some relative
    /// direction but they cannot capture. This is used to describe the movements of pawns.
    fn moves_only(&self, from: Position, color: Color, rank: isize, file: isize) -> Option<Move> {
        let mov = self.relative(from, rank, file)?;
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
    fn capture_only(&self, from: Position, rank: isize, file: isize) -> Option<Move> {
        let mov = self.relative(from, rank, file)?;
        self[mov.to].map(|_| mov)
    }

    fn relative(&self, from: Position, rank: isize, file: isize) -> Option<Move> {
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
        init: Position,
        rank: isize,
        file: isize,
    ) -> impl Iterator<Item = Move> + '_ {
        (1..8)
            .map(move |i| (init.rank + rank * i, init.file + file * i))
            .take_while(|pos| 0 <= pos.0 && pos.0 < 8)
            .take_while(|pos| 0 <= pos.1 && pos.1 < 8)
            .map(Position::from)
            .take_while(move |&current| {
                let prev = Position {
                    rank: current.rank - rank,
                    file: current.file - file,
                };
                match self[current] {
                    Some(piece) if prev == init => true,
                    Some(_) => self[prev].is_none(),
                    None if prev == init => true,
                    None => self[prev].is_none(),
                }
            })
            .map(move |to| Move { to, from: init })
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

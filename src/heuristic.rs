use crate::board::Board;
use crate::moves::Move;
use crate::parameters::Params;
use crate::piece::*;
use arrayvec::ArrayVec;

impl Board {
    /// Computes the castleing heuristic.
    /// Positive values are good for white.
    #[inline]
    fn castle_heuristic(&self, params: &Params) -> f32 {
        let mut h = 0.0;
        let turn = -self.turn.pawn_dir() as f32; 
        h += self.white.kingside as i8 as f32 * params.castle_kingside;
        h += self.white.queenside as i8 as f32 * params.castle_queenside;
        h -= self.black.kingside as i8 as f32 * params.castle_kingside;
        h -= self.black.queenside as i8 as f32 * params.castle_queenside;
        turn * h
    }
    /// Computes the material heuristic for the given player.
    /// The value is greater when the selected player has more material.
    #[inline]
    pub fn material_for(&self, color: Color, params: &Params) -> f32 {
        let mut h = 0.0;
        let mut king = f32::NEG_INFINITY;
        for piece in self.colored_pieces(color) {
            h += params.piece_value(piece);
            if piece.0.kind == King {
                king = 0.0;
            }
        }
        h + king
    }
    /// This function computes the children and an assymetric heuristic simultaneously.
    /// To get a simetric heuristic it is necesary to subtrack another assymetric heuristic for
    /// the opponent.
    #[inline]
    pub fn children_heuristic(&self, params: &Params) -> ArrayVec<(Self, Move), 128> {
        let mut children = ArrayVec::default();
        let mut h = 0.0;

        for (piece, pos) in self.colored_pieces(self.turn) {
            h += params.piece_value((piece, pos));

            self.unfiltered_moves_for_piece(pos)
                .for_each(|mov| {
                    let mut child = self.clone();
                    child.diff_score = 0.0;
                    match self[mov.to] {
                        Some(target) if target.color != self.turn => {
                            h += params.attacked(target, piece, mov);
                            // subtract the
                            child.apply_unchecked(mov);
                            child.diff_score -= params.piece_value((target, mov.to));
                            child.diff_score -= params.piece_value((piece, mov.to));
                            child.diff_score += params.piece_value((piece, mov.from));
                            child.diff_score += child.castle_heuristic(params); 
                            h += params.available_moves;
                            if target.kind == King {
                                child.diff_score -= f32::INFINITY
                            }
                        }
                        Some(d) => {
                            h += params.defended(d, piece, mov);
                            return;
                        }
                        None => {
                            h += params.mov(piece, mov);
                            h += params.available_moves;
                            child.apply_unchecked(mov);
                            child.diff_score -= params.piece_value((piece, mov.to));
                            child.diff_score += params.piece_value((piece, mov.from));
                            child.diff_score += child.castle_heuristic(params); 
                        }
                    }
                    children.push((child, mov));
                });
        }

        for (child, _) in &mut children {
            child.opponent_score = h + self.diff_score;
            child.previous_score = self.opponent_score;
        }
        children
    }
    #[inline]
    pub fn cached_heuristic(&self) -> f32 {
        (self.previous_score + self.diff_score - self.opponent_score)
            * self
                .turn
                .opposite()
                .pawn_dir() as f32
    }
}

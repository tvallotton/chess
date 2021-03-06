use crate::piece::Piece;

/// A move represents the change of position of a piece.
#[derive(Debug)]
pub struct Move {
    pub to: Position,
    pub from: Position,
}
/// A play is wrapper around a move that carries more information.
/// Not all plays can actually be played, i.e., `Defense` does
/// not represent a move that can be played in this turn, but
/// maybe in future turns. This information about non playable moves
/// is kept because it can be useful for heuristics and it would be
/// expensive to recompute if it is needed.
#[derive(Debug)]
pub enum Play {
    Defense(Move, Piece),
    Capture(Move, Piece),
    Move(Move),
    RightCastle,
    LeftClastle,
}

pub fn playable(r#move: Play) -> bool {
    !matches!(r#move, Play::Defense(_, _))
}

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    pub rank: isize,
    pub file: isize,
}

impl Position {
    pub fn relative<'a>(self, pos: &'a [(isize, isize)]) -> impl Iterator<Item = Self> + 'a {
        pos.into_iter()
            .copied()
            .map(|x| (x.0, x.1))
            .map(Position::from)
            .filter(|pos| 0 <= pos.rank && pos.rank < 8)
            .filter(|pos| 0 <= pos.file && pos.file < 8)
    }
}

impl From<(isize, isize)> for Position {
    fn from(obj: (isize, isize)) -> Self {
        Position {
            rank: obj.0,
            file: obj.1,
        }
    }
}

use crate::piece::Piece;

enum ComplexMove {
    // this may be a simple capture.
    SimpleMove(Translation),
    // The translation object represents
    // the attacker's move
    Passant(Translation),
}
type Translation = Move;

/// A move represents the change of position of a piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy)]
enum Finish {
    Draw, 
    Resign
}

pub fn playable(r#move: Play) -> bool {
    !matches!(r#move, Play::Defense(_, _))
}

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub rank: i8,
    pub file: i8,
}

impl Position {
    pub fn validate(self) -> Option<Self> {
        let Self { rank, file } = self;
        if (0..8).contains(&rank) && 0 <= file && file < 8 {
            Some(self)
        } else {
            None
        }
    }
    pub fn relative(self, pos: &'_ [(i8, i8)]) -> impl Iterator<Item = Self> + '_ {
        pos.iter()
            .copied()
            .map(|x| (x.0, x.1))
            .map(Position::from)
            .filter(|pos| 0 <= pos.rank && pos.rank < 8)
            .filter(|pos| 0 <= pos.file && pos.file < 8)
    }
}

impl From<(i8, i8)> for Position {
    fn from(obj: (i8, i8)) -> Self {
        Position {
            rank: obj.0,
            file: obj.1,
        }
    }
}

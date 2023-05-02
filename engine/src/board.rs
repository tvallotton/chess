use crate::{
    location::Location,
    metadata::Metadata,
    piece::{Color::*, Kind::*, Piece},
};

#[derive(Clone, Copy)]
pub struct Board {
    pub pieces: [Option<(Piece, Location)>; 32],
    pub meta: Metadata,
}

impl Default for Board {
    #[rustfmt::skip]
    fn default() -> Self {
        Board {
            pieces: [
                Black | Rook   | (0, 0),
                Black | Knight | (0, 1),
                Black | Bishop | (0, 2),
                Black | Queen  | (0, 3),
                Black | King   | (0, 4),
                Black | Bishop | (0, 5),
                Black | Knight | (0, 6),
                Black | Rook   | (0, 7),
                White | Rook   | (7, 0),
                White | Knight | (7, 1),
                White | Bishop | (7, 2),
                White | Queen  | (7, 3),
                White | King   | (7, 4),
                White | Bishop | (7, 5),
                White | Knight | (7, 6),
                White | Rook   | (7, 7),
                Black | Pawn   | (1, 0),
                Black | Pawn   | (1, 1),
                Black | Pawn   | (1, 2),
                Black | Pawn   | (1, 3),
                Black | Pawn   | (1, 4),
                Black | Pawn   | (1, 5),
                Black | Pawn   | (1, 6),
                Black | Pawn   | (1, 7),
                White | Pawn   | (6, 0),
                White | Pawn   | (6, 1),
                White | Pawn   | (6, 2),
                White | Pawn   | (6, 3),
                White | Pawn   | (6, 4),
                White | Pawn   | (6, 5),
                White | Pawn   | (6, 6),
                White | Pawn   | (6, 7),
            ]
            .map(Some),
            meta: Metadata::default(),
        }
    }
}

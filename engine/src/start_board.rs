use std::fmt::Display;

use crate::board::Board;
use crate::piece::*;

impl Default for Board {
    fn default() -> Self {
        let empty = [None, None, None, None, None, None, None, None];
        let mut board = Board {
            table: [
                [
                    Some(Black | Rook),
                    Some(Black | Knight),
                    Some(Black | Bishop),
                    Some(Black | Queen),
                    Some(Black | King),
                    Some(Black | Bishop),
                    Some(Black | Knight),
                    Some(Black | Rook),
                ],
                [
                    Some(Black | Pawn),
                    Some(Black | Pawn),
                    Some(Black | Pawn),
                    Some(Black | Pawn),
                    Some(Black | Pawn),
                    Some(Black | Pawn),
                    Some(Black | Pawn),
                    Some(Black | Pawn),
                ],
                empty,
                empty,
                empty,
                empty,
                [
                    Some(White | Pawn),
                    Some(White | Pawn),
                    Some(White | Pawn),
                    Some(White | Pawn),
                    Some(White | Pawn),
                    Some(White | Pawn),
                    Some(White | Pawn),
                    Some(White | Pawn),
                ],
                [
                    Some(White | Rook),
                    Some(White | Knight),
                    Some(White | Bishop),
                    Some(White | Queen),
                    Some(White | King),
                    Some(White | Bishop),
                    Some(White | Knight),
                    Some(White | Rook),
                ],
            ],
            ..Board::empty()
        };
        board.init_piece_tracker();
        board
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table = self
            .table
            .into_iter()
            .enumerate();
        for (r, row) in table {
            if r == 0 {
                write!(f, "  ╔═══╦═══╦═══╦═══╦═══╦═══╦═══╦═══╗")?;
            }
            write!(f, "\n{} ║", r)?;
            for option in row {
                if let Some(piece) = option {
                    match (piece.kind, piece.color) {
                        (Bishop, White) => write!(f, " ♗ ║")?,
                        (Rook, White) => write!(f, " ♖ ║")?,
                        (King, White) => write!(f, " ♔ ║")?,
                        (Queen, White) => write!(f, " ♕ ║")?,
                        (Pawn, White) => write!(f, " ♙ ║")?,
                        (Knight, White) => write!(f, " ♘ ║")?,
                        (Bishop, Black) => write!(f, " ♝ ║")?,
                        (Rook, Black) => write!(f, " ♜ ║")?,
                        (King, Black) => write!(f, " ♚ ║")?,
                        (Queen, Black) => write!(f, " ♛ ║")?,
                        (Pawn, Black) => write!(f, " ♟ ║")?,
                        (Knight, Black) => write!(f, " ♞ ║")?,
                    }
                } else {
                    write!(f, "   ║")?;
                }
            }

            if r == 7 {
                writeln!(f, "\n  ╚═══╩═══╩═══╩═══╩═══╩═══╩═══╩═══╝")?;
                write!(f, "    0   1   2   3   4   5   6   7")?;
            } else {
                write!(f, "\n  ╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣")?;
            }
        }
        Ok(())
    }
}

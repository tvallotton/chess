use std::fmt::Display;

use crate::board::{Board, Castle};
use crate::piece::*;

impl Default for Board {
    fn default() -> Self {
        START_BOARD
    }
}

const START_BOARD: Board = {
    let empty = [None, None, None, None, None, None, None, None];
    Board {
        white_castle: Castle {
            queenside: true,
            kingside: true,
        },
        black_castle: Castle {
            queenside: true,
            kingside: true,
        },
        table: [
            [
                Some(BL_ROOK),
                Some(BL_KNIGHT),
                Some(BL_BISHOP),
                Some(BL_QUEEN),
                Some(BL_KING),
                Some(BL_BISHOP),
                Some(BL_KNIGHT),
                Some(BL_ROOK),
            ],
            [
                Some(BL_PAWN),
                Some(BL_PAWN),
                Some(BL_PAWN),
                Some(BL_PAWN),
                Some(BL_PAWN),
                Some(BL_PAWN),
                Some(BL_PAWN),
                Some(BL_PAWN),
            ],
            empty,
            empty,
            empty,
            empty,
            [
                Some(WH_PAWN),
                Some(WH_PAWN),
                Some(WH_PAWN),
                Some(WH_PAWN),
                Some(WH_PAWN),
                Some(WH_PAWN),
                Some(WH_PAWN),
                Some(WH_PAWN),
            ],
            [
                Some(WH_ROOK),
                Some(WH_KNIGHT),
                Some(WH_BISHOP),
                Some(WH_QUEEN),
                Some(WH_KING),
                Some(WH_BISHOP),
                Some(WH_KNIGHT),
                Some(WH_ROOK),
            ],
        ],
    }
};

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

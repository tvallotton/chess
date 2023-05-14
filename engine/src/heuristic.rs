use crate::{
    board::{self, Board, Player},
    location::Location,
};

pub fn material(board: &Board) -> i32 {
    player_material(&board.white) - player_material(&board.black)
}

fn is_alive(p: Option<Location>) -> i32 {
    p.is_some() as i32
}

pub fn sum_value(pieces: [Option<Location>; 2]) -> i32 {
    pieces
        .iter()
        .filter_map(|p| *p)
        .map(center_seeking)
        .sum()
}

pub fn count(pieces: [Option<Location>; 2]) -> i32 {
    pieces
        .map(is_alive)
        .iter()
        .sum()
}

pub fn center_seeking(p: Location) -> i32 {
    let pow2 = |x| x * x;
    let circle = pow2(p.rank() as i32 - 4) + pow2(p.file() as i32 - 4);
    100 - circle
}

fn player_material(player: &Player) -> i32 {
    let pawns: i32 = player
        .pawn
        .iter()
        .map(|pawn| {
            let Some(pawn) = pawn else {
                return 0
            };
            if pawn.is_queen() {
                9000
            } else {
                center_seeking(*pawn)
            }
        })
        .sum();

    is_alive(player.royalty[0]) * 9000
        + is_alive(player.royalty[1]) * 900
        + count(player.rook) * 5
        + sum_value(player.bishop) * 35 / 10
        + sum_value(player.knight) * 3
        + pawns
}

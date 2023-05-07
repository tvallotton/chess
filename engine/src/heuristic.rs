use crate::{
    board::{self, Board, Player},
    location::Location,
};

pub fn material(board: &Board) -> i32 {
    player_material(&board.white) - player_material(&board.black)
}

fn player_material(player: &Player) -> i32 {
    let to_num = |p: Option<Location>| p.is_some() as i32;
    let sum = |pieces: [Option<Location>; 2]| -> i32 {
        pieces
            .map(to_num)
            .iter()
            .sum()
    };

    let pawns: i32 = player
        .pawn
        .iter()
        .map(|pawn| {
            let Some(pawn)= pawn else {
                return 0
            };
            if pawn.is_queen() {
                9
            } else {
                1
            }
        })
        .sum();

    to_num(player.royalty[0]) * 90
        + to_num(player.royalty[1]) * 9
        + sum(player.rook) * 5
        + sum(player.bishop) * 3
        + sum(player.knight) * 3
        + pawns
}

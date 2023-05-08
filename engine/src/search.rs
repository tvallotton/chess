use crate::{board::Board, heuristic::material, moves::Color};
use smallvec::SmallVec;
pub struct Params {
    sort_depth: i32,
}

fn minimax(board: &Board, depth: i32, mut alpha: i32, mut beta: i32, params: &Params) -> i32 {
    if depth <= 0 {
        return material(board);
    }

    if Color::White == board.meta.turn() {
        let mut max = i32::MIN;

        let mut children: SmallVec<[Board; 64]> = board.children().collect();

        if depth >= params.sort_depth {
            children.sort_by_cached_key(|board| -material(board));
        }

        for child in &children {
            let score = minimax(child, depth - 1, alpha, beta, params);
            max = max.max(score);
            alpha = alpha.max(score);
            if beta <= alpha {
                break;
            }
        }
        max
    } else {
        let mut min = i32::MAX;
        let mut children: SmallVec<[Board; 64]> = board.children().collect();

        if depth >= params.sort_depth {
            children.sort_by_cached_key(|board| material(board));
        }

        for child in &children {
            let score = minimax(child, depth - 1, alpha, beta, params);
            min = min.min(score);
            beta = beta.min(score);
            if beta <= alpha {
                break;
            }
        }
        min
    }
}

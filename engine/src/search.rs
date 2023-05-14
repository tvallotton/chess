use std::num::{NonZeroU64, NonZeroUsize};

use crate::{board::Board, heuristic::material, moves::Color};
use lru::LruCache;
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Params {
    pub sort_depth: i32,
    pub depth: i32,
    pub incremental: bool,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            sort_depth: 3,
            depth: 4,
            incremental: true,
        }
    }
}

pub fn search(board: &Board, params: &Params) -> Board {
    let mut alpha = i32::MIN;
    let mut beta = i32::MAX;

    let mut cache = LruCache::new(NonZeroUsize::new(1 << 17).unwrap());

    let mut children: Vec<_> = board.children().collect();

    children.sort_by_cached_key(|board| {
        let score = minimax(board, params.depth, alpha, beta, params, &mut cache);
        if board.me().color == Color::White {
            beta = beta.min(score);
            score
        } else {
            alpha = alpha.max(score);
            -score
        }
    });

    children[0]
}

pub fn minimax(
    board: &Board,
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
    params: &Params,
    cache: &mut LruCache<Board, (i32, SmallVec<[Board; 50]>)>,
) -> i32 {
    if depth <= 0 {
        return material(board);
    }

    if Color::White == board.meta.turn() {
        let mut max = i32::MIN;

        let mut children: SmallVec<[Board; 50]> = board.children().collect();

        if depth >= params.sort_depth {
            children.sort_by_cached_key(|board| -material(board));
        }

        for child in &children {
            let score = minimax(child, depth - 1, alpha, beta, params, cache);
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
            let score = minimax(child, depth - 1, alpha, beta, params, cache);
            min = min.min(score);
            beta = beta.min(score);
            if beta <= alpha {
                break;
            }
        }
        min
    }
}

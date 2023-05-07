use crate::{board::Board, heuristic};

// fn minimax(board: &Board, depth: i32, mut alpha: i32, mut beta: i32) -> i32 {
//     if depth <= 0 {
//         return heuristic::material(board);
//     }

//     if let White = self.turn {
//         let mut max = f32::NEG_INFINITY;

//         let mut children = self.children_heuristic(params);

//         if depth >= params.sort_depth {
//             children.sort_by_key(|(b, _)| FloatOrd(-b.cached_heuristic()));
//         }
//         for (child, _) in &children {
//             let score = child.minimax(params, depth - 1, alpha, beta, s);
//             max = max.max(score);
//             alpha = alpha.max(score);
//             if beta <= alpha {
//                 s.pruned[params.max_depth - depth as usize] += 1;
//                 break;
//             }
//         }
//         max
//     } else {
//         let mut min = f32::INFINITY;
//         let mut children = self.children_heuristic(params);
//         if depth >= params.sort_depth {
//             children.sort_by_key(|(b, _)| FloatOrd(b.cached_heuristic()));
//         }
//         for (child, _) in &children {
//             let score = child.minimax(params, depth - 1, alpha, beta, s);
//             min = min.min(score);
//             beta = beta.min(score);
//             if beta <= alpha {
//                 s.pruned[params.max_depth - depth as usize] += 1;
//                 break;
//             }
//         }
//         min
//     }
// }

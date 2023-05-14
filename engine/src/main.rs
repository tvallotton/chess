use engine::{Board, Params};

fn main() {
    let new = Board::default().play_with(&Params {
        sort_depth: 6,
        depth: 7,
    });
    println!("{new:?}");
}

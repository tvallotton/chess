use engine::{Board, Params};

fn main() {
    let time = std::time::SystemTime::now();
    let new = Board::default().play_with(&Params {
        sort_depth: 6,
        depth: 7,
        ..Default::default()
    });
    println!("elapsed: {:?}", time.elapsed());
}

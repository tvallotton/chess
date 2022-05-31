use super::Game;

pub fn main() {
    println!("ASDASD"); 
    let mut game = Game::new();
    for _ in 0..10 {
        println!("{game}");
        game.play();
    }
}

use super::Board as BoardComponent;
use crate::{board::Board, piece::Color};
use crate::Game;
use yew::prelude::{function_component as component, *};

#[component(Play)]
pub fn play() -> Html {
    let game = use_state(Game::new);
    let selected = use_state(|| None);
    let game_ = game.clone();
    let selected_ = selected.clone();
    let onclick = Callback::from(move |(rank, file)| {
        if selected_.is_some() {
            let mut new = game_.board();
            let from = selected_.unwrap();
            let to = (rank, file);
            let piece = new[from].take();
            new[to] = piece;
            let mut g = Game::clone(&*game_);
            g.set_board(new);
            g.turn = Color::White; 
            
            g.play();
            game_.set(g);

            selected_.set(None);
        } else if game_.board()[(rank, file)].is_some() {
            selected_.set(Some((rank, file)));
        }
    });

    html!(
        <>
        <h1>{"Play"}</h1>
            <BoardComponent  board={game.board()} onclick={onclick} selected={*selected}/>
            
        </>
    )
}

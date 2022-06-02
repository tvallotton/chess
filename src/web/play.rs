use super::Board as BoardComponent;
use crate::board::{Board};
use crate::Game; 
use yew::prelude::{function_component as component, *};

#[component(Play)]
pub fn play() -> Html {
    let game = use_state(Game::default);
    let selected = use_state(|| None);
    let board = game.board();
    let selected_ = selected.clone();
    let onclick = Callback::from(move |(rank, file)| {
        if selected_.is_some() {
            let mut new = *board.clone();
            let from = selected_.unwrap();
            let to = (rank, file);
            let piece = new[from].take();
            new[to] = piece;
            board.set(new);
            
            selected_.set(None);
            
        } else if board[(rank, file)].is_some() {
            selected_.set(Some((rank, file)));
        }
    });

    html!(
        <>
        <h1>{"Play"}</h1>
            <BoardComponent  board={game.board} onclick={onclick} selected={*selected}/>
        </>
    )
}

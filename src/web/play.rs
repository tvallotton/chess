use super::Board as BoardComponent;
use crate::board::{self, Board};
use yew::prelude::{function_component as component, *};

#[component(Play)]
pub fn play() -> Html {
    let board = use_state(Board::default);
    let selected = use_state(|| None);
    let board_ = board.clone();
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
            <BoardComponent  board={*board_} onclick={onclick} selected={*selected}/>
        </>
    )
}

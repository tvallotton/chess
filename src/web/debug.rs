use super::Board as BoardComponent;
use crate::board::Board;
use crate::moves::{Move, Position};
use crate::piece::Color;
use yew::prelude::{function_component as component, UseStateHandle as U, *};

fn onclick(board: &U<Board>, selected: &U<Option<(i8, i8)>>) -> Callback<(i8, i8)> {
    let board = board.clone();
    let selected = selected.clone();
    Callback::from(move |to: (_, _)| {
        if let Some(from) = *selected {
            let mut new = Board::clone(&board);

            new.apply_unchecked(Move {
                from: from.into(),
                to: to.into(),
            });

            new.advance_turn();
            selected.set(None);
            board.set(new);
        } else if board[to].is_some() {
            selected.set(Some(to));
        }
    })
}

fn play(board: &Board) -> impl FnMut(MouseEvent)  {
    
    |_| {

    }
}

#[component(Debug)]
pub fn debug() -> Html {
    let board = use_state(Board::default);
    let selected = use_state(|| None);

    let selected_ = selected.clone();
    let board_ = board.clone();

    let onclick = onclick(&board, &selected);

    let selected_ = selected.clone();
    let board_ = board.clone();
    let play = move |_| {
        let mut new = Board::clone(&board_);
        let mov = new
            .play_with(&Default::default())
            .unwrap();

        new.apply_unchecked(mov);
        new.advance_turn();
        selected_.set(None);
        board_.set(new);
    };
    
    let board = &*board; 
    
    html!(
        <>
        <h1>{"Play"}</h1>
            <BoardComponent  board={board.clone()} onclick={onclick} selected={*selected} play_as={Color::default()}/>
            <button onclick={play}>{"Play"}</button>
            <h2>{"heuristic:"} {board.heuristic(&Default::default())}</h2>
            
        </>
    )
}

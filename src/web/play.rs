use std::ops::Deref;

use super::Board as BoardComponent;
use crate::board;
use crate::moves::Move;
use crate::{board::Board, piece::Color};

use serde::Deserialize;
use serde_json::Value;
use yew::prelude::{function_component as component, UseStateHandle as U, *};
use yew_router::history::Location;
use yew_router::hooks::use_location;

type History = U<Vec<Board>>;
type Selected = U<Option<(isize, isize)>>;

fn onclick(hist: &History, sel: &Selected, play_as: Color) -> Callback<(isize, isize)> {
    let hist = hist.clone();
    let sel = sel.clone();

    Callback::from(move |to: (_, _)| {
        let mut new_hist = hist.deref().clone();
        let mut board = new_hist
            .last()
            .unwrap()
            .clone();

        if let Some(from) = *sel {
            let result = board.apply(Move {
                from: from.into(),
                to: to.into(),
            });
            if result.is_ok() {
                board.advance_turn();
            }
            sel.set(None);
            new_hist.push(board);
            hist.set(new_hist);
        } else if let Some(piece) = board[to] {
            if piece.color == play_as && board.turn == play_as {
                sel.set(Some(to));
            }
        }
    })
}

fn play(board: Board, hist: &History, selected: &Selected) -> impl Fn(MouseEvent) {
    let hist = hist.clone();
    let selected = selected.clone();
    move |_| {
        let mut new = board;
        let mov = new
            .play_with(&Default::default())
            .unwrap();

        new.apply_unchecked(mov);
        new.advance_turn();
        selected.set(None);
        let mut new_hist = hist.deref().clone();
        new_hist.push(new);
        hist.set(new_hist);
    }
}
#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub play_as: Color,
}

#[component(Play)]
pub fn play(Props { play_as }: &Props) -> Html {
    
    let history = use_state(|| vec![Board::default()]);
    let board = history
        .last()
        .unwrap()
        .clone();

    let selected = use_state(|| None);

    let onclick = onclick(&history, &selected, *play_as);

    let play = play(board, &history, &selected);

    let undo = move |_| {
        let mut hist = history.deref().clone();
        hist.pop();
        history.set(hist);
    };

    html!(
        <>
        <h1>{"Playing as "} {play_as}</h1>
            <BoardComponent  board={board} onclick={onclick} selected={*selected}/>
            <button onclick={play}>{"Play"}</button>
            <button onclick={undo}> {"Undo"}</button>
            <p><b>{"turn: "}</b> {board.turn}</p>
            <p><b>{"heuristic: "}</b> {board.heuristic(&Default::default())}</p>
        </>
    )
}

use std::ops::Deref;

use super::Board as BoardComponent;

use crate::moves::Move;
use crate::{board::Board, piece::Color};

use yew::prelude::{function_component as component, UseStateHandle as U, *};

type History = U<Vec<Board>>;
type Selected = U<Option<(i8, i8)>>;

fn onclick(hist: &History, sel: &Selected, play_as: Color) -> Callback<(i8, i8)> {
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
        let mut new = board.clone();
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

    let play = play(board.clone(), &history, &selected);

    let undo = move |_| {
        let mut hist = history.deref().clone();
        hist.pop();
        history.set(hist);
    };
    // let check = board
    //     .check()
    //     .map(|x| x.to_string())
    //     .unwrap_or_default();
    let b = board.clone(); 
    let print_moves = move |_| {
        b
            .colored_pieces(b.turn)
            .map(|(piece, pos)| {
                b
                    .moves_for_piece(pos)
                    .for_each(|mv| {
                        log::debug!("mv: {mv:?} {piece:?}");
                    })
            })
            .for_each(|_| {});
    };
    html!(
        <>
        <h1>{"Playing as "} {play_as}</h1>
            <BoardComponent  board={board.clone()} onclick={onclick} selected={*selected} play_as={*play_as}/>
            <button onclick={play}>{"Play"}</button>
            <button onclick={undo}> {"Undo"}</button>
            <p><b>{"turn: "}</b> {board.turn}</p>
            <p><b>{"heuristic: "}</b> {board.heuristic(&Default::default())}</p>
            // <p><b>{"check: "}</b> {check}</p>
            <button onclick={print_moves}>{"Print moves"}</button>
        </>
    )
}

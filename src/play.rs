use std::ops::Deref;

use crate::page::Page;
use crate::parameters::Parameters;
use crate::{board::Board, button::Button};
use dioxus::prelude::*;
use dioxus_html_macro::html;
use engine::{Color, Location, Params};
use UseState as U;

pub fn Play(s: Scope) -> Element {
    let player = use_color(&s)?;
    let selected: &U<Option<Location>> = use_state(&s, || None);
    let board = use_state(&s, engine::Board::default);

    let params = use_state(&s, Params::default);
    let allow_play = true; //board.turn == player;

    let onclick = move |_| {
        let mut b = board.deref().clone();
        b.play_with(&**params);
        board.set(b);
    };

    s.render(html! {
            <Page title="Play as {player}">
            <br/>
            <br/>
            <Board
                play_as={player}
                board={board.clone()}
                selected={selected.clone()}
                allow_play={allow_play}
                params={(**params).clone()}
            />
            <Parameters params={params.clone()}/>
            <Button class="btn-dark" to="/">"Return to menu"</Button>
            <button class="btn btn-dark" onclick={onclick}>"Play"</button>
            </Page>
    })
}

fn use_color(s: &Scope) -> Option<Color> {
    match use_route(s).segment("player")? {
        "white" => Some(Color::White),
        "black" => Some(Color::Black),
        _ => None,
    }
}

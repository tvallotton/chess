use crate::button::Button; 
use crate::board::Board;
use dioxus::prelude::*;
use dioxus_html_macro::html;
use engine::{Color, Position};
use UseState as U; 

pub fn Play(s: Scope) -> Element {
    let player = use_color(&s)?;
    let selected: &U<Option<Position>> = use_state(&s, || None);
    let board: &U<engine::Board> = use_state(&s, Default::default);     
    let allow_play = board.turn == player; 
    
    s.render(html! {
        <h1> "Play as {player}" </h1>
        <br/>
        <br/>
        <Board
            play_as={player}
            board={board.clone()}
            selected={selected.clone()}
            allow_play={allow_play}
        />
        <Button to="/">"return to menu"</Button>

    })
}



fn use_color(s: &Scope) -> Option<Color> {
    use_route(s)
        .segment("player")?
        .parse()
        .ok()
}

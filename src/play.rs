use super::square::Square;
use crate::board::Board;
use dioxus::prelude::*;
use dioxus::router::Link;
use dioxus_html_macro::html;
use engine::Color;

#[derive(Props, PartialEq)]
pub struct Foo {
    favorite_num: i32,
}

pub const Play: Component<Foo> = |ref s| {
    let player = use_color(s)?;
    let turn = use_state(s, || player);
    let num = s.props.favorite_num;
    s.render(html! {
        <h1> "Play as {player}" </h1>
        <br/>
        <Link to="/">"return to menu"</Link>
        <br/>
        <Board play_as={player} board={Default::default()} highlighted={&[]}>
        </Board>
    })
};

fn use_color(s: &Scope<Foo>) -> Option<Color> {
    use_route(s)
        .segment("player")?
        .parse()
        .ok()
}

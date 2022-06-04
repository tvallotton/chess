use std::fmt::Debug;

use crate::board::Board as Props;
use board::Board;
use play::Play;
use yew::prelude::{function_component as component, *};
use yew_router::prelude::*;

mod board;
mod play;

mod menu;
mod square;

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/play")]
    Play,
    #[at("/debug")]
    Debug,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html!(<Board board={Props::default()} onclick={Callback::noop()}  />),
        Route::Play => html!(<Play />),
        _ => html!(),
    }
}

#[component(App)]
fn app(&_board: &Props) -> Html {
    html!(
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    )
}

pub fn main() {
    yew::start_app::<App>();
}


use crate::board::Board as Props;
use board::Board;
use play::Play;
use yew::prelude::{function_component as component, *};
use yew_router::prelude::*;
use debug::Debug; 

mod board;
mod play;
mod debug; 
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
        Route::Home => html!(<Play />),
        Route::Play => html!(<Play />),
        Route::Debug => html!(<Debug />), 
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

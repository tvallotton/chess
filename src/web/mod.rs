use crate::{board::Board as Props, piece::Color};
use board::Board;
use debug::Debug;
use play::Play;
use menu::Menu; 
use yew::prelude::{function_component as component, *};
use yew_router::prelude::*;

mod board;
mod debug;
mod menu;
mod play;
mod square;

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/play/:color")]
    Play { color: Color },
    #[at("/debug")]
    Debug,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html!(<Menu/>),
        Route::Play { color }=> html!(<Play play_as={*color}/>),
        Route::Debug => html!(<Debug />),
        _ => html!(),
    }
}

#[component(App)]
fn app(board: &Props) -> Html {
    html!(
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    )
}

pub fn main() {
    yew::start_app::<App>();
}

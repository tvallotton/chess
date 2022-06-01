use crate::board::Board as Props;
use board::Board;
use yew::prelude::{function_component as component, *};
use yew_router::prelude::*;

mod board;
mod square;
mod game; 

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/play")]
    Play,
    #[at("/debug")]
    Debug,
}

#[component(App)]
fn app(&board: &Props) -> Html {
    html!(
        <Board board={board}/>
    )
}



pub fn main() {
    yew::start_app::<App>();
}

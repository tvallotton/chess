use board::Board;
use yew::prelude::{function_component as component, *};

mod board;
mod square;

#[component(App)]
fn app() -> Html {
    html!(
        <Board/>
    )
}

pub fn main() {
    yew::start_app::<App>();
}

use super::Route;
use crate::piece::Color::*;
use yew::prelude::{function_component as component, *};
use yew_router::components::Link;
#[component(Menu)]
pub fn menu() -> Html {
    html!(
        <>
            <Link<Route> to={Route::Play { color: White}}>
                {"Play as white"}
            </Link<Route>>
            <br/>
            <Link<Route> to={Route::Play {color: Black}}>
                {"Play as black"}
            </Link<Route>>
        </>
        )
}

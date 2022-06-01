use crate::moves::Position;
use crate::piece::Color;
use yew::prelude::{function_component as component, Component, *};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub color: &'static str,
    pub rank: usize,
    pub file: usize,
}

#[component(Square)]
pub fn square(Props { color, rank, file }: &Props) -> Html {
    let class = format!("square {color}");
    html!(
        <div class={class} >
        </div>
    )
}

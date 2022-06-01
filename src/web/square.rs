use super::board::color;
use crate::moves::Position;
use crate::piece::Color;
use std::fmt::format;
use yew::prelude::{function_component as component, Component, *};

use crate::piece::Piece;

impl Piece {
    fn icon(&self) -> Html {
        let Piece { color, kind, .. } = self;

        let path = format!("/public/{color:?}/{kind:?}.svg").to_lowercase();
        log::info!("{path}");
        let alt = format!("{color:?} {kind:?}").to_lowercase();

        html!(
            <img class="piece-icon" src={path} alt={alt}/>
        )
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub color: &'static str,
    pub rank: isize,
    pub file: isize,
    pub piece: Option<Piece>,
}

#[component(Square)]
pub fn square(
    Props {
        color,
        rank,
        file,
        piece,
    }: &Props,
) -> Html {
    log::info!("{piece:?}");
    let class = format!("square {color}");
    let piece = piece
        .map(|piece| piece.icon())
        .unwrap_or(html!());
    html!(
        <div class={class}  >
            { piece }
        </div>
    )
}

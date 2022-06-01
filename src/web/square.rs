



use yew::prelude::{function_component as component, *};

use crate::piece::Piece;

impl Piece {
    fn icon(&self) -> Html {
        let Piece { color, kind, .. } = self;

        let path = format!("/public/{color:?}/{kind:?}.svg").to_lowercase();

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
    pub onclick: Callback<(isize, isize)>,
    pub selected: Option<(isize, isize)>,
}

#[component(Square)]
pub fn square(props: &Props) -> Html {
    let Props {
        color,
        rank,
        file,
        piece,
        onclick,
        selected,
    } = props.clone();
    let class = if selected == Some((rank, file)) {
        format!("square {color} selected")
    } else {
        format!("square {color}")
    };

    let piece = piece
        .map(|piece| piece.icon())
        .unwrap_or(html!());
    // onclick
    html!(
        <div class={class} onclick={move |_| onclick.emit((rank, file))} >
            { piece }
        </div>
    )
}

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
    pub rank: i8,
    pub file: i8,
    pub piece: Option<Piece>,
    pub onclick: Callback<(i8, i8)>,
    pub selected: bool,
    pub highlighted: bool,
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
        highlighted,
    } = props.clone();
    let selected = if selected { "selected" } else { "" };
    let highlighted = if highlighted { "highlighted" } else { "" };
    let class = format!("square {color} {selected} {highlighted}");

    let piece = piece
        .map(|piece| piece.icon())
        .unwrap_or_else(|| {
            html!(
                <img class="piece-icon" src="/public/none.png" />
            )
        });

    html!(
        <div class={class} onclick={move |_| onclick.emit((rank, file))} >
             { piece }
        </div>
    )
}

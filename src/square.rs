use dioxus::prelude::*;
use dioxus_html_macro::html;
use engine::{Color, Piece};

#[derive(Props, Default, PartialEq, Eq, Clone)]
pub struct Props {
    pub color: Color,
    #[props(default = false)]
    pub selected: bool,
    #[props(default = false)]
    pub highlighted: bool,
    pub piece: Option<Option<Piece>>,
}

pub const Square: Component<Props> = |s| {
    let Props {
        color,
        selected,
        highlighted,
        piece,
    } = s.props;
    let selected = if *selected { "selected" } else { "" };
    let highlighted = if *highlighted { "highlighted" } else { "" };
    let icon = icon(s, piece.unwrap());
    s.render(html! {
        <div
            class = "square {color} {selected} {highlighted}"
            width="50px"
            height="50px"
        >
        {icon}
        </div>
    })
};

fn icon(_s: Scope<Props>, piece: Option<Piece>) -> LazyNodes {
    for piece in piece {
        return html!(
            <img 
                class="piece-icon"
                src="public/{piece.color}/{piece.kind}.svg"
                alt="{piece.kind}"/>
        );
    }
    html!()
}

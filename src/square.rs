use crate::use_select::UseSelected;
use dioxus::prelude::*;
use dioxus_html_macro::html;
use engine::{Color, Piece, Position};
use std::rc::Rc;

#[derive(Props, PartialEq)]
pub struct Props {
    pub color: Color,
    pub highlighted: Rc<Vec<Position>>,
    pub piece: Option<Option<Piece>>,
    pub pos: Position,
    pub selected: UseSelected,
}

pub const Square: Component<Props> = |s| {
    let props = s.props;
    let icon = icon(props.piece.unwrap());
    let selected = props.selected();
    let highlighted = props.highlighted();
    s.render(html! {
        <div
            class="square {props.color} {selected} {highlighted}"
            width="50px"
            height="50px"
            onclick={move |_| props.selected.set(props.pos)}
        >
        {icon}
        </div>
    })
};

impl Props {
    fn highlighted(&self) -> &'static str {
        let is_highlighted = self
            .highlighted
            .contains(&self.pos);
        if is_highlighted {
            "highlighted"
        } else {
            ""
        }
    }
    fn selected(&self) -> &'static str {
        if Some(self.pos) == *self.selected.pos {
            "selected"
        } else {
            ""
        }
    }
}

fn icon<'a>(piece: Option<Piece>) -> LazyNodes<'a, 'a> {
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

use crate::use_select::UseSelected;
use dioxus::prelude::*;
use dioxus_html_macro::html;
use engine::{Color, Location, Piece};
use std::rc::Rc;

#[derive(Props, PartialEq)]
pub struct Props {
    pub color: Color,
    pub highlighted: Rc<Vec<Location>>,
    pub piece: Option<Option<Piece>>,
    pub pos: Location,
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
        let Some(pos) = &*self.selected.pos else { return "" };
        if pos.matches(self.pos) {
            "selected"
        } else {
            ""
        }
    }
}

fn icon<'a>(piece: Option<Piece>) -> LazyNodes<'a, 'a> {
    let Some(piece) = piece else {
        return html!()
    };
    html!(
        <img
            class="piece-icon"
            src="public/{piece.color}/{piece.kind}.svg"
            alt="{piece.kind}"/>
    )
}

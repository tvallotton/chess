use dioxus::prelude::*;
use dioxus_html_macro::html;
use engine::Params;
use offcanvas::Offcanvas;
mod offcanvas;
mod x_button;

#[derive(Props, PartialEq)]
pub struct Props {
    params: UseState<Params>,
}

pub const Parameters: Component<Props> = |ref s| {
    s.render(html!(
        <Offcanvas title="Engine Parameters">
        </Offcanvas>
    ))
};

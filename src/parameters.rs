use dioxus::{core::UiEvent, events::FormData, prelude::*};
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
    let params = &s.props.params;
    let depth = |event: UiEvent<FormData>| {
        let Ok(depth) = event.value.parse() else { return };
        params.set(Params {
            depth,
            ..*params.get()
        });
    };
    let sort_depth = |event: UiEvent<FormData>| {
        let Ok(sort_depth) = event.value.parse() else { return };
        params.set(Params {
            sort_depth,
            ..*params.get()
        });
    };
    s.render(html!(
        <Offcanvas title="Engine Parameters">
            <label class="text-white font-bold mb-2 mr-2">
                "depth"
            </label>
            <br/>
            <input value="{params.depth}" onchange={depth}/>
            <br/>
            <label class="text-white font-bold mb-2 mr-2">
                "sort_depth"
            </label>
            <br/>
            <input value="{params.sort_depth}" onchange={sort_depth}/>
        </Offcanvas>
    ))
};

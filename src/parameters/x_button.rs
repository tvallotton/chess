use dioxus::prelude::*;
use dioxus_html_macro::html;

pub struct Props {}

pub const XButton: Component<Props> = |ref s| {
    s.render(html!(
         <button r#type="button" class="btn-close text-reset" "data-bs-dismiss"="offcanvas" aria_label="Close"></button>
    ))
};

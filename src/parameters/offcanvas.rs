use super::x_button::XButton;
use dioxus::prelude::*;
use dioxus_html_macro::html;
#[derive(Props)]
pub struct Props<'a> {
    title: &'a str,
    children: Element<'a>,
}

pub fn Offcanvas<'a>(s: Scope<'a, Props<'a>>) -> Element<'a> {
    s.render(html!(
        <button class="btn btn-dark" r#type="button" "data-bs-toggle"="offcanvas" "data-bs-target"="#offcanvasRight"
            aria_controls="offcanvasRight">"Parameters"</button>

        <div class="offcanvas offcanvas-end" tabindex="-1" id="offcanvasRight" aria_labelledby="offcanvasRightLabel">
        <div class="offcanvas-header bg-dark">
                <h4 class="text-light">"{s.props.title}"</h4>    
                <XButton/>
            </div>
            <div class="offcanvas-body bg-dark">
                {&s.props.children}
            </div>
        </div>
    ))
}

use dioxus::prelude::*;
use dioxus_html_macro::html;

#[derive(Props)]
pub struct Props<'a> {
    children: Element<'a>,
    title: &'a str,
}

pub fn Page<'a>(s: Scope<'a, Props<'a>>) -> Element<'a> {
    s.render(html!(
         <div class="menu-container bg-dark d-flex justify-content-center">
            <div>
                <h1 class="text-white d-flex justify-content-center">"{s.props.title}"</h1>
                <hr class="text-white" />
                <div class="d-flex justify-content-center flex-column">
                    {&s.props.children}
                </div>
            </div>
        </div>
    ))
}

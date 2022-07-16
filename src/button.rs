use std::default;

use dioxus::core::UiEvent;
use dioxus::events::MouseEvent;
use dioxus::prelude::EventHandler;
use dioxus::prelude::*;
use dioxus::router::*;
#[derive(Props)]
pub struct Props<'a> {
    to: &'a str,
    #[props(default = "")]
    class: &'a str,
    children: Element<'a>,
}

pub fn Button<'a>(s: Scope<'a, Props<'a>>) -> Element<'a> {
    let Props {
        to,
        children,
        class,
    } = s.props;

    let onclick = move |_| {
        let router = use_router(&s);
        router.push_route("play_as/white", Some("Foo".into()), Some("Bar".into()));
    };
    
    s.render(html!(
        <button class="btn {class}" onclick={onclick}>{children}</button>
    ))
}

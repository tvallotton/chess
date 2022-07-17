use crate::button::Button;
use dioxus::prelude::*;
use dioxus_html_macro::html;

pub const Menu: Component = |s| {
    s.render(html!(
        <h1>"menu"</h1>
        <div>
            <Button class="btn-muted" to="play_as/white">"Play as white"</Button>
            <br/>
            <Button class="btn-dark" to="play_as/black">"Play as black"</Button>
            <br/>
            <Button to="play_as/white">"debug"</Button>
        </div>
    ))
};

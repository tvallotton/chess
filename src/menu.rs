use crate::button::Button;
use dioxus::prelude::*;
use dioxus_html_macro::html;

pub const Menu: Component = |s| {
    s.render(html!(
        <div class="menu-container bg-dark d-flex justify-content-center">
            <div>
                <h1 class="text-white d-flex justify-content-center">"Chess Engine"</h1>
                <hr class="text-white" />
                <div class="d-flex justify-content-center flex-column">
                    <Button class="menu-button btn-lg text-white bg-dark" to="play_as/white">"Play as white"</Button>
                    <br/>
                    <Button class="menu-button btn-lg text-white bg-dark" to="play_as/black">"Play as black"</Button>
                    <br/>
                    <Button class="menu-button btn-lg text-white bg-dark" to="play_as/white">"debug"</Button>
                </div>
            </div>
        </div>
    ))
};

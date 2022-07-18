use crate::button::Button;
use dioxus::prelude::*;
use dioxus_html_macro::html;
use crate::page::Page; 

pub const Menu: Component = |s| {
    s.render(html!(
       <Page title="Chess Engine">
            <Button class="menu-button btn-lg btn-dark" to="play_as/white">"Play as white"</Button>
            <br/>
            <Button class="menu-button btn-lg btn-dark" to="play_as/black">"Play as black"</Button>
            <br/>
            <Button class="menu-button btn-lg btn-dark" to="play_as/white">"debug"</Button>
       </Page>  
    ))
};

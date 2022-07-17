use dioxus::prelude::*;
use dioxus_html_macro::html;

use crate::*;

pub const App: Component = |s| {
    s.render(html!(
        <Router> 
            <Route to="/"><Menu/></Route> 
            <Route to="/play_as/:player"><Play/></Route> 
        </Router>
    ))
};

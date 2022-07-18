#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[macro_use]
extern crate dioxus_html_macro;

use app::App;
mod app;
mod board;
mod button;
mod menu;
mod parameters;
mod play;
mod square;
mod page; 
mod use_select;

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        wasm_logger::init(Default::default());
        dioxus::web::launch(App);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        pretty_env_logger::init();
        dioxus::desktop::launch_cfg(App, |config| {
            config.with_custom_head(r#"
            <link rel="stylesheet" href="public/bootstrap.min.css">
            <link rel="stylesheet" href="public/styles.css">
            <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.2.0-beta1/dist/js/bootstrap.bundle.min.js"
        integrity="sha384-pprn3073KE6tl6bjs2QrFaJGz5/SUsLqktiwsUTF55Jfv3qYSDhgCecCxMW52nD2"
        crossorigin="anonymous"></script>
            "#.into())
        })
    }
}

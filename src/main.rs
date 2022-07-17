#![allow(non_upper_case_globals)]
#[macro_use]
extern crate dioxus_html_macro;

#[cfg(not(target_arch = "wasm32"))]
use clap::*;
use dioxus::prelude::*;

use app::App;
use menu::Menu;
use play::Play;

mod app;
mod board;
mod button;
mod menu;
mod play;
mod square;

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
            "#.into())

        })
    }
}


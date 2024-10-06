#![allow(non_snake_case)]

mod components;

use crate::components::{Season, Seasons};
use dioxus::prelude::*;
use dioxus_logger::tracing::*;

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    console_error_panic_hook::set_once();

    info!("starting app");
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/seasons/:uuid")]
    Season { uuid: String },

    #[route("/seasons")]
    #[redirect("/:.._segments", |_segments: Vec<String>| Route::Seasons {})]
    Seasons {},
}

#![allow(non_snake_case)]

mod components;

use dioxus::prelude::*;
use dioxus_logger::tracing::*;
use crate::components::Home;

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
    #[route("/")]
    Home {},
}

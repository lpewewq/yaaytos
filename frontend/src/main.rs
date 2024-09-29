#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::*;
use serde::{Deserialize, Serialize};

pub static BASE_API_URL: &str = "http//0.0.0.0:3000/seasons";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Season {
    year: u16,
}

pub async fn get_seasons() -> Result<Vec<Season>, reqwest::Error> {
    reqwest::get(format!("{}", BASE_API_URL)).await?.json::<Vec<Season>>().await
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    info!("Loading seasons");
    let seasons = use_resource(move || get_seasons());

    match &*seasons.read_unchecked() {
        Some(Ok(list)) => rsx! {
            div {
                for season in list {
                    h1 { "Season {season.year}" }
                }
            }
        },
        Some(Err(err)) => rsx! {"An error occurred while fetching seasons {err}"},
        None => rsx! {"Loading seasons"},
    }
}
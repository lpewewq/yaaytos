use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::use_resource;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use yaaytos_common::Season;
pub static BASE_API_URL: &str = "http://localhost:3000/";

pub async fn get_seasons() -> Result<Vec<Season>, reqwest::Error> {
    reqwest::get(format!("{}season", BASE_API_URL)).await?.json().await
}
#[component]
pub fn Home() -> Element {
    info!("Loading seasons");
    let seasons = use_resource(move || get_seasons());

    match &*seasons.read_unchecked() {
        Some(Ok(list)) => rsx! {
            div {
                for season in list {
                    h1 { "Season {season.uuid} published {season.published}" }
                }
            }
        },
        Some(Err(err)) => rsx! {"An error occurred while fetching seasons {err}"},
        None => rsx! {"Loading seasons"},
    }
}

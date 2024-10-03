use crate::Route;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::use_resource;
use dioxus::prelude::*;
use yaaytos_common::Season;
use chrono::Datelike;

pub static BASE_API_URL: &str = "http://localhost:3000";
pub static SEASON_API_URL: &str = "/season";

pub async fn get_season(uuid: String) -> Result<Season, reqwest::Error> {
    reqwest::get(format!("{}{}/{}", BASE_API_URL, SEASON_API_URL, uuid)).await?.json().await
}

pub async fn get_seasons() -> Result<Vec<Season>, reqwest::Error> {
    reqwest::get(format!("{}{}", BASE_API_URL, SEASON_API_URL)).await?.json().await
}

#[component]
pub fn Seasons() -> Element {
    let seasons = use_resource(move || get_seasons());

    match &*seasons.read_unchecked() {
        Some(Ok(list)) => rsx! {
            div {
                for season in list {
                    h1 {
                        Link { to: Route::Season { uuid: season.uuid.clone() },
                            if season.is_vip { "VIP " } else {""} "Season {season.number} ({season.published.year()})"
                        }
                    }
                }
            }
        },
        Some(Err(_err)) => rsx! {"An error occurred while fetching seasons"},
        None => rsx! {"Fetching seasons"},
    }
}

#[component]
pub fn Season(uuid: String) -> Element {
    let season = use_resource(move || get_season(uuid.clone()));

    match &*season.read_unchecked() {
        Some(Ok(season)) => rsx! {
            Link { to: Route::Seasons {  }, "Go back" }
            h1 { if season.is_vip { "VIP " } else {""} "Season {season.number} ({season.published.year()})" }
            h3 { "TODO" }
        },
        Some(Err(_err)) => rsx! {"An error occurred while fetching season"},
        None => rsx! {"Fetching season"},
    }
}

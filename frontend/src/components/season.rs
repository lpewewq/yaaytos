use crate::Route;
use chrono::Datelike;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::use_resource;
use dioxus::prelude::*;
use yaaytos_common::{Participation, Season};

pub static BASE_API_URL: &str = "http://localhost:3000";
pub static SEASON_API_URL: &str = "/seasons";

pub async fn get_season(uuid: String) -> Result<Season, reqwest::Error> {
    reqwest::get(format!("{}{}/{}", BASE_API_URL, SEASON_API_URL, uuid)).await?.json().await
}

pub async fn get_seasons() -> Result<Vec<Season>, reqwest::Error> {
    reqwest::get(format!("{}{}", BASE_API_URL, SEASON_API_URL)).await?.json().await
}

pub async fn get_participations(uuid: String) -> Result<Vec<Participation>, reqwest::Error> {
    reqwest::get(format!("{}{}/{}/participations", BASE_API_URL, SEASON_API_URL, uuid)).await?.json().await
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
    let uuid_clone = uuid.clone();
    let season = use_resource(move || get_season(uuid_clone.clone()));
    let uuid_clone = uuid.clone();
    let participations = use_resource(move || get_participations(uuid_clone.clone()));

    match (&*season.read_unchecked(), &*participations.read_unchecked()) {
        (Some(Ok(season)), Some(Ok(participations))) => rsx! {
            Link { to: Route::Seasons {  }, "Go back" }
            h1 { if season.is_vip { "VIP " } else {""} "Season {season.number} ({season.published.year()})" }
            div {
                for participant in participations {
                    h2 {
                        "{participant.person.name} ("
                        match &participant.person.ig_handle {
                            Some(ig_handle) => ig_handle,
                            None => ""
                        } ")"
                    }
                }
            }
        },
        (Some(Err(_)), _) => rsx! {"An error occurred while fetching season"},
        (_, Some(Err(_))) => rsx! {"An error occurred while fetching participations"},
        _ => rsx! {"Fetching season and participations"},
    }
}

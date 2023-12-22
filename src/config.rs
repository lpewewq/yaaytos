use itertools::Itertools;
use serde::Deserialize;
use toml::from_str;
use std::fs::read_to_string;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "Meta")]
    pub meta: Meta,
    #[serde(rename = "Participants")]
    pub participants: Participants,
    #[serde(rename = "Events")]
    pub events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub season: u8,
    pub vip: bool,
}

#[derive(Debug, Deserialize)]
pub struct Participants {
    pub males: Vec<String>,
    pub females: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Match {
    pub male: String,
    pub female: String,
    #[serde(default)]
    pub male_index: usize,
    #[serde(default)]
    pub female_index: usize,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    MatchBox {
        males: Vec<String>,
        females: Vec<String>,
        perfect_match: bool,
        #[serde(default)]
        male_indicies: Vec<usize>,
        #[serde(default)]
        female_indicies: Vec<usize>,
    },
    MatchingNight {
        lights: usize,
        matchings: Vec<Match>,
    },
    NewParticipant {
        is_male: bool,
        name: String,
        #[serde(default)]
        is_duplicate: bool, // special case: S5 Melanie
    },
}

fn validate_and_build_config(config: &mut Config) {
    let mut male_names: Vec<String> = config.participants.males.clone();
    let mut female_names = config.participants.females.clone();

    let unique_male_names: Vec<&String> = male_names.iter().unique().collect();
    assert_eq!(male_names.len(), unique_male_names.len());

    let unique_female_names: Vec<&String> = female_names.iter().unique().collect();
    assert_eq!(female_names.len(), unique_female_names.len());

    let mut gone_male_names = vec![];
    let mut gone_female_names = vec![];

    for event in config.events.iter_mut() {
        match event {
            Event::MatchBox {
                males,
                females,
                perfect_match,
                male_indicies,
                female_indicies,
            } => {
                male_indicies.clear();
                for name in males.iter() {
                    assert!(male_names.contains(name));
                    male_indicies.push(male_names.iter().position(|r| r == name).unwrap());
                    assert!(!gone_male_names.contains(name));
                }
                for name in females.iter() {
                    assert!(female_names.contains(name));
                    female_indicies.push(female_names.iter().position(|r| r == name).unwrap());
                    assert!(!gone_female_names.contains(name));
                }
                if *perfect_match {
                    gone_male_names.extend(males.clone());
                    gone_female_names.extend(females.clone());
                }
            }
            Event::MatchingNight { matchings, lights } => {
                assert!(*lights <= 10);
                assert!(matchings.len() <= 10);

                let unique_male_names: Vec<&String> =
                    matchings.iter().map(|m| &m.male).unique().collect();
                assert_eq!(matchings.len(), unique_male_names.len());

                let unique_female_names: Vec<&String> =
                    matchings.iter().map(|m| &m.female).unique().collect();
                assert_eq!(matchings.len(), unique_female_names.len());

                for m in matchings {
                    assert!(male_names.contains(&m.male));
                    m.male_index = male_names.iter().position(|r| r == &m.male).unwrap();
                    assert!(!gone_male_names.contains(&m.male));
                    assert!(female_names.contains(&m.female));
                    m.female_index = female_names.iter().position(|r| r == &m.female).unwrap();
                    assert!(!gone_female_names.contains(&m.female));
                }
            }
            Event::NewParticipant { is_male, name, .. } => {
                if *is_male {
                    assert!(!male_names.contains(&name));
                    assert!(!gone_male_names.contains(&name));
                    male_names.push(name.clone());
                } else {
                    assert!(!female_names.contains(&name));
                    assert!(!gone_female_names.contains(&name));
                    female_names.push(name.clone());
                }
            }
        }
    }
}


pub fn build(path: &str) -> Config {
    let contents = read_to_string(path).unwrap();
    let mut config: Config = from_str(&contents).unwrap();
    validate_and_build_config(&mut config);
    config
}

use std::env;
use std::ops::IndexMut;
use std::time::Instant;

use array2d::Array2D;
use itertools::Itertools;

mod config;
mod match_box;
mod matching_night;
mod multiset_permutations;
mod new_participant;
mod permutations;

fn compute_matchings(mut config: config::Config) -> config::Config {
    let mut iterator: Box<dyn Iterator<Item = Array2D<bool>>> =
        permutations::iterate_matching_matricies(
            config.participants.males.len(),
            config.participants.females.len(),
        );

    let mut cur_perfect_matches: usize = 0;
    let mut males_gone: Vec<usize> = vec![];
    let mut females_gone: Vec<usize> = vec![];

    for event in config.events.iter_mut() {
        // Apply event filter / map
        iterator = match &event.kind {
            config::EventType::MatchBox {
                perfect_match,
                male_indicies,
                female_indicies,
                ..
            } => {
                if *perfect_match {
                    cur_perfect_matches += 1;
                    males_gone.extend(male_indicies.clone());
                    females_gone.extend(female_indicies.clone());
                }
                let filter_closure =
                    match_box::new_filter(*perfect_match, male_indicies, female_indicies);
                Box::new(iterator.filter(filter_closure))
            }
            config::EventType::MatchingNight {
                lights, matchings, ..
            } => {
                let filter_closure =
                    matching_night::new_filter(lights - cur_perfect_matches, matchings);
                Box::new(iterator.filter(filter_closure))
            }
            config::EventType::NewParticipant {
                is_male,
                is_duplicate,
                ..
            } => {
                let map_closure = new_participant::new_map(
                    *is_male,
                    *is_duplicate,
                    males_gone.clone(),
                    females_gone.clone(),
                );
                Box::new(iterator.flat_map(map_closure))
            }
            config::EventType::Start { .. } => iterator,
        };
        // Counting elements per stage
        iterator = Box::new(iterator.inspect(|m| {
            event.number_matchings += 1;
            for (index, b) in m.enumerate_row_major() {
                if *b {
                    *event.matching_distribution.0.index_mut(index) += 1;
                }
            }
        }));
    }
    iterator.count(); // Consume iterator
    config
}

fn print_overview(config: &config::Config) {
    println!("Result:");

    let mut i_match_box: usize = 0;
    let mut i_matching_night: usize = 0;
    for event in config.events.iter() {
        println!("{}", "-".repeat(40));
        match &event.kind {
            config::EventType::Start {} => println!("Start!"),
            config::EventType::MatchBox {
                males,
                females,
                perfect_match,
                ..
            } => {
                i_match_box += 1;
                println!(
                    "Match Box {}: {:?} {:?} ({})",
                    i_match_box,
                    males,
                    females,
                    if *perfect_match {
                        "Perfect Match"
                    } else {
                        "No Match"
                    }
                );
            }
            config::EventType::MatchingNight { lights, .. } => {
                i_matching_night += 1;
                println!("Matching Night {}: {} Lights", i_matching_night, *lights);
            }
            config::EventType::NewParticipant { name, .. } => println!("New Participant {}", name),
        }

        for (matches, male_name) in event
            .matching_distribution
            .0
            .rows_iter()
            .zip(&event.all_males)
        {
            println!("{}:", male_name);
            let decreasing_matches = matches
                .zip(&event.all_females)
                .sorted_by(|(v1, _), (v2, _)| (*v2).cmp(*v1));
            for (n, female_name) in decreasing_matches {
                let percentage = (100.0 * *n as f64) / event.number_matchings as f64;
                println!(" {:10} {:.1}%", female_name, percentage);
            }
        }
        println!(
            "Total number of possible matchings: {}",
            event.number_matchings
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args.get(1).expect("Missing config file path.");
    let config = config::build(file_path);

    println!(
        "Parsed AYTO {}Season {} Config",
        if config.meta.vip { "VIP " } else { "" },
        config.meta.season
    );

    let start = Instant::now();
    let config = compute_matchings(config);
    println!("Duration: {:?}", start.elapsed());
    print_overview(&config);
}

#[cfg(test)]
mod tests {
    use crate::{compute_matchings, config};

    #[test]
    fn test_s5() {
        let config = config::build("S5.toml");
        let totals: Vec<usize> = compute_matchings(config)
            .events
            .iter()
            .map(|s| s.number_matchings)
            .collect();
        assert_eq!(
            totals,
            vec![
                199584000, 179625600, 33752266, 27149771, 6755599, 6222309, 6222309, 5598916,
                83141, 51701, 19143, 291, 115
            ]
        );
    }

    #[test]
    fn test_s1() {
        let config = config::build("S1.toml");
        let totals: Vec<usize> = compute_matchings(config)
            .events
            .iter()
            .map(|s| s.number_matchings)
            .collect();
        assert_eq!(
            totals,
            vec![
                3628800, 3265920, 1201464, 1084655, 202933, 19012, 16682, 4720, 1696, 1432, 144,
                20, 140, 75, 63, 15, 10, 1, 1, 1, 1, 1
            ]
        );
    }
}

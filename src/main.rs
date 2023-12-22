use std::env;
use std::time::Instant;

mod config;
mod match_box;
mod matching_night;
mod new_participant;
mod permutations;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

fn get_matchings(config: &config::Config) -> Vec<usize> {
    let mut iterator = permutations::iterate_matching_matricies(
        config.participants.males.len(),
        config.participants.females.len(),
    );

    let mut cur_perfect_matches: usize = 0;
    let mut males_gone: Vec<usize> = vec![];
    let mut females_gone: Vec<usize> = vec![];

    let overview = Arc::new(Vec::from_iter(
        std::iter::repeat_with(|| AtomicU64::new(0)).take(config.events.len()),
    ));
    for (i, event) in config.events.iter().enumerate() {
        // Counting elements per stage
        let overview = overview.clone();
        iterator = Box::new(iterator.inspect(move |_| {
            overview[i].fetch_add(1, Ordering::Relaxed);
        }));

        // Apply event filter / map
        iterator = match event {
            config::Event::MatchBox {
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
            config::Event::MatchingNight { lights, matchings } => {
                let filter_closure =
                    matching_night::new_filter(lights - cur_perfect_matches, matchings);
                Box::new(iterator.filter(filter_closure))
            }
            config::Event::NewParticipant {
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
        }
    }

    let count = iterator.count();
    let mut overview: Vec<usize> = overview
        .iter()
        .map(|v| v.load(Ordering::Relaxed) as usize)
        .collect();
    overview.push(count);
    overview
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
    let overview = get_matchings(&config);
    println!("Number of Matchings: {overview:?}");
    println!("Duration: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use crate::{config, get_matchings};

    #[test]
    fn test_s5() {
        let config = config::build("S5.toml");
        let overview = get_matchings(&config);
        assert_eq!(
            overview,
            vec![
                199584000, 179625600, 33752266, 27149771, 6755599, 6222309, 6222309, 5598916,
                83141, 51701, 19143, 291, 115
            ]
        );
    }

    #[test]
    fn test_s1() {
        let config = config::build("S1.toml");
        let overview = get_matchings(&config);
        assert_eq!(
            overview,
            vec![
                3628800, 3265920, 1201464, 1084655, 202933, 19012, 16682, 4720, 1696, 1432, 144,
                20, 140, 75, 63, 15, 10, 1, 1, 1, 1, 1
            ]
        );
    }
}

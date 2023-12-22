use std::env;
use std::time::Instant;

mod config;
mod match_box;
mod matching_night;
mod new_participant;
mod permutations;

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

    let mut iterator = permutations::iterate_matching_matricies(
        config.participants.males.len(),
        config.participants.females.len(),
    );

    let mut cur_perfect_matches: usize = 0;
    let mut males_gone: Vec<usize> = vec![];
    let mut females_gone: Vec<usize> = vec![];

    for event in config.events.iter() {
        // println!("{event:?}");
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

    let n_matchings: usize = iterator.count();
    println!("Final Number of Matchings: {:?}", n_matchings);
    println!("Duration: {:?}", start.elapsed());
}

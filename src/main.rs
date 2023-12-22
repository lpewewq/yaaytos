use array2d::Array2D;
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

    let mut iter: Box<dyn Iterator<Item = Array2D<bool>>> =
        Box::new(permutations::iterate_matching_matricies(
            config.participants.males.len(),
            config.participants.females.len(),
        ));

    let mut cur_perfect_matches: usize = 0;
    let mut males_gone: Vec<usize> = vec![];
    let mut females_gone: Vec<usize> = vec![];

    for event in config.events.iter() {
        // println!("{event:?}");
        iter = match event {
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
                let filter_closure = match_box::new_filter(
                    *perfect_match,
                    male_indicies.to_vec(),
                    female_indicies.to_vec(),
                );
                Box::new(iter.filter(filter_closure))
            }
            config::Event::MatchingNight { lights, matchings } => {
                let filter_closure =
                    matching_night::new_filter(lights - cur_perfect_matches, matchings.to_vec());
                Box::new(iter.filter(filter_closure))
            }
            config::Event::NewParticipant {
                is_male,
                is_duplicate,
                ..
            } => {
                let map_closure = new_participant::new_map(
                    *is_male,
                    *is_duplicate,
                    males_gone.to_vec(),
                    females_gone.to_vec(),
                );
                Box::new(iter.flat_map(map_closure))
            }
        }
    }

    let n_matchings: usize = iter.count();
    println!("Final Number of Matchings: {:?}", n_matchings);
    println!("Duration: {:?}", start.elapsed());
}

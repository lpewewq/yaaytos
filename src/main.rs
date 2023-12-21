use array2d::Array2D;
use itertools::Itertools;
use serde::Deserialize;
use std::fs;
use std::time::Instant;
use toml::from_str;

fn match_box(
    matching_matrix: &Array2D<bool>,
    perfect_match: bool,
    male_idxs: &Vec<usize>,
    female_idxs: &Vec<usize>,
) -> bool {
    if perfect_match {
        for (i, res) in matching_matrix.row_iter(male_idxs[0]).unwrap().enumerate() {
            if female_idxs.contains(&i) != *res {
                return false;
            }
        }
        for (i, res) in matching_matrix
            .column_iter(female_idxs[0])
            .unwrap()
            .enumerate()
        {
            if male_idxs.contains(&i) != *res {
                return false;
            }
        }
        true
    } else {
        !matching_matrix[(male_idxs[0], female_idxs[0])]
    }
}

fn matching_night(
    matching_matrix: &Array2D<bool>,
    n_matches: usize,
    matching: &Vec<Match>,
) -> bool {
    let mut cur_matches = 0;
    for m in matching.iter() {
        if matching_matrix[(m.male_index, m.female_index)] {
            cur_matches += 1;
        }
    }
    cur_matches == n_matches
}

// fn new_participant(
//     matching_matrix: &Array2D<bool>, // n * n
//     is_male: bool,
// ) -> impl std::iter::Iterator<Item = Array2D<bool>> {
//     // add row
//     (0..matching_matrix.num_columns()).map(|i| {
//         let mut as_rows = matching_matrix.as_rows();
//         let mut new_row = vec![false; matching_matrix.num_columns()];
//         new_row[i] = true;
//         as_rows.push(new_row);
//         Array2D::from_rows(&as_rows).unwrap()
//     });

//     std::iter::from_fn(move || {
//         Array2D
//     })
// }

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(rename = "Meta")]
    meta: Meta,
    #[serde(rename = "Participants")]
    participants: Participants,
    #[serde(rename = "Events")]
    events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
struct Meta {
    season: u8,
    vip: bool,
}

#[derive(Debug, Deserialize)]
struct Participants {
    males: Vec<String>,
    females: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Match {
    male: String,
    female: String,
    #[serde(default)]
    male_index: usize,
    #[serde(default)]
    female_index: usize,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Event {
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
            Event::NewParticipant { is_male, name } => {
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

fn iterate_matching_matricies(n: usize) -> impl std::iter::Iterator<Item = Array2D<bool>> {
    let matching: Vec<usize> = (0..n).collect();
    let mut permutations = matching.into_iter().permutations(n);

    std::iter::from_fn(move || match permutations.next() {
        Some(permutation) => {
            let mut matching_matrix = Array2D::filled_with(false, n, n);
            for (male_idx, female_idx) in permutation.iter().enumerate() {
                matching_matrix[(male_idx, *female_idx)] = true;
            }
            Some(matching_matrix)
        }
        None => None,
    })
}

fn main() {
    let contents = fs::read_to_string("S1.toml").unwrap();
    let mut config: Config = from_str(&contents).unwrap();

    println!(
        "Parsed AYTO {}Season {} Config",
        if config.meta.vip { "VIP " } else { "" },
        config.meta.season
    );

    validate_and_build_config(&mut config);
    // println!("{:?}", config);

    assert_eq!(
        config.participants.males.len(),
        config.participants.females.len(),
        "Unequal participant number at start not yet implemented"
    );

    let start = Instant::now();
    let n: usize = config.participants.males.len();

    let mut iter: Box<dyn Iterator<Item = Array2D<bool>>> = Box::new(iterate_matching_matricies(n));

    let mut cur_perfect_matches: usize = 0;
    let mut n_match_box: usize = 0;
    let mut n_matching_night: usize = 0;
    let mut males_gone: Vec<usize> = vec![];
    let mut females_gone: Vec<usize> = vec![];
    for event in config.events {
        // iter = Box::new(iter.map(|m| {
        //     // TODO capture statistics
        //     m
        // }));
        match event {
            Event::MatchBox {
                males,
                females,
                perfect_match,
                male_indicies,
                female_indicies,
            } => {
                println!(
                    "Match Box {}: Perfect {} {:?} {:?}",
                    n_match_box, perfect_match, males, females
                );
                if perfect_match {
                    cur_perfect_matches += 1;
                    males_gone.extend(male_indicies.clone());
                    females_gone.extend(female_indicies.clone());
                }
                iter = Box::new(iter.filter(move |m| {
                    match_box(&m, perfect_match, &male_indicies, &female_indicies)
                }));
                n_match_box += 1;
            }
            Event::MatchingNight { lights, matchings } => {
                println!("Matching Night {}: Lights {}", n_matching_night, lights);
                iter =
                    Box::new(iter.filter(move |m| {
                        matching_night(&m, lights - cur_perfect_matches, &matchings)
                    }));
                n_matching_night += 1;
            }
            Event::NewParticipant { is_male, name } => {
                println!("New Participant {}", name);
                if is_male {
                    // let x = females_gone.clone();
                    // let filter_closure = move |i| !x.contains(i);
                    iter = Box::new(iter.flat_map(|m| {
                        (0..m.num_columns())
                            // .filter(filter_closure)
                            .map(move |i| {
                                let mut as_rows = m.as_rows();
                                let mut new_row = vec![false; m.num_columns()];
                                new_row[i] = true;
                                as_rows.push(new_row);
                                Array2D::from_rows(&as_rows).unwrap()
                            })
                    }))
                } else {
                    iter = Box::new(iter.flat_map(|m| {
                        (0..m.num_rows())
                            // .filter(|i| !males_gone.contains(i))
                            .map(move |i| {
                                let mut as_columns = m.as_columns();
                                let mut new_column = vec![false; m.num_rows()];
                                new_column[i] = true;
                                as_columns.push(new_column);
                                Array2D::from_columns(&as_columns).unwrap()
                            })
                    }))
                }
            }
        }
    }

    let mut n_matchings: usize = 0;
    for _ in iter {
        n_matchings += 1;
    }

    println!("Final Number of Matchings: {:?}", n_matchings);
    println!("Duration: {:?}", start.elapsed());
}

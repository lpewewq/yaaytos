use array2d::Array2D;
use indicatif::ProgressBar;
use itertools::Itertools;
use number_encoding::{factorial, multinomial};
use std::fs;
use std::time::Instant;
use toml::from_str;

mod config;

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
    matching: &Vec<config::Match>,
) -> bool {
    let mut cur_matches = 0;
    for m in matching.iter() {
        if matching_matrix[(m.male_index, m.female_index)] {
            cur_matches += 1;
        }
    }
    cur_matches == n_matches
}

fn iterate_matching_matricies(
    n_males: usize,
    n_females: usize,
) -> Box<dyn std::iter::Iterator<Item = Array2D<bool>>> {
    let n_min = n_males.min(n_females);
    let n_max = n_males.max(n_females);
    let n_diff = n_males.abs_diff(n_females);

    if n_diff > 0 {
        let mut multisetpermutations = (0..n_diff)
            .map(|_| 0..n_min)
            .multi_cartesian_product()
            .map(move |mprod| {
                let mut x = mprod.clone();
                x.extend(0..n_min);
                x
            })
            .flat_map(move |matching| matching.into_iter().permutations(n_max))
            .unique(); // TODO remove unique

        let n_multisetpermutations: usize = (0..n_diff)
            .map(|_| 0..n_min)
            .multi_cartesian_product()
            .map(move |mprod| {
                let mut x = mprod.clone();
                x.extend(0..n_min);
                multinomial(&x)
            })
            .sum();
        let bar = ProgressBar::new(n_multisetpermutations as u64);

        Box::new(std::iter::from_fn(move || {
            match multisetpermutations.next() {
                Some(permutation) => {
                    bar.inc(1);
                    let mut matching_matrix = Array2D::filled_with(false, n_males, n_females);
                    for (idx1, idx2) in permutation.iter().enumerate() {
                        if n_males < n_females {
                            matching_matrix[(*idx2, idx1)] = true;
                        } else {
                            matching_matrix[(idx1, *idx2)] = true;
                        }
                    }
                    Some(matching_matrix)
                }
                None => None,
            }
        }))
    } else {
        let mut permutations = (0..n_min).permutations(n_min);

        let n_permutations = factorial(n_min);
        let bar = ProgressBar::new(n_permutations as u64);

        Box::new(std::iter::from_fn(move || match permutations.next() {
            Some(permutation) => {
                bar.inc(1);
                let mut matching_matrix = Array2D::filled_with(false, n_males, n_females);
                for (male_idx, female_idx) in permutation.iter().enumerate() {
                    matching_matrix[(male_idx, *female_idx)] = true;
                }
                Some(matching_matrix)
            }
            None => None,
        }))
    }
}

fn main() {
    let contents = fs::read_to_string("S1.toml").unwrap();
    let mut config: config::Config = from_str(&contents).unwrap();

    println!(
        "Parsed AYTO {}Season {} Config",
        if config.meta.vip { "VIP " } else { "" },
        config.meta.season
    );

    config::validate_and_build_config(&mut config);
    // println!("{:?}", config);

    let start = Instant::now();

    let mut iter: Box<dyn Iterator<Item = Array2D<bool>>> = Box::new(iterate_matching_matricies(
        config.participants.males.len(),
        config.participants.females.len(),
    ));

    let mut cur_perfect_matches: usize = 0;
    let mut n_match_box: usize = 0;
    let mut n_matching_night: usize = 0;
    let mut males_gone: Vec<usize> = vec![];
    let mut females_gone: Vec<usize> = vec![];
    // let mut stats: Vec<usize> = vec![0, config.events.len()];
    for event in config.events.iter() {
        match event {
            config::Event::MatchBox {
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
                if *perfect_match {
                    cur_perfect_matches += 1;
                    males_gone.extend(male_indicies.clone());
                    females_gone.extend(female_indicies.clone());
                }
                iter = Box::new(iter.filter(move |m| {
                    match_box(&m, *perfect_match, &male_indicies, &female_indicies)
                }));
                n_match_box += 1;
            }
            config::Event::MatchingNight { lights, matchings } => {
                println!("Matching Night {}: Lights {}", n_matching_night, lights);
                iter =
                    Box::new(iter.filter(move |m| {
                        matching_night(&m, lights - cur_perfect_matches, &matchings)
                    }));
                n_matching_night += 1;
            }
            config::Event::NewParticipant {
                is_male,
                name,
                is_duplicate,
            } => {
                println!("New Participant {}", name);
                let curr_females_gone = females_gone.clone();
                if *is_male {
                    iter = Box::new(iter.flat_map(move |m| {
                        let curr_females_gone2 = curr_females_gone.clone();

                        (0..m.num_columns()).filter_map(move |i| {
                            if curr_females_gone2.contains(&i) {
                                return None;
                            }

                            if *is_duplicate {
                                let sum: usize = m
                                    .column_iter(i)
                                    .unwrap()
                                    .map(|&b| if b { 1 } else { 0 })
                                    .sum();
                                if sum != 2 {
                                    // Hardcoded for S5 Melanie
                                    return None;
                                }
                            }

                            let mut as_rows = m.as_rows();
                            let mut new_row = vec![false; m.num_columns()];
                            new_row[i] = true;
                            as_rows.push(new_row);
                            Some(Array2D::from_rows(&as_rows).unwrap())
                        })
                    }))
                } else {
                    let curr_males_gone = males_gone.clone();
                    iter = Box::new(iter.flat_map(move |m| {
                        let curr_males_gone2 = curr_males_gone.clone();

                        (0..m.num_rows()).filter_map(move |i| {
                            if curr_males_gone2.contains(&i) {
                                return None;
                            }

                            if *is_duplicate {
                                let sum: usize =
                                    m.row_iter(i).unwrap().map(|&b| if b { 1 } else { 0 }).sum();
                                if sum != 2 {
                                    // Hardcoded for S5 Melanie
                                    return None;
                                }
                            }

                            let mut as_columns = m.as_columns();
                            let mut new_column = vec![false; m.num_rows()];
                            new_column[i] = true;
                            as_columns.push(new_column);
                            Some(Array2D::from_columns(&as_columns).unwrap())
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

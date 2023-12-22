use array2d::Array2D;

use crate::config;

fn matching_night(
    matching_matrix: &Array2D<bool>,
    n_matches: usize,
    matching: &[config::Match],
) -> bool {
    let mut cur_matches = 0;
    for m in matching.iter() {
        if matching_matrix[(m.male_index, m.female_index)] {
            cur_matches += 1;
        }
    }
    cur_matches == n_matches
}

pub fn new_filter<'a>(
    n_matches: usize,
    matching: &'a [config::Match],
) -> impl FnMut(&Array2D<bool>) -> bool + 'a {
    move |m: &Array2D<bool>| matching_night(&m, n_matches, matching)
}

use crate::multiset_permutations::MultisetPermutations;
use array2d::Array2D;
use indicatif::ProgressBar;
use itertools::Itertools;
use number_encoding::{factorial, multinomial};

pub fn iterate_matching_matricies(
    n_males: usize,
    n_females: usize,
) -> Box<dyn std::iter::Iterator<Item = Array2D<bool>>> {
    let n_min = n_males.min(n_females);
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
            .flat_map(|matching| MultisetPermutations::new(&matching));

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

use array2d::Array2D;
use indicatif::ProgressBar;
use itertools::Itertools;
use number_encoding::{factorial, multinomial};
use std::cmp::Reverse;
pub struct MSP<I> {
    vec: Vec<I>,
    first_next: bool,
}

impl<I: Copy + Ord> MSP<I> {
    pub fn new(vec: &Vec<I>) -> MSP<I> {
        let mut sorted_vec = vec.clone();
        sorted_vec.sort_unstable_by_key(|w| Reverse(*w));
        MSP {
            vec: sorted_vec,
            first_next: true,
        }
    }
}

impl<I: Clone + std::cmp::PartialOrd> Iterator for MSP<I> {
    type Item = Vec<I>;

    // Williams, Aaron. "Loopless generation of multiset permutations using a constant number of variables by prefix shifts."
    // https://epubs.siam.org/doi/abs/10.1137/1.9781611973068.107

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_next {
            self.first_next = false;
            let result = Some(self.vec.clone());
            let shift_element = self.vec.remove(self.vec.len() - 1);
            self.vec.insert(0, shift_element);
            result
        } else {
            let i_option = self.vec.iter().tuple_windows().position(|(x, y)| x < y);
            match i_option {
                Some(i) => {
                    let shift_index = if i + 2 < self.vec.len() {
                        if self.vec[i] < self.vec[i + 2] {
                            i + 1
                        } else {
                            i + 2
                        }
                    } else {
                        self.vec.len() - 1
                    };
                    let result = Some(self.vec.clone());
                    let shift_element = self.vec.remove(shift_index);
                    self.vec.insert(0, shift_element);
                    result
                }
                None => None,
            }
        }
    }
}

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
            .flat_map(|matching| MSP::new(&matching));

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

use itertools::Itertools;
use std::cmp::Reverse;

// Cool-lex ordered multiset permutations
// Williams, Aaron. "Loopless generation of multiset permutations using a constant number of variables by prefix shifts."
// https://epubs.siam.org/doi/abs/10.1137/1.9781611973068.107

pub struct MultisetPermutations<I> {
    vec: Vec<I>,
    first_next: bool,
}

impl<I: Copy + Ord> MultisetPermutations<I> {
    pub fn new(vec: &Vec<I>) -> MultisetPermutations<I> {
        let mut sorted_vec = vec.clone();
        sorted_vec.sort_unstable_by_key(|w| Reverse(*w));
        MultisetPermutations {
            vec: sorted_vec,
            first_next: true,
        }
    }
}

impl<I: Clone + std::cmp::PartialOrd> Iterator for MultisetPermutations<I> {
    type Item = Vec<I>;

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

#[cfg(test)]
mod tests {
    use crate::multiset_permutations::MultisetPermutations;

    #[test]
    fn test1() {
        let vec = vec![1, 4, 2, 1];
        let mut iter = MultisetPermutations::new(&vec);
        assert_eq!(iter.next(), Some(vec![4, 2, 1, 1]));
        assert_eq!(iter.next(), Some(vec![1, 4, 2, 1]));
        assert_eq!(iter.next(), Some(vec![4, 1, 2, 1]));
        assert_eq!(iter.next(), Some(vec![1, 4, 1, 2]));
        assert_eq!(iter.next(), Some(vec![1, 1, 4, 2]));
        assert_eq!(iter.next(), Some(vec![4, 1, 1, 2]));
        assert_eq!(iter.next(), Some(vec![2, 4, 1, 1]));
        assert_eq!(iter.next(), Some(vec![1, 2, 4, 1]));
        assert_eq!(iter.next(), Some(vec![2, 1, 4, 1]));
        assert_eq!(iter.next(), Some(vec![1, 2, 1, 4]));
        assert_eq!(iter.next(), Some(vec![1, 1, 2, 4]));
        assert_eq!(iter.next(), Some(vec![2, 1, 1, 4]));
        assert_eq!(iter.next(), None);
    }
}

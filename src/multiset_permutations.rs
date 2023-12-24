use std::cmp::Reverse;

// Cool-lex ordered multiset permutations
// Williams, Aaron. "Loopless generation of multiset permutations using a constant number of variables by prefix shifts."
// https://epubs.siam.org/doi/abs/10.1137/1.9781611973068.107

pub struct MultisetPermutations<I> {
    vec: Vec<I>,
    first_next: bool,
    current_index: usize,
}

impl<I: Copy + Ord> MultisetPermutations<I> {
    pub fn new(vec: &Vec<I>) -> MultisetPermutations<I> {
        let mut sorted_vec = vec.clone();
        sorted_vec.sort_unstable_by_key(|w| Reverse(*w));
        MultisetPermutations {
            vec: sorted_vec,
            first_next: true,
            current_index: vec.len().saturating_sub(2),
        }
    }
}

impl<I: Clone + std::cmp::PartialOrd> Iterator for MultisetPermutations<I> {
    type Item = Vec<I>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_next {
            self.first_next = false;
            return Some(self.vec.clone());
        }

        if self.current_index + 2 < self.vec.len()
            || (self.current_index + 1 < self.vec.len()
                && self.vec[self.current_index + 1] < self.vec[0])
        {
            let shift_index = if self.current_index + 2 < self.vec.len()
                && self.vec[self.current_index + 2] <= self.vec[self.current_index]
            {
                self.current_index + 2
            } else {
                self.current_index + 1
            };
            let shift_element = self.vec.remove(shift_index);
            self.vec.insert(0, shift_element);
            if self.vec[0] < self.vec[1] {
                self.current_index = 0;
            } else {
                self.current_index += 1;
            }
            Some(self.vec.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::multiset_permutations::MultisetPermutations;
    use std::time::Instant;

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
    #[test]
    fn test2() {
        let vec = vec![0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let iter = MultisetPermutations::new(&vec);
        let now = Instant::now();
        let count = iter.count(); // ~2.22s
        println!("Number of permutations {} in {:?}", count, now.elapsed());
        assert_eq!(count, 19958400);
    }
    #[test]
    fn test3() {
        let vec = vec![0];
        let mut iter = MultisetPermutations::new(&vec);
        assert_eq!(iter.next(), Some(vec![0]));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test4() {
        let vec = vec![0, 1];
        let mut iter = MultisetPermutations::new(&vec);
        assert_eq!(iter.next(), Some(vec![1, 0]));
        assert_eq!(iter.next(), Some(vec![0, 1]));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test5() {
        let vec = vec![0, 0];
        let mut iter = MultisetPermutations::new(&vec);
        assert_eq!(iter.next(), Some(vec![0, 0]));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test6() {
        let vec = vec![0, 0, 1];
        let mut iter = MultisetPermutations::new(&vec);
        assert_eq!(iter.next(), Some(vec![1, 0, 0]));
        assert_eq!(iter.next(), Some(vec![0, 1, 0]));
        assert_eq!(iter.next(), Some(vec![0, 0, 1]));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test7() {
        let vec: Vec<i32> = vec![];
        let mut iter = MultisetPermutations::new(&vec);
        assert_eq!(iter.next(), Some(vec![]));
        assert_eq!(iter.next(), None);
    }
}

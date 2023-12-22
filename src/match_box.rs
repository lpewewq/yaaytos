use array2d::Array2D;

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

pub fn new_filter(
    perfect_match: bool,
    male_idxs: Vec<usize>,
    female_idxs: Vec<usize>,
) -> impl FnMut(&Array2D<bool>) -> bool {
    move |m: &Array2D<bool>| match_box(&m, perfect_match, &male_idxs, &female_idxs)
}

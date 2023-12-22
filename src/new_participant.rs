use array2d::Array2D;

pub fn new_map(
    is_male: bool,
    is_duplicate: bool,
    males_gone: Vec<usize>,
    females_gone: Vec<usize>,
) -> impl FnMut(Array2D<bool>) -> Vec<Array2D<bool>> {
    move |m: Array2D<bool>| {
        let males_gone = males_gone.clone();
        let females_gone = females_gone.clone();
        if is_male {
            (0..m.num_columns())
                .filter_map(move |i| {
                    if females_gone.contains(&i) {
                        return None;
                    }
                    if is_duplicate {
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
                .collect()
        } else {
            (0..m.num_rows())
                .filter_map(move |i| {
                    if males_gone.contains(&i) {
                        return None;
                    }
                    if is_duplicate {
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
                .collect()
        }
    }
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut row_max = vec![Vec::new(); input.len()];
    let mut col_min: Vec<Vec<(usize, u64)>> = vec![Vec::new(); input[0].len()];

    for (row, values) in input.iter().enumerate() {
        for (col, &value) in values.iter().enumerate() {
            let mut c = &mut col_min[col];

            if c.is_empty() || c[0].1 == value {
                c.push((row, value));
            } else if c[0].1 > value {
                c.clear();
                c.push((row, value));
            }

            let mut r = &mut row_max[row];

            if r.is_empty() || r[0] == value {
                r.push(value);
            } else if r[0] < value {
                r.clear();
                r.push(value);
            }
        }
    }

    col_min
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut points, (col, values)| {
            let mut col_points = values
                .iter()
                .filter(|c| row_max[c.0].contains(&c.1))
                .map(|c| (c.0, col))
                .collect();

            points.append(&mut col_points);
            points
        })
}

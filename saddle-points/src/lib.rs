pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut row_max = vec![None; input.len()];
    let mut col_min = vec![(None, Vec::new()); input[0].len()];

    for (row, values) in input.iter().enumerate() {
        for (col, &value) in values.iter().enumerate() {
            if row_max[row].filter(|&v| v >= value).is_none() {
                row_max[row] = Some(value);
            }

            let mut c = &mut col_min[col];

            if c.0.filter(|&v| v <= value).is_none() {
                c.1.clear();
            }

            if c.0.filter(|&v| v < value).is_none() {
                c.0 = Some(value);
                c.1.push(row);
            }
        }
    }

    col_min
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut points, (col, (value, rows))| {
            let mut col_points = rows
                .iter()
                .filter(|&&row| row_max[row] == *value)
                .map(|&row| (row, col))
                .collect();

            points.append(&mut col_points);
            points
        })
}

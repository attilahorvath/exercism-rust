pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = Vec::<Vec<u32>>::with_capacity(self.row_count as usize);

        for row_index in 0..self.row_count {
            let mut row = Vec::<u32>::with_capacity((row_index + 1) as usize);

            for i in 0..(row_index + 1) {
                if i == 0 || i == row_index {
                    row.push(1);
                    continue;
                }

                let previous_row = (row_index - 1) as usize;

                row.push(
                    rows[previous_row][(i - 1) as usize] + rows[previous_row][i as usize],
                );
            }

            rows.push(row);
        }

        rows
    }
}

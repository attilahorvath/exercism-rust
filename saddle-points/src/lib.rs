fn is_saddle_point(input: &[Vec<u64>], px: usize, py: usize) -> bool {
    let value = input[py][px];

    for y in 0..input.len() {
        if input[y][px] < value {
            return false;
        }
    }

    for x in 0..input[0].len() {
        if input[py][x] > value {
            return false;
        }
    }

    true
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if is_saddle_point(input, x, y) {
                saddle_points.push((y, x));
            }
        }
    }

    saddle_points
}

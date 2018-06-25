enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

struct SpiralMatrix {
    size: usize,
    matrix: Vec<Vec<usize>>,
}

impl SpiralMatrix {
    fn new(size: usize) -> Self {
        SpiralMatrix {
            size,
            matrix: (0..size).map(|_| (0..size).map(|_| 0).collect()).collect(),
        }
    }

    fn set(&mut self, coords: (usize, usize), value: usize) {
        self.matrix[coords.1][coords.0] = value;
    }

    fn is_empty(&self, coords: (usize, usize)) -> bool {
        self.matrix[coords.1][coords.0] == 0
    }
}

impl Into<Vec<Vec<usize>>> for SpiralMatrix {
    fn into(self) -> Vec<Vec<usize>> {
        self.matrix
    }
}

struct MatrixPosition {
    coords: (usize, usize),
}

impl MatrixPosition {
    fn new() -> Self {
        MatrixPosition { coords: (0, 0) }
    }

    fn step(&self, matrix: &SpiralMatrix, direction: &Direction) -> Option<Self> {
        let coords = match direction {
            Direction::Up => if self.coords.1 > 0 {
                (self.coords.0, self.coords.1 - 1)
            } else {
                return None;
            },
            Direction::Down => if self.coords.1 < matrix.size - 1 {
                (self.coords.0, self.coords.1 + 1)
            } else {
                return None;
            },
            Direction::Left => if self.coords.0 > 0 {
                (self.coords.0 - 1, self.coords.1)
            } else {
                return None;
            },
            Direction::Right => if self.coords.0 < matrix.size - 1 {
                (self.coords.0 + 1, self.coords.1)
            } else {
                return None;
            },
        };

        if matrix.is_empty(coords) {
            Some(Self { coords })
        } else {
            None
        }
    }
}

pub fn spiral_matrix(size: usize) -> Vec<Vec<usize>> {
    let mut matrix = SpiralMatrix::new(size);

    let mut position = MatrixPosition::new();
    let mut direction = Direction::Right;

    for i in 1..(size.pow(2) + 1) {
        matrix.set(position.coords, i);

        position = if let Some(next_position) = position.step(&matrix, &direction) {
            next_position
        } else {
            direction = direction.next();
            position.step(&matrix, &direction).unwrap_or(position)
        }
    }

    matrix.into()
}

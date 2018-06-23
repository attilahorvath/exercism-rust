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

    fn set(&mut self, x: usize, y: usize, value: usize) {
        self.matrix[y][x] = value;
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        self.matrix[y][x] == 0
    }
}

impl Into<Vec<Vec<usize>>> for SpiralMatrix {
    fn into(self) -> Vec<Vec<usize>> {
        self.matrix
    }
}

struct MatrixPosition {
    x: usize,
    y: usize,
}

impl MatrixPosition {
    fn new() -> Self {
        MatrixPosition { x: 0, y: 0 }
    }

    fn step(&mut self, matrix: &SpiralMatrix, direction: &Direction) -> bool {
        match direction {
            Direction::Up => if self.y > 0 && matrix.is_empty(self.x, self.y - 1) {
                self.y -= 1;
                true
            } else {
                false
            },
            Direction::Down => if self.y < matrix.size - 1 && matrix.is_empty(self.x, self.y + 1) {
                self.y += 1;
                true
            } else {
                false
            },
            Direction::Left => if self.x > 0 && matrix.is_empty(self.x - 1, self.y) {
                self.x -= 1;
                true
            } else {
                false
            },
            Direction::Right => {
                if self.x < matrix.size - 1 && matrix.is_empty(self.x + 1, self.y) {
                    self.x += 1;
                    true
                } else {
                    false
                }
            }
        }
    }
}

pub fn spiral_matrix(size: usize) -> Vec<Vec<usize>> {
    let mut matrix = SpiralMatrix::new(size);

    let mut position = MatrixPosition::new();
    let mut direction = Direction::Right;

    for i in 1..(size.pow(2) + 1) {
        matrix.set(position.x, position.y, i);

        if !position.step(&matrix, &direction) {
            direction = direction.next();
            position.step(&matrix, &direction);
        }
    }

    matrix.into()
}

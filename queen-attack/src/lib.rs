pub struct ChessPosition {
    row: i32,
    column: i32,
}

impl ChessPosition {
    pub fn new(row: i32, column: i32) -> Result<Self, &'static str> {
        if row < 0 || row > 7 || column < 0 || column > 7 {
            return Err("Invalid position on the chess board");
        }

        Ok(ChessPosition {
            row: row,
            column: column,
        })
    }
}

pub struct Queen {
    position: ChessPosition,
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position: position }
    }

    pub fn can_attack(&self, other: &Self) -> bool {
        self.position.row == other.position.row || self.position.column == other.position.column ||
            (self.position.row - other.position.row).abs() ==
                (self.position.column - other.position.column).abs()
    }
}

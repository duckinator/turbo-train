use super::GRID_WIDTH;
use super::PIECE_WIDTH;

#[derive(Copy, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn down(&self) -> Self {
        let mut new = self.clone();
        new.row += 1;
        new
    }

    pub fn left(&self) -> Self {
        let mut new = self.clone();
        new.col -= 1;
        new
    }

    pub fn right(&self) -> Self {
        let mut new = self.clone();
        new.col += 1;
        new
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { col: (GRID_WIDTH - PIECE_WIDTH) / 2, row: 0 }
    }
}

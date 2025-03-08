use super::*;

#[derive(Debug, PartialEq)]
pub enum Action {
    None,
    RotateLeft,
    RotateRight,
    DropSoft,
    DropHard,
    DumpLine,
}

type GridLine = [u8; GRID_WIDTH];
type Grid = [GridLine; GRID_HEIGHT];
#[derive(Clone, Resource)]
pub struct Game {
    grid: Grid,
    current_landed: bool,
    current: Piece,
    position: Position,
}

impl Game {
    pub fn stuck(&mut self) -> bool {
        // If we can't drop into the game area, we're done.
        self.position.row < GRID_FIRST_VISIBLE && self.collides(self.position.clone().down())
    }

    pub fn tick(&mut self, action: Action) -> Option<(Piece, Position)> {
        if self.stuck() {
            return None;
        }

        if self.current_landed {
            self.new_piece();
        }

        match action {
            Action::None => {},
            Action::RotateLeft => {
                self.current.rotate_left();
                return None;
            },
            Action::RotateRight => {
                self.current.rotate_right();
                return None;
            },
            Action::DropSoft => {},
            Action::DropHard => {
                while !self.current_landed {
                    self.tick(Action::None);
                }
            },
            Action::DumpLine => {
                self.dump_line();
                return None;
            },
        }

        let pos = self.position.clone();


        if pos.row == (GRID_HEIGHT - 1) || self.collides(pos.down()) {
            return Some(self.place());
        } else {
            self.position = pos.down();
        }

        None
    }

    pub fn collides(&self, position: Position) -> bool {
        let piece = self.current.blocks;
        let grid = &self.grid;

        let mut piece_height = 0;
        for row in 0..piece.len() {
            for col in 0..piece[row].len() {
                if piece[row][col] == 1 {
                    piece_height += 1;
                    break;
                }
            }
        }

        let mut piece_width = 0;
        for row in 0..piece.len() {
            for col in 0..piece[row].len() {
                if piece[row][col] == 1 && col > piece_width {
                    piece_width = col;
                }
            }
        }

        let x = position.col;
        let y = position.row;

        let y_upper_bound = std::cmp::min(y + piece_height, GRID_HEIGHT);
        let x_upper_bound = std::cmp::min(x + piece_width, GRID_WIDTH);

        let area: Vec<Vec<u8>> = grid[y..y_upper_bound].iter().map(|r| r[x..x_upper_bound].into()).collect();
        let v_piece: Vec<Vec<u8>> = piece.clone().map(|r| r.into()).into();

        let comparison: Vec<u8> = area
            .iter()
            .zip(v_piece)
            .map(|(a, b)| a.into_iter().zip(b).map(|(l, r)| l*r).collect::<Vec<_>>().into_iter().sum())
            .collect();
        let total: u8 = comparison.iter().sum();

        total > 0
    }

    fn place(&mut self) -> (Piece, Position) {
        let piece = &self.current;

        if self.collides(self.position) {
            //panic!("called grid.place(piece) with a piece that collides?");
        }

        let row_offset = self.position.row;
        let col_offset = self.position.col;
        for row in 0..piece.blocks.len() {
            if row + row_offset >= GRID_HEIGHT {
                break;
            }

            for col in 0..piece.blocks[row].len() {
                let value = piece.blocks[row][col];
                if value != 0 {
                    self.grid[row + row_offset][col + col_offset] = value;
                }
            }
        }

        let result = (piece.clone(), self.position.clone());

        self.clear_piece();

        result
    }

    fn dump_line(&mut self) {
        let cols = self.grid[0].len();
        for row in (1..self.grid.len()).rev() {
            for col in 0..cols {
                self.grid[row - 1][col] = self.grid[row][col];
            }
        }

        for col in 0..cols {
            self.grid[self.grid.len() - 1][col] = 0;
        }
    }

    fn clear_piece(&mut self) {
        self.current_landed = true;
    }

    fn new_piece(&mut self) {
        self.current = Piece::next();
        self.position = Position::default();
        self.current_landed = false;
    }

    pub fn compare(&self, expected: Grid) -> bool {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                if self.grid[row][col] != expected[row][col] {
                    return false;
                }
            }
        }

        true
    }

    pub fn print(&self) {
        let mut clone: Game = self.clone();
        if !self.current_landed {
            clone.place();
        }

        for idx in 0..clone.grid.len() {
            let row = clone.grid[idx];

            print!("| ");
            for col in row {
                print!("{} ", col);
            }
            println!("|");

            if idx == (PIECE_HEIGHT - 1) {
                println!("|={}|", "=".repeat(GRID_WIDTH * 2));
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            grid: [[0; GRID_WIDTH]; GRID_HEIGHT],
            current_landed: false,
            current: Piece::next(),
            position: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_natural_drop() {
        let mut game = Game::default();

        let mut expected: Grid = Default::default();
        let row = GRID_HEIGHT - 1;
        let start_col = game.position.col;

        // First piece.
        game.current = LINE;
        game.position = Default::default();
        for i in 0..GRID_HEIGHT {
            game.tick(Action::None);
        }

        for col in start_col..(start_col + 4) {
            expected[row][col] = 1;
        }

        assert_eq!(game.grid, expected);

        // Second piece.
        game.current = LINE;
        game.position = Default::default();
        for i in 0..GRID_HEIGHT {
            game.tick(Action::None);
        }

        let row = row - 1;
        for col in start_col..(start_col + 4) {
            expected[row][col] = 1;
        }

        assert_eq!(game.grid, expected);
    }

    #[test]
    fn test_hard_drop() {
        let mut game = Game::default();

        let mut expected: Grid = Default::default();
        let row = GRID_HEIGHT - 1;
        let start_col = game.position.col;

        game.current = LINE;
        game.position = Default::default();
        game.tick(Action::DropHard);

        for col in start_col..(start_col + 4) {
            expected[row][col] = 1;
        }

        assert_eq!(game.grid, expected);
    }

    #[test]
    fn test_dump_line() {
        let mut game = Game::default();
        for col in 0..GRID_WIDTH {
            game.grid[GRID_HEIGHT - 1][col] = 1;
        }

        assert_eq!(game.grid[GRID_HEIGHT - 1], [1; GRID_WIDTH]);

        game.tick(Action::DumpLine);

        assert_eq!(game.grid[GRID_HEIGHT - 1], [0; GRID_WIDTH]);
    }

    #[test]
    fn test_compare() {
        let mut game = Game::default();
        for col in 0..GRID_WIDTH {
            game.grid[GRID_HEIGHT - 1][col] = 1;
        }

        let mut expected: Grid = Default::default();
        for col in 0..GRID_WIDTH {
            expected[GRID_HEIGHT - 1][col] = 1;
        }

        assert_eq!(game.compare(expected), true);
    }
}

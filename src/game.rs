use super::*;

#[derive(Debug, PartialEq)]
pub enum Action {
    None,
    RotateLeft,
    RotateRight,
    DropSoft,
    DropHard,
}

type GridLine = [u8; GRID_WIDTH];
#[derive(Clone)]
pub struct Game {
    grid: [GridLine; GRID_HEIGHT],
    current: Option<Piece>,
    position: Position,
}

impl Game {
    pub fn tick(&mut self, action: Action) -> bool {
        if action != Action::None {
            println!("!!! ignoring action: {:?} !!!", action);
        }

        let pos = self.position.clone();

        if pos.row < GRID_FIRST_VISIBLE && self.collides(pos.down()) {
            // If we can't drop into the game area, we're done.
            return false;
        }

        if pos.row == (GRID_HEIGHT - 1) || self.collides(pos.down()) {
            self.place();
        } else {
            self.position = pos.down();
        }

        // By default, continue.
        true
    }

    pub fn collides(&self, position: Position) -> bool {
        let Some(current) = &self.current else { return false };

        let piece = current.blocks;
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
        /*let piece_height = piece.len();
        let piece_width = piece[0].len();*/

        let x = position.col;
        let y = position.row;

        let area: Vec<Vec<u8>> = grid[y..(y+piece_height)].iter().map(|r| r[x..(x+piece_width)].into()).collect();
        let v_piece: Vec<Vec<u8>> = piece.clone().map(|r| r.into()).into();

        let comparison: Vec<u8> = area
            .iter()
            .zip(v_piece)
            .map(|(a, b)| a.into_iter().zip(b).map(|(l, r)| l*r).collect::<Vec<_>>().into_iter().sum())
            .collect();
        let total: u8 = comparison.iter().sum();

        total > 0
    }

    fn place(&mut self) {
        let Some(piece) = &self.current else { return };

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

        self.new_piece();
    }

    fn new_piece(&mut self) {
        self.current = Some(Piece::next());
        self.position = Position::default();
    }

    pub fn print(&self) {
        let mut clone: Game = self.clone();
        clone.place();

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
            current: Some(Piece::next()),
            position: Default::default(),
        }
    }
}

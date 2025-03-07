use super::PIECE_WIDTH;
use super::PIECE_HEIGHT;

type PieceLine = [u8; PIECE_WIDTH];
#[derive(Clone)]
pub struct Piece {
    pub blocks: [PieceLine; PIECE_HEIGHT],
}

impl Piece {
    pub fn next() -> Self {
        LINE
    }

    pub fn rotate_left(&mut self) {
    }

    pub fn rotate_right(&mut self) {
    }
}

pub const LINE: Piece = Piece {
    blocks: [
        [1,1,1,1],
        [0,0,0,0],
        [0,0,0,0],
        [0,0,0,0],
    ],
};

pub const SQUARE: Piece = Piece {
    blocks: [
        [0,1,1,0],
        [0,1,1,0],
        [0,0,0,0],
        [0,0,0,0],
    ],
};

pub const ELL: Piece = Piece {
    blocks: [
        [1,1,1,0],
        [1,0,0,0],
        [0,0,0,0],
        [0,0,0,0],
    ],
};

pub const ESS: Piece = Piece {
    blocks: [
        [0,1,1,0],
        [1,1,0,0],
        [0,0,0,0],
        [0,0,0,0],
    ],
};

pub const TEE: Piece = Piece {
    blocks: [
        [1,1,1,0],
        [0,1,0,0],
        [0,0,0,0],
        [0,0,0,0],
    ],
};

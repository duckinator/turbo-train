use super::PIECE_HEIGHT;
use super::PIECE_WIDTH;
use super::color::PieceColor;
use rand::prelude::IndexedRandom;

type PieceLine = [u8; PIECE_WIDTH];
#[derive(Clone, Debug)]
pub struct Piece {
    pub blocks: [PieceLine; PIECE_HEIGHT],
    pub color: PieceColor,
}

impl Piece {
    pub fn next() -> Self {
        [LINE, SQUARE, ELL, ESS, TEE]
            .choose(&mut rand::rng())
            .unwrap()
            .clone()
    }

    pub fn width(&self) -> usize {
        self.blocks[0].len()
    }

    pub fn height(&self) -> usize {
        self.blocks.len()
    }

    // Is there a better way to implement rotate_left() and rotate_right()?
    // Probably.
    //
    // Am I looking into it right now?
    // Nope.

    pub fn rotate_left(&mut self) {
        let blocks = self.blocks;
        let a = blocks[0];
        let e = blocks[1];
        let i = blocks[2];
        let m = blocks[3];

        let new_blocks = [
            [m[0], i[0], e[0], a[0]],
            [m[1], i[1], e[1], a[1]],
            [m[2], i[2], e[2], a[2]],
            [m[3], i[3], e[3], a[3]],
        ];

        self.blocks = new_blocks;
    }

    pub fn rotate_right(&mut self) {
        let blocks = self.blocks;
        let a = blocks[0];
        let e = blocks[1];
        let i = blocks[2];
        let m = blocks[3];

        let new_blocks = [
            [a[3], e[3], i[3], m[3]],
            [a[2], e[2], i[2], m[2]],
            [a[1], e[1], i[1], m[1]],
            [a[0], e[0], i[0], m[0]],
        ];

        self.blocks = new_blocks;
    }
}

pub const LINE: Piece = Piece {
    blocks: [
        [1,1,1,1],
        [0,0,0,0],
        [0,0,0,0],
        [0,0,0,0],
    ],
    color: PieceColor::Red,
};

pub const SQUARE: Piece = Piece {
    blocks: [
        [0,1,1,0],
        [0,1,1,0],
        [0,0,0,0],
        [0,0,0,0],
    ],
    color: PieceColor::Yellow,
};

pub const ELL: Piece = Piece {
    blocks: [
        [1,1,1,0],
        [1,0,0,0],
        [0,0,0,0],
        [0,0,0,0],
    ],
    color: PieceColor::Orange,
};

pub const ESS: Piece = Piece {
    blocks: [
        [0,1,1,0],
        [1,1,0,0],
        [0,0,0,0],
        [0,0,0,0],
    ],
    color: PieceColor::Green,
};

pub const TEE: Piece = Piece {
    blocks: [
        [1,1,1,0],
        [0,1,0,0],
        [0,0,0,0],
        [0,0,0,0],
    ],
    color: PieceColor::Purple,
};

mod game;
mod piece;
mod position;

use bevy::prelude::*;

use game::*;
use piece::*;
use position::Position;

const PIECE_WIDTH: usize = 4;
const PIECE_HEIGHT: usize = 4;

const GRID_FIRST_VISIBLE: usize = PIECE_HEIGHT;
const GRID_WIDTH: usize = 5;
const GRID_HEIGHT: usize = 6 + PIECE_HEIGHT;

fn main() {
    let mut game = Game::default();

    while !game.stuck() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        game.tick(Action::None);
        game.print();
        std::thread::sleep(std::time::Duration::from_millis(300));
    }

    //App::new().run();
}

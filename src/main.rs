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

#[derive(Resource)]
struct GameClock(Timer);

fn init(mut commands: Commands) {
    println!("init()");
    commands.spawn(Game::default());
}

fn tick(time: Res<Time>, mut timer: ResMut<GameClock>, mut query: Query<&mut Game>) {
    // update our timer with the time elapsed since the last update.
    // if that didn't cause the timer to finish, return immediately
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    println!("tick()");
    for mut game in &mut query {
        game.tick(Action::None); // TODO: handle input
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        game.print();
    }
}

fn main() {
    App::new()
        .insert_resource(GameClock(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_systems(Update, tick)
        .run();
}

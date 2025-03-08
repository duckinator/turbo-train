mod color;
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

fn tick(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<GameClock>,
    mut game: ResMut<Game>,
) {
    // update our timer with the time elapsed since the last update.
    // if that didn't cause the timer to finish, return immediately
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    // TODO: handle input
    if let Some((piece, position)) = game.tick(Action::None) {
        println!("piece {:?} landed @ {:?}", piece, position);

        let sq_size = 20.0;
        let gutter = 2.0;

        let square = Rectangle::new(20.0, 20.0);
        let color = piece.color.to_bevy_color();

        for col_offset in 0..piece.width() {
            for row_offset in 0..piece.height() {
                let col = position.col + col_offset;
                let row = position.row + row_offset;

                if row > GRID_HEIGHT || col > GRID_WIDTH {
                    break;
                }

                if piece.blocks[row_offset][col_offset] != 0 {
                    commands.spawn((
                        Mesh2d(meshes.add(square)),
                        MeshMaterial2d(materials.add(color)),
                        //Transform::from_xyz(60.0 * position.col as f32, 60.0 * position.row as f32, 0.),
                        Transform::from_xyz(
                            (sq_size + gutter) * col as f32,
                            (sq_size + gutter) * (GRID_HEIGHT - row) as f32,
                            0.,
                        ),
                    ));
                }
            }
        }
    }
    //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!();
    game.print();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    //let interval = 1.0;
    let interval = 0.1;

    App::new()
        .insert_resource(Game::default())
        .insert_resource(GameClock(Timer::from_seconds(interval, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, tick)
        .run();
}

use bevy::prelude::*;

const TILE_SIZE: f32 = 40.0;

#[derive(Component)]
struct Board {
    size: u8,
    color: Color,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .run()
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default());
}

fn spawn_board(
    mut commands: Commands,
) {
    let board = Board {size: 4, color: Color::rgb(0.7, 0.2, 0.8)};
    let physical_board_size = f32::from(board.size) * TILE_SIZE;

    let sprite = Sprite{
        custom_size: Some(Vec2::new(
            physical_board_size,
            physical_board_size,
        )),
        color: board.color,
        ..Default::default()
    };

    commands
        .spawn(SpriteBundle {
            sprite: sprite,
            ..Default::default()
        });
}
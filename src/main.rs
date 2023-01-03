use bevy::prelude::*;

const TILE_SIZE: f32 = 40.0;

#[derive(Component)]
struct Board {
    size: u8,
    color: Color,
}

struct Tile {
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

    let board_sprite = Sprite{
        custom_size: Some(Vec2::new(
            physical_board_size,
            physical_board_size,
        )),
        color: board.color,
        ..Default::default()
    };

    let tile = Tile {color: Color::rgb(0.75, 0.75, 0.9)};
    let tile_sprite = Sprite {
        custom_size: Some(Vec2::new(
            TILE_SIZE,
            TILE_SIZE,
        )),
        color: tile.color,
        ..Default::default()
    };

    commands
        .spawn(SpriteBundle {
            sprite: board_sprite,
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(SpriteBundle {
                sprite: tile_sprite,
                transform: Transform::from_xyz(
                    0.0, 0.0, 1.0,
                ),
                ..Default::default()
            });
        });
}
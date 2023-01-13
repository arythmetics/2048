use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;

const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

#[derive(Component)]
struct Board {
    size: u8,
    physical_size: f32,
}
impl Board {
    fn new(size: u8) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE 
            + f32::from(size + 1) * TILE_SPACER;
        Board {
            size,
            physical_size,
        }
    }
    fn cell_position_to_physical(&self, pos: u8) -> f32 {
        let offset = -self.physical_size / 2.0 + 0.5 * TILE_SIZE;
        offset + f32::from(pos) * TILE_SIZE + f32::from(pos + 1) * TILE_SPACER
    }
}

#[derive(Component)]
struct Points {
    value: u32,
}

#[derive(Component)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Component)]
struct TileText;

#[derive(Resource)]
struct FontSpec {
    family: Handle<Font>,
}

impl FromWorld for FontSpec {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource_mut::<AssetServer>()
            .unwrap();
        FontSpec {
            family: asset_server
                .load("fonts/FiraSans-Bold.ttf"),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<FontSpec>()
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .add_startup_system_to_stage(
            StartupStage::PostStartup,
            spawn_tiles,
        )
        .run()
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default());
}

fn spawn_board(
    mut commands: Commands,
) {
    let board = Board::new(4);
    let board_sprite = Sprite{
        custom_size: Some(Vec2::new(
            board.physical_size,
            board.physical_size,
        )),
        color: Color::rgb(0.7, 0.7, 0.8),
        ..Default::default()
    };

    let tile_sprite = Sprite {
        custom_size: Some(Vec2::new(
            TILE_SIZE,
            TILE_SIZE,
        )),
        color: Color::rgb(0.75, 0.75, 0.9),
        ..Default::default()
    };

    commands
        .spawn(SpriteBundle {
            sprite: board_sprite,
            ..Default::default()
        })
        .with_children(|builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                builder.spawn(SpriteBundle {
                    sprite: tile_sprite.clone(),
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(tile.0), 
                        board.cell_position_to_physical(tile.1), 
                        1.0,
                    ),
                    ..Default::default()
                });
            }   
        }).insert(board);
}

fn spawn_tiles(
    mut commands: Commands,
    query_board: Query<&Board>,
    font_spec: Res<FontSpec>,
) {
    let board = query_board
        .single();
    
    let mut rng = rand::thread_rng();
    let starting_tiles: Vec<(u8, u8)> = (0..board.size)
        .cartesian_product(0..board.size)
        .choose_multiple(&mut rng, 2);
    
    for (x, y) in starting_tiles.iter() {
        let pos = Position { x: *x, y: *y };

        let tile_sprite = Sprite {
            color: Color::rgb(0.85, 0.85, 0.9),
            custom_size: Some(Vec2::new(
                TILE_SIZE,
                TILE_SIZE,
            )),
            ..Default::default()
        };

        commands.spawn(SpriteBundle {
            sprite: tile_sprite,
            transform: Transform::from_xyz(
                board.cell_position_to_physical(pos.x),
                board.cell_position_to_physical(pos.y),
                1.0,
            ),
            ..Default::default()
        })
        .with_children(|child_builder| {
            child_builder
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        "2",
                        TextStyle {
                            font: font_spec
                                .family
                                .clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                            ..Default::default()
                        },
                    ).with_alignment(TextAlignment::CENTER),
                    transform: Transform::from_xyz(
                        0.0, 0.0, 1.0,
                    ),
                    ..Default::default()
                })
                .insert(TileText);
        })
        .insert(Points { value: 2 })
        .insert(pos);
    }
}
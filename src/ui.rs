use bevy::prelude::*;
use crate::{FontSpec, Game, RunState};

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ButtonMaterials>()
            .add_startup_system(setup_ui)
            .add_system(scoreboard)
            .add_system(button_interaction_system);
    }
}

#[derive(Resource)]
struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .unwrap();
        ButtonMaterials { 
            normal: materials
                .add(Color::rgb(0.75, 0.75, 0.9).into()), 
            hovered: materials
                .add(Color::rgb(0.70, 0.70, 0.9).into()), 
            pressed: materials
                .add(Color::rgb(0.60, 0.60, 1.0).into()), 
        }
    }
}

fn button_interaction_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
    mut run_state: ResMut<State<RunState>>,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                match run_state.current() {
                    RunState::Playing => {
                        run_state
                            .set(RunState::GameOver)
                            .unwrap();
                    }
                    RunState::GameOver => {
                        run_state
                            .set(RunState::Playing)
                            .unwrap();
                    }
                }
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn setup_ui(
    mut commands:Commands,
    font_spec: Res<FontSpec>
) {
    commands
    .spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::FlexStart,
            padding: UiRect::all(Val::Px(50.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "2048", 
                TextStyle {
                    font: font_spec.family.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                }
            ).with_alignment(
                TextAlignment { 
                    vertical: VerticalAlign::Center, 
                    horizontal: HorizontalAlign::Center,
                }
            ),
            ..Default::default()
        });

        parent
            .spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.0), Val::Auto),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                // scorebox
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::ColumnReverse,
                            align_items: AlignItems::Center,
                            margin: UiRect { 
                                left: Val::Px(20.0), 
                                right: Val::Px(20.0),
                                top: Val::Px(0.0),
                                bottom: Val::Px(0.0),
                            },
                            padding: UiRect::all(Val::Px(10.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                "Score",
                                TextStyle {
                                    font: font_spec.family.clone(),
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                },
                            ).with_alignment(
                                TextAlignment { 
                                    vertical: VerticalAlign::Center, 
                                    horizontal: HorizontalAlign::Center,
                                }
                            ),
                            ..Default::default()
                        });
                        parent
                            .spawn(TextBundle {
                                text: Text::from_section(
                                    "<score>", 
                                    TextStyle { 
                                        font: font_spec.family.clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    }
                                ).with_alignment(
                                    TextAlignment {
                                        vertical: VerticalAlign::Center,
                                        horizontal: HorizontalAlign::Center,
                                    },
                                ),
                                ..Default::default()
                            })
                            .insert(ScoreDisplay);
                    });
                // end scorebox
                // best scorebox
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::ColumnReverse,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(10.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                "Best",
                                TextStyle {
                                    font: font_spec.family.clone(),
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                }).with_alignment(
                                    TextAlignment {
                                        vertical: VerticalAlign::Center,
                                        horizontal: HorizontalAlign::Center,
                                    },
                                ),
                            ..Default::default()
                        });
                        parent
                            .spawn(TextBundle {
                                text: Text::from_section(
                                    "<score>",
                                    TextStyle {
                                        font: font_spec.family.clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                ).with_alignment(
                                    TextAlignment {
                                        vertical: VerticalAlign::Center,
                                        horizontal: HorizontalAlign::Center,
                                    },
                                ),
                                ..Default::default()
                            })
                            .insert(BestScoreDisplay);
                    });
                // end best scorebox
            });
        parent
            .spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(100.0), Val::Px(30.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Button",
                        TextStyle {
                            font: font_spec.family.clone(),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        }
                    ),
                    ..Default::default()
                });
            });
    });
}

fn scoreboard(
    game: Res<Game>,
    mut query_scores: Query<&mut Text, With<ScoreDisplay>>,
) {
    let mut text = query_scores.single_mut();
    text.sections[0].value = game.score.to_string();
}
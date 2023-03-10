use bevy::prelude::*;
use bevy::prelude::BackgroundColor;
use crate::{FontSpec, Game, RunState, MATERIALS};


#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;

#[derive(Resource)]
pub struct ButtonMaterials {
    normal: Color,
    hovered: Color,
    pressed: Color,
}

pub const BUTTON_MATERIALS: ButtonMaterials =
    ButtonMaterials {
        normal: Color::rgb(0.75, 0.75, 0.9),
        hovered: Color::rgb(0.7, 0.7, 0.9),
        pressed: Color::rgb(0.6, 0.6, 1.0),
    };

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_ui)
            .add_system(scoreboard)
            .add_system(button_interaction_system)
            .add_system(button_text_system);
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
        background_color: BackgroundColor(MATERIALS.none),
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
                TextAlignment::default(),
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
                background_color: BackgroundColor(MATERIALS.none),
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
                        background_color: BackgroundColor(MATERIALS.tile_placeholder),
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
                        background_color: BackgroundColor(MATERIALS.tile_placeholder),
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
    mut query_scores: ParamSet<(
        Query<&mut Text, With<ScoreDisplay>>,
        Query<&mut Text, With<BestScoreDisplay>>,
    )>
) {
    let mut binding = query_scores.p0();
    let mut text = binding.single_mut();
    text.sections[0].value = game.score.to_string();

    let mut binding = query_scores.p1();
    let mut text = binding.single_mut();
    text.sections[0].value = game.score_best.to_string();
}

fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut run_state: ResMut<State<RunState>>,
) {
    for (interaction, mut color) 
        in interaction_query.iter_mut() {
            match interaction {
                Interaction::Clicked => {
                    *color = BUTTON_MATERIALS.pressed.into();

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
                    *color = BUTTON_MATERIALS.hovered.into();
                }
                Interaction::None => {
                    *color = BUTTON_MATERIALS.normal.into();
                }
            }
        }
}

fn button_text_system(
    button_query: Query<&Children, With<Button>>,
    mut text_query: Query<&mut Text>,
    run_state: Res<State<RunState>>,
) {
    let children = button_query
        .single();

    let mut text = text_query
        .get_mut(*children.first().expect(
            "expect button to have a first child"
        ))
        .unwrap();

    match run_state.current() {
        RunState::Playing => {
            text.sections[0].value = "End Game".to_string();
        }
        RunState::GameOver => {
            text.sections[0].value = "New Game".to_string();
        }
    }
}

use bevy::prelude::*;
use crate::{FontSpec, Game};

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_ui)
            .add_system(scoreboard);
    }
}

fn setup_ui(
    mut commands:Commands,
    font_spec: Res<FontSpec>
) {
    commands.spawn(Camera2dBundle::default());
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
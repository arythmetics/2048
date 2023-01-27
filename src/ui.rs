use bevy::prelude::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui);
    }
}

fn setup_ui(
    mut commands:Commands,
) {
    commands
    .spawn(NodeBundle {...})
    .with_children(|parent| {
        parent.spawn(TextBundle {...});

        parent
            .spawn(NodeBundle {...})
            .with_children(|parent| {
                // scorebox
                parent
                    .spawn(NodeBundle {...})
                    .with_children(|parent| {
                        parent.spawn(TextBundle {...});
                        parent
                            .spawn(TextBundle {...})
                            .insert(ScoreDisplay);
                    });
                // end scorebox
                // best scorebox
                parent
                    .spawn(NodeBundle {...})
                    .with_children(|parent| {
                        parent.spawn(TextBundle {...});
                        parent
                            .spawn(TextBundle {...})
                            .insert(BestScoreDisplay);
                    });
                // end best scorebox
            });
        parent
            .spawn(ButtonBundle {...})
            .with_children(|parent| {
                parent.spawn(TextBundle {...});
            });
    });
}
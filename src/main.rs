use bevy::prelude::*;
#[cfg(feature = "editor")]
use bevy_editor::EditorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "SolaceInSolitude".into(),
                        name: Some("SolaceInSolitude".into()),
                        ..default()
                    }),
                    ..default()
                }),
            #[cfg(feature = "editor")]
            EditorPlugin,
        ))
        .run();
}

mod world;

use bevy::prelude::*;

use crate::world::WorldPlugin;

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
            WorldPlugin,
        ))
        .run();
}

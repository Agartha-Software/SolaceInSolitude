mod editor;
mod loading;
mod world;

use bevy::prelude::*;

use crate::{editor::EditorPlugin, loading::LoadingPlugin, world::WorldPlugin};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    Game,
}

fn main() {
    App::new()
        .add_plugins(
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
        )
        .init_state::<GameState>()
        .add_plugins((WorldPlugin, LoadingPlugin, EditorPlugin))
        .run();
}

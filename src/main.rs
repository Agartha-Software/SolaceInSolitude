mod editor;
mod loading;
mod player;
mod world;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_skein::SkeinPlugin;

use crate::{
    editor::EditorPlugin, loading::LoadingPlugin, player::PlayerPlugin, world::WorldPlugin,
};

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
        .add_plugins((
            WorldPlugin,
            LoadingPlugin,
            EditorPlugin,
            PlayerPlugin,
            SkeinPlugin::default(),
            PhysicsPlugins::default(),
        ))
        .run();
}

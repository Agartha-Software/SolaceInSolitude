mod flycam;
mod keybinds;

use bevy::prelude::*;
use flycam::FlyCamPlugin;
use keybinds::KeybindsPlugin;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FlyCamPlugin, KeybindsPlugin))
            .add_systems(Startup, spawn_editor);
    }
}

fn spawn_editor(_commands: Commands) {}

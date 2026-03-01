use avian3d::prelude::*;
use bevy::prelude::*;

use crate::editor::keybinds::EditorKeyBindings;

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsDebugPlugin)
            .add_systems(Update, toggle_physics_debug)
            .insert_gizmo_config(
                PhysicsGizmos::default(),
                GizmoConfig {
                    enabled: false,
                    ..default()
                },
            );
    }
}

fn toggle_physics_debug(
    mut store: ResMut<GizmoConfigStore>,
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<EditorKeyBindings>,
) {
    if keys.just_pressed(key_bindings.toggle_debug_display) {
        let phys_config = store.config_mut::<PhysicsGizmos>().0;
        phys_config.enabled = !phys_config.enabled;
    }
}

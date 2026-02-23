/// Heavily inspired by bevy_flycam
use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::editor::keybinds::EditorKeyBindings;

pub struct FlyCamPlugin;

impl Plugin for FlyCamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_flycam)
            .add_systems(Update, (update_flycam, handle_cursor_grab, player_look));
    }
}

#[derive(Component)]
pub struct FlyCam;

fn spawn_flycam(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        FlyCam,
        Transform::from_xyz(-1.0, 1.8, 0.0).looking_at(Vec3::X, Vec3::Y),
    ));
}

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(mut primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    match primary_cursor_options.grab_mode {
        CursorGrabMode::None => {
            primary_cursor_options.grab_mode = CursorGrabMode::Confined;
            primary_cursor_options.visible = false;
        }
        _ => {
            primary_cursor_options.grab_mode = CursorGrabMode::None;
            primary_cursor_options.visible = true;
        }
    }
}

// Grab cursor when an entity with FlyCam is added or keybind pressed
fn handle_cursor_grab(
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<EditorKeyBindings>,
    query_added: Query<Entity, Added<FlyCam>>,
    primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if !query_added.is_empty() && matches!(primary_cursor_options.grab_mode, CursorGrabMode::None) {
        toggle_grab_cursor(primary_cursor_options);
        return;
    }

    if keys.just_pressed(key_bindings.toggle_grab_cursor) {
        toggle_grab_cursor(primary_cursor_options);
    }
}

fn update_flycam(
    mut query: Query<&mut Transform, With<FlyCam>>,
    primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    keybinds: Res<EditorKeyBindings>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for key in keys.get_pressed() {
            match primary_cursor_options.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    let key = *key;
                    if key == keybinds.move_forward {
                        velocity += forward;
                    } else if key == keybinds.move_backward {
                        velocity -= forward;
                    } else if key == keybinds.move_left {
                        velocity -= right;
                    } else if key == keybinds.move_right {
                        velocity += right;
                    } else if key == keybinds.move_ascend {
                        velocity += Vec3::Y;
                    } else if key == keybinds.move_descend {
                        velocity -= Vec3::Y;
                    }
                }
            }
        }

        velocity = velocity.normalize_or_zero();

        transform.translation += velocity * time.delta_secs() * SPEED
    }
}

static SPEED: f32 = 12.;
static SENSITIVITY: f32 = 0.00012;

/// Handles looking around if cursor is locked
fn player_look(
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut state: MessageReader<MouseMotion>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Ok(window) = primary_window.single() {
        for mut transform in query.iter_mut() {
            for ev in state.read() {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match primary_cursor_options.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale = window.height().min(window.width());
                        pitch -= (SENSITIVITY * ev.delta.y * window_scale).to_radians();
                        yaw -= (SENSITIVITY * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

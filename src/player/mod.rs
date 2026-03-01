use avian3d::prelude::*;
use bevy::prelude::*;

use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::default())),
        Transform::from_translation(Vec3 {
            x: 0.,
            y: 2.,
            z: 0.,
        }),
        Collider::capsule(0.5, 1.),
        RigidBody::Dynamic,
        LinearVelocity::default(),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1., 0., 0.).into(),
            ..Default::default()
        })),
    ));
}

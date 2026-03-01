use bevy::{math::Affine2, prelude::*};

use crate::{GameState, loading::MainAssets};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_world);
    }
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<MainAssets>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(assets.color_grid.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        uv_transform: Affine2::from_scale(Vec2::new(100., 100.)),
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(100.0)))),
        MeshMaterial3d(material_handle),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn(SceneRoot(assets.scene.clone()));
}

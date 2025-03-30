use bevy::{prelude::*, render::camera::ScalingMode};
use tilemap::{MAP_SIZE, TILE_SCALE};

pub mod editor;
pub mod tilemap;
pub mod ui;

pub const DEFAULT_CAMERA: Transform = Transform::from_xyz(TILE_SCALE * MAP_SIZE as f32 * 1.2, 75.0, TILE_SCALE * MAP_SIZE as f32 * 1.2);

#[derive(Debug, States, Default, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum AppState {
    #[default]
   StartMenu,
   InGame,
   InEditor,
   PauseMenu,
   Exit, 
}

#[derive(Debug, Event)]
pub struct StartGameEvent;

/// main setup from the beginning
pub fn setup(
    mut commands: Commands,
) {

    // Spawn 3d Camera
    commands.spawn((
        Camera3d::default(),
        Camera::default(),
        Projection::Orthographic(
            OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical { viewport_height: 10.0 * TILE_SCALE },
                ..OrthographicProjection::default_3d()
            }),
            DEFAULT_CAMERA.clone()
                .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
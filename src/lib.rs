use bevy::{prelude::*, render::camera::ScalingMode};
use tilemap::TILE_SCALE;

pub mod editor;
pub mod tilemap;
pub mod ui;


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
        Transform::from_xyz(120.0, 75.0, 120.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
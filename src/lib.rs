use bevy::{animation::{animated_field, AnimationTarget, AnimationTargetId}, math::vec3, prelude::*, render::camera::ScalingMode};
use tilemap::{MAP_SIZE, TILE_SCALE};

pub mod editor;
pub mod tilemap;
pub mod ui;

pub const CAMERA_START_LOC: Transform = Transform::from_xyz(TILE_SCALE * MAP_SIZE as f32 * 1.2, 75.0, TILE_SCALE * MAP_SIZE as f32 * 1.2);
pub const CAMERA_EDITOR_LOC: Transform = Transform::from_xyz(MAP_SIZE as f32 * TILE_SCALE / 2., MAP_SIZE as f32 * TILE_SCALE, MAP_SIZE as f32 * TILE_SCALE / 2.);

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
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut animation_clips: ResMut<Assets<AnimationClip>>,
    mut commands: Commands,
) {
    // Setup Camera Curve Transition Animation
    let animation_target_name = Name::new("Cam");
    let animation_target_id = AnimationTargetId::from_name(&animation_target_name);
    let mut animation_clip = AnimationClip::default();
    let animation_domation = interval(1.0, 5.0);


    let trans_curve = EasingCurve::new(
        vec3(CAMERA_START_LOC.translation.x, CAMERA_START_LOC.translation.y, CAMERA_START_LOC.translation.z),
        vec3(CAMERA_EDITOR_LOC.translation.x, CAMERA_EDITOR_LOC.translation.y, CAMERA_EDITOR_LOC.translation.z),
        EaseFunction::QuadraticInOut
    );

    animation_clip.add_curve_to_target(
        animation_target_id,
        AnimatableCurve::new(animated_field!(Transform::translation), trans_curve),
    );

    let animation_clip_handle = animation_clips.add(animation_clip);

    let (animation_graph, animation_node_index) = AnimationGraph::from_clip(animation_clip_handle);

    let mut animation_player = AnimationPlayer::default();
    animation_player.play(animation_node_index).pause();

    let animation_graph_handle = animation_graphs.add(animation_graph);

    // Spawn 3d Camera
    let cam_id = commands.spawn((
        Camera3d::default(),
        Camera::default(),
        Projection::Orthographic(
            OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical { viewport_height: 10.0 * TILE_SCALE },
                ..OrthographicProjection::default_3d()
            }),
            CAMERA_START_LOC.clone()
                .looking_at(Vec3::ZERO, Vec3::Y),
        animation_target_name,
        animation_player,        
        AnimationGraphHandle(animation_graph_handle)
    )).id();

    commands.entity(cam_id).insert(
        AnimationTarget {
            id: animation_target_id,
            player: cam_id,
        }
    );

}
use std::f32::consts::PI;

use bevy::{animation::{animated_field, AnimationTarget, AnimationTargetId}, math::vec3, prelude::*, render::camera::ScalingMode};

use crate::tilemap::{MAP_SIZE, TILE_SCALE};

pub const CAMERA_START: Vec3 = Vec3::new(TILE_SCALE * MAP_SIZE as f32 * 1.2, TILE_SCALE * MAP_SIZE as f32 * 0.75, TILE_SCALE * MAP_SIZE as f32 * 1.2);
pub const CAMERA_START_LOC: Transform = Transform::from_xyz(CAMERA_START.x, CAMERA_START.y, CAMERA_START.z);
pub const CAMERA_EDITOR: Vec3 = Vec3::new(MAP_SIZE as f32 * TILE_SCALE / 2., MAP_SIZE as f32 * TILE_SCALE, MAP_SIZE as f32 * TILE_SCALE / 2.);
pub const CAMERA_EDITOR_LOC: Transform = Transform::from_xyz(CAMERA_EDITOR.x, CAMERA_EDITOR.y, CAMERA_EDITOR.z);

pub struct CamCtrl;

impl Plugin for CamCtrl {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, setup)
            .add_systems(Update, cam_move_edit)
            ;
    }
}

fn setup(
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut animation_clips: ResMut<Assets<AnimationClip>>,
    mut commands: Commands,
) {
    // Setup Camera Curve Transition Animations
    let mut animation_clip = AnimationClip::default();
    let animation_domain = interval(1.0, 7.0).unwrap();
    let animation_target_name1 = Name::new("PanToEditor");
    let animation_target_id1 = AnimationTargetId::from_name(&animation_target_name1);


    let trans_curve1 = EasingCurve::new(
        vec3(CAMERA_START_LOC.translation.x, CAMERA_START_LOC.translation.y, CAMERA_START_LOC.translation.z),
        // vec3(CAMERA_EDITOR_LOC.translation.x, CAMERA_EDITOR_LOC.translation.y, CAMERA_EDITOR_LOC.translation.z),
        vec3(0.0,50.,0.0),
        EaseFunction::QuadraticInOut
    )
        .ping_pong()
        .expect("curve is domain bounded, shouldn't fail")
        .reparametrize_linear(animation_domain)
        .expect("curve is domain-bouded, shouldn't fail");

    let mut editor_quat = CAMERA_EDITOR_LOC.clone().looking_at(Vec3::new(CAMERA_EDITOR.x, 0.0, CAMERA_EDITOR.z), -Vec3::Z);
    // editor_quat.rotate_y(PI/2.);

    let rot_curve1 = EasingCurve::new(
        CAMERA_START_LOC.clone().looking_at(Vec3::new(CAMERA_EDITOR.x, 0.0, CAMERA_EDITOR.z), Vec3::Y).rotation.normalize(),
        editor_quat.rotation,
        EaseFunction::QuadraticInOut,
    )
        .ping_pong()
        .expect("curve is domain bounded, shouldn't fail")
        .reparametrize_linear(animation_domain)
        .expect("shouldn't fail...");

    animation_clip.add_curve_to_target(
        animation_target_id1,
        AnimatableCurve::new(animated_field!(Transform::translation), trans_curve1),
    );
    animation_clip.add_curve_to_target(
        animation_target_id1,
        AnimatableCurve::new(animated_field!(Transform::rotation), rot_curve1),
    );

    let animation_clip_handle = animation_clips.add(animation_clip);

    let (animation_graph, animation_node_index) = AnimationGraph::from_clip(animation_clip_handle);

    let mut animation_player = AnimationPlayer::default();
    animation_player.play(animation_node_index)
        .repeat()
        .pause();

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
            .looking_at(Vec3::new(CAMERA_EDITOR.x , 0.0, CAMERA_EDITOR.z ), Vec3::Y),
        animation_target_name1,
        animation_player,        
        AnimationGraphHandle(animation_graph_handle)
    )).id();

    commands.entity(cam_id).insert(
        AnimationTarget {
            id: animation_target_id1,
            player: cam_id,
        }
    );
}

fn cam_move_edit(
    buttons: Res<ButtonInput<KeyCode>>,
    mut cam_anim_q: Query<&mut AnimationPlayer, With<Camera>>,
) {
    // TODO move to Editor Pos if AppState::InEditor
    if buttons.just_pressed(KeyCode::Space) {
        let mut cam_anims = cam_anim_q.single_mut();
        if cam_anims.all_paused() {
            //let idx = cam_anims.stop
            cam_anims.resume_all();
        } else {
            cam_anims.pause_all();
        }
    }
}
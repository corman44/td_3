
use std::ops::Deref;

use bevy::{
    animation::{AnimationTarget, AnimationTargetId, animated_field},
    math::vec3,
    prelude::*,
    render::camera::ScalingMode,
};

use crate::{
    AppState,
    tilemap::{MAP_SIZE, TILE_SCALE},
};

pub const CAMERA_START: Vec3 = Vec3::new(
    TILE_SCALE * MAP_SIZE as f32 * 1.2,
    TILE_SCALE * MAP_SIZE as f32 * 0.75,
    TILE_SCALE * MAP_SIZE as f32 * 1.2,
);
pub const CAMERA_START_LOC: Transform =
    Transform::from_xyz(CAMERA_START.x, CAMERA_START.y, CAMERA_START.z);
pub const CAMERA_EDITOR: Vec3 = Vec3::new(
    MAP_SIZE as f32 * TILE_SCALE / 2.,
    MAP_SIZE as f32 * TILE_SCALE,
    MAP_SIZE as f32 * TILE_SCALE / 2.,
);
pub const CAMERA_EDITOR_LOC: Transform =
    Transform::from_xyz(CAMERA_EDITOR.x, CAMERA_EDITOR.y, CAMERA_EDITOR.z);

#[derive(Debug, Clone, Default, States, PartialEq, Eq, Hash)]
pub enum CamState {
    #[default]
    GameView,
    EditorView,
    Moving(CamMoveDir),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum CamMoveDir {
    MoveToEditor,
    MoveToGame,
}

pub struct CamCtrl;

impl Plugin for CamCtrl {
    fn build(&self, app: &mut App) {
        app
            .init_state::<CamState>()
            .add_systems(PreStartup, setup)
            .add_systems(OnEnter(AppState::InEditor), cam_move_edit)
            .add_systems(OnEnter(AppState::InGame), cam_move_game);
    }
}

fn setup(
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut animation_clips: ResMut<Assets<AnimationClip>>,
    mut commands: Commands,
) {
    // Setup Camera Curve Transition Animations
    let mut animation_clip = AnimationClip::default();
    let animation_domain = interval(0.75, 5.5).unwrap();
    let animation_target_name1 = Name::new("PanToEditor");
    let animation_target_id1 = AnimationTargetId::from_name(&animation_target_name1);
    let animation_target_name2 = Name::new("PanToGame");
    let animation_target_id2 = AnimationTargetId::from_name(&animation_target_name2);
    let easing = EaseFunction::QuadraticInOut;

    let trans_curve1 = EasingCurve::new(
        vec3(
            CAMERA_START_LOC.translation.x,
            CAMERA_START_LOC.translation.y,
            CAMERA_START_LOC.translation.z,
        ),
        vec3(
            CAMERA_EDITOR_LOC.translation.x,
            CAMERA_EDITOR_LOC.translation.y,
            CAMERA_EDITOR_LOC.translation.z - TILE_SCALE / 2.,
        ),
        easing,
    )
    .reparametrize_linear(animation_domain)
    .expect("curve is domain-bouded, shouldn't fail");

    let trans_curve2 = trans_curve1.clone().reverse().expect("Expecting reverse possible.");

    let editor_quat = CAMERA_EDITOR_LOC
        .clone()
        .looking_at(Vec3::new(CAMERA_EDITOR.x, 0.0, CAMERA_EDITOR.z), -Vec3::Z);

    let rot_curve1 = EasingCurve::new(
        CAMERA_START_LOC
            .clone()
            .looking_at(Vec3::new(CAMERA_EDITOR.x, 0.0, CAMERA_EDITOR.z), Vec3::Y)
            .rotation
            .normalize(),
        editor_quat.rotation,
        easing,
    )
    .reparametrize_linear(animation_domain)
    .expect("shouldn't fail...");

    let rot_curve2 = rot_curve1.clone().reverse().expect("Expecting reverse possible.");

    animation_clip.add_curve_to_target(
        animation_target_id1,
        AnimatableCurve::new(animated_field!(Transform::translation), trans_curve1),
    );
    animation_clip.add_curve_to_target(
        animation_target_id1,
        AnimatableCurve::new(animated_field!(Transform::rotation), rot_curve1),
    );
    animation_clip.add_curve_to_target(
        animation_target_id2,
        AnimatableCurve::new(animated_field!(Transform::translation), trans_curve2),
    );
    animation_clip.add_curve_to_target(
        animation_target_id2,
        AnimatableCurve::new(animated_field!(Transform::rotation), rot_curve2),
    );

    let animation_clip_handle = animation_clips.add(animation_clip);

    let (animation_graph, animation_node_index) = AnimationGraph::from_clip(animation_clip_handle);

    let mut animation_player = AnimationPlayer::default();
    animation_player.play(animation_node_index).pause();

    let animation_graph_handle = animation_graphs.add(animation_graph);

    // Spawn 3d Camera
    let cam_id = commands
        .spawn((
            Camera3d::default(),
            Camera::default(),
            Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical {
                    viewport_height: TILE_SCALE * MAP_SIZE as f32,
                },
                ..OrthographicProjection::default_3d()
            }),
            CAMERA_START_LOC
                .clone()
                .looking_at(Vec3::new(CAMERA_EDITOR.x, 0.0, CAMERA_EDITOR.z), Vec3::Y),
            animation_target_name1,
            // animation_target_name2,
            animation_player,
            AnimationGraphHandle(animation_graph_handle),
        ))
        .id();

    commands.entity(cam_id).insert(AnimationTarget {
        id: animation_target_id1,
        player: cam_id,
    });
    commands.entity(cam_id).insert(AnimationTarget {
        id: animation_target_id2,
        player: cam_id,
    });
}

fn cam_move_edit(
    buttons: Res<ButtonInput<KeyCode>>,
    mut cam_player: Query<&mut AnimationPlayer, With<Camera>>,
    mut anim_target: Query<&AnimationTarget, With<Camera>>,
    mut cam_nextstate: ResMut<NextState<CamState>>,
    cam_state: ResMut<State<CamState>>,
) {
    if cam_state.get() != &CamState::EditorView && cam_state.get() != &CamState::Moving(CamMoveDir::MoveToEditor) {
        // Not in EditorView or Moving to it, can move cam
        // TODO find animation player with the target name "PanToEditor" 
        let player = cam_player.single_mut();
        for target in anim_target.iter_mut() {
            let name = Name::from("PanToEditor");
            if target.id == AnimationTargetId::from_name(&name) {
                player.play(target.player);
            }

        }

        // let (mut cam_anims, anim_target) = cam_anim_q.single_mut();
        // if cam_anims.all_paused() {
            // //let idx = cam_anims.stop
            // cam_anims.resume_all();
        // } else {
            // cam_anims.pause_all();
        // }
    }
}

fn cam_move_game(
    buttons: Res<ButtonInput<KeyCode>>,
    mut cam_anim_q: Query<(&mut AnimationPlayer, &AnimationTarget), With<Camera>>,
    mut cam_nextstate: ResMut<NextState<CamState>>,
    cam_state: ResMut<State<CamState>>,
) {
    if cam_state.get() != &CamState::GameView && cam_state.get() != &CamState::Moving(CamMoveDir::MoveToGame) {
        // Not in GameView or Moving to it, can move cam
        // TODO find animation player with the target name "PanToGame" 

    } 
} 
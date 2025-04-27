
use bevy::{
    animation::{animated_field, graph::AnimationGraph, graph::AnimationGraphHandle, AnimationPlayer, AnimationTarget, AnimationTargetId, AnimationClip},
    math::vec3,
    prelude::*,
    render::camera::{Camera, OrthographicProjection, Projection, ScalingMode},
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

#[derive(Debug, Resource)]
struct Animations {
    animations: Vec<AnimationNodeIndex>,
    _graph: Handle<AnimationGraph>,
}

pub struct CamCtrl;

impl Plugin for CamCtrl {
    fn build(&self, app: &mut App) {
        app
            .init_state::<CamState>()
            .add_systems(PreStartup, setup)
            .add_systems(OnEnter(AppState::ToEditor), cam_move_edit)
            .add_systems(OnEnter(AppState::ToGame), cam_move_game)
            .add_systems(Update, cam_finished.run_if(
                in_state(CamState::Moving(CamMoveDir::MoveToEditor))
                .or(in_state(CamState::Moving(CamMoveDir::MoveToGame)))));
    }
}

fn setup(
    mut animation_clips: ResMut<Assets<AnimationClip>>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut commands: Commands,
) {
    // Setup Camera Curve Transition Animations
    let mut animation_clip_editor = AnimationClip::default();
    let mut animation_clip_game = AnimationClip::default();
    let animation_domain = interval(0.25, 1.0).unwrap();
    let animation_target_name1 = Name::new("CameraPan");
    let animation_target_id1 = AnimationTargetId::from_name(&animation_target_name1);
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
            .rotation,
            // .normalize(),
        editor_quat.rotation,
        easing,
    )
    .reparametrize_linear(animation_domain)
    .expect("shouldn't fail...");

    let rot_curve2 = rot_curve1.clone().reverse().expect("Expecting reverse possible.");

    animation_clip_editor.add_curve_to_target(
        animation_target_id1,
        AnimatableCurve::new(animated_field!(Transform::translation), trans_curve1),
    );
    animation_clip_editor.add_curve_to_target(
        animation_target_id1,
        AnimatableCurve::new(animated_field!(Transform::rotation), rot_curve1),
    );
    animation_clip_game.add_curve_to_target(
        animation_target_id1,
        AnimatableCurve::new(animated_field!(Transform::translation), trans_curve2),
    );
    animation_clip_game.add_curve_to_target(
        animation_target_id1,
        AnimatableCurve::new(animated_field!(Transform::rotation), rot_curve2),
    );

    let animation_clip_handle_editor = animation_clips.add(animation_clip_editor);
    let animation_clip_handle_game = animation_clips.add(animation_clip_game);

    let (animation_graph_editor, _animation_node_index_editor) = AnimationGraph::from_clip(animation_clip_handle_editor.clone());
    let (animation_graph_game, _animation_node_index_game) = AnimationGraph::from_clip(animation_clip_handle_game.clone());

    let (graph, clips) = AnimationGraph::from_clips([
        animation_clip_handle_game.clone(),
        animation_clip_handle_editor.clone(),
    ]);
    let graph_handle = animation_graphs.add(graph);
    commands.insert_resource(Animations {
        animations: clips,
        _graph: graph_handle.clone(),
    });

    let animation_player = AnimationPlayer::default();

    animation_graphs.add(animation_graph_editor.clone());
    animation_graphs.add(animation_graph_game.clone());

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
            animation_target_name1.clone(),
            animation_player,
            AnimationGraphHandle(graph_handle.clone()),
        ))
        .id();

    commands.entity(cam_id).insert(AnimationTarget {
        id: animation_target_id1,
        player: cam_id,
    });
}

fn cam_move_edit(
    animations: Res<Animations>,
    mut cam_query: Query<&mut AnimationPlayer, With<Camera>>,
    mut cam_nextstate: ResMut<NextState<CamState>>,
    cam_state: Res<State<CamState>>,
) {
    if cam_state.get() != &CamState::EditorView
    && cam_state.get() != &CamState::Moving(CamMoveDir::MoveToEditor)
    && cam_state.get() != &CamState::Moving(CamMoveDir::MoveToGame) {
        cam_nextstate.set(CamState::Moving(CamMoveDir::MoveToEditor));
        let mut player = cam_query.single_mut().expect("Camera not found.. ");
        player.stop_all();
        player.play(*animations.animations.get(1).expect("Animations not initatialized properly.. "));
    }
}

fn cam_move_game(
    animations: Res<Animations>,
    mut cam_query: Query<&mut AnimationPlayer, With<Camera>>,
    mut cam_nextstate: ResMut<NextState<CamState>>,
    cam_state: Res<State<CamState>>,
) {
    if cam_state.get() != &CamState::GameView 
    && cam_state.get() != &CamState::Moving(CamMoveDir::MoveToEditor)
    && cam_state.get() != &CamState::Moving(CamMoveDir::MoveToGame) {
        cam_nextstate.set(CamState::Moving(CamMoveDir::MoveToGame));
        let mut player = cam_query.single_mut().expect("Camera not found.. ");
        player.stop_all();
        player.play(*animations.animations.get(0).expect("Animations not initatialized properly.. "));
    }
}

fn cam_finished(
    mut app_nextstate: ResMut<NextState<AppState>>,
    cam_state: Res<State<CamState>>,
    mut cam_nextstate: ResMut<NextState<CamState>>,
    cam_query: Query<&AnimationPlayer, With<Camera>>
) {
    if cam_query.single().expect("No Anim Player..").all_finished() {
        match cam_state.get() {
            CamState::Moving(cam_move_dir) => {
                match cam_move_dir {
                    CamMoveDir::MoveToEditor => {
                        cam_nextstate.set(CamState::EditorView);
                        app_nextstate.set(AppState::InEditor);
                    },
                    CamMoveDir::MoveToGame => {
                        cam_nextstate.set(CamState::GameView);
                        app_nextstate.set(AppState::InGame);
                    },
                }
            },
            _ => (),
        }
    }
}
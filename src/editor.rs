use bevy::{prelude::*, window::PrimaryWindow}; use crate::{tilemap::GameTilemap, ui::{button, ButtonType}, AppState};

// TODO add functionality to place the paths on existing
// TODO add save functionality (and define format)
// TODO don't allow saving unless a path is defined
    // TODO determine if Enemy Path is valid
    // TODO display message of reason for failed save
// TODO add Load Map functionality 

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Debug, Component)]
struct EditorUI;

#[derive(Debug, Component)]
pub struct MiniTile;

#[derive(Clone, Debug, Component, PartialEq, Eq)]
pub enum TilePath {
    Vertical,
    Horizontal,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Blocked,
    Ground,
}

/// Usage
/// Click a Tile Type (Enemy Path, Free, Rock, Water, etc.) then a small version of that tile follows the cursor while selected
/// when clicking a tile the tile type is applied 
pub struct Editor;

impl Plugin for Editor {
    fn build(&self, app: &mut App) {
        app
            .init_state::<MiniTileState>()
            .add_systems(Update, setup)
            .add_systems(Update, editor_buttons.run_if(in_state(AppState::InEditor)))
            .add_systems(Update, minitile_cursor_follow.run_if(in_state(MiniTileState::Spawned)))
            .add_systems(Update, despawn_minitile.run_if(in_state(MiniTileState::Despawn)));

    }
}

#[derive(States, Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum MiniTileState {
    Spawned,
    #[default]
    NotSpawned,
    Despawn,
}

/// Setup Map Editor
fn setup(
    app_state: Res<State<AppState>>,
    mut commands: Commands,
    mut gtm: ResMut<GameTilemap>,
) {
    if app_state.is_changed() && &AppState::InEditor == app_state.get() {
        // transition to InEditor detected, launch editor

        commands.spawn((
            Node {
                width: Val::Percent(20.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::End,
                align_content: AlignContent::End,
                justify_items: JustifyItems::Center,
                flex_direction: FlexDirection::Column,
                column_gap: Val::Px(10.0),
                ..default()
            },
            EditorUI,
            children![
                // first Row
                (
                    Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    children![
                        button("Vertical", ButtonType::Editor(TilePath::Vertical)),
                        button("Hotizontal", ButtonType::Editor(TilePath::Horizontal)),
                    ]
                ),
                // second Row
                (
                    Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    children![
                        button("Top Left", ButtonType::Editor(TilePath::TopLeft)),
                        button("Top Right", ButtonType::Editor(TilePath::TopRight)),
                    ]
                ),
                // third Row
                (
                    Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    children![
                        button("Bottom Left", ButtonType::Editor(TilePath::BottomLeft)),
                        button("Bottom Right", ButtonType::Editor(TilePath::BottomRight)),
                    ]
                ),
                // Fourth Row
                (
                    Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    children![
                        button("Blocked", ButtonType::Editor(TilePath::Blocked)),
                        button("Ground", ButtonType::Editor(TilePath::Ground)),
                    ]
                ),
            ],
        ));
        
        // Reset Map and Redraw it
        gtm.reset_map();
    }
}

fn editor_buttons(
    mut commands: Commands,
    mut buttons: Query<(&ButtonType, &mut BackgroundColor, &Interaction), (Changed<Interaction>, With<Button>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut minitile_state: ResMut<NextState<MiniTileState>>
) {
    for (button_type, mut _color, interaction) in buttons.iter_mut() {
        let mut tile_type = &TilePath::TopLeft;
        match button_type {
            ButtonType::Editor(tile_path) => tile_type = tile_path,
            _ => (),
        }
        match interaction {
            Interaction::Pressed => {
                spawn_minitile(&mut commands, &mut meshes, &mut materials, &tile_type , &mut minitile_state);
            }
            _ => (),
        }
    }
}

fn spawn_minitile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    tile_path: &TilePath,
    minitile_state: &mut ResMut<NextState<MiniTileState>>,
) {
    info!("Spawning MiniTile");
    commands.spawn((
        Mesh3d::from(meshes.add(Cuboid::new(2., 1., 2.))),
        MeshMaterial3d::from(materials.add(Color::BLACK)),
        Transform::from_translation(Vec3::new(1., 1., 1.)),
        MiniTile,
        tile_path.clone(),
    ));
    minitile_state.set(MiniTileState::Spawned);
}

/// Using State Change to Despawn the MiniTile that follows the cursor
fn despawn_minitile(
    mut commands: Commands,
    minitile_query: Query<Entity, With<MiniTile>>,
) {
    for e in &minitile_query {
        commands.entity(e).despawn();
    }
}

fn minitile_cursor_follow(
    mut minitile: Query<&mut Transform, With<MiniTile>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let mut transform = minitile.single_mut().expect("No minitile found..");
    // FIXME Handle if cursor goes off screen (don't panic.)
    let cursor_pos = window_query.single().expect("No window found").cursor_position().expect("No Cursor Pos..");
    // FIXME how to properly scale where the cursor is compared to the world
    // x: 31 -> 115, cursor: 0 -> 153 
    // y: 0 -> 84 , cursor: 0 -> 84
    // map dimensions is 0 -> 120 in both dirs
    transform.translation.x = (cursor_pos.x / 10. - 34.) * 160./115. ;
    transform.translation.z = (cursor_pos.y / 10. - 3.) * 120./84.;
}

use std::{fs::File, io::Write};

use crate::{
    AppState,
    tilemap::{
        BLOCKED_TILE_COLOR, ENEMY_TILE_COLOR, EnemyTile, GROUND_TILE_COLOR, GameTilemap,
        TileLocation, TileType,
    },
    ui::{ButtonType, button},
};
use bevy::{prelude::*, window::PrimaryWindow};

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

/// Usage:
/// Click a Tile Type (Enemy Path, Free, Rock, Water, etc.) then a small version of that tile follows the cursor while selected
/// when clicking a tile the tile type is applied
pub struct Editor;

impl Plugin for Editor {
    fn build(&self, app: &mut App) {
        app.init_state::<MiniTileState>()
            .add_systems(Update, setup)
            .add_systems(Update, editor_buttons.run_if(in_state(AppState::InEditor)))
            .add_systems(
                Update,
                minitile_cursor_follow.run_if(in_state(MiniTileState::Spawned)),
            )
            .add_systems(
                Update,
                despawn_minitile.run_if(in_state(MiniTileState::Despawn)),
            );
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
fn setup(app_state: Res<State<AppState>>, mut commands: Commands, mut gtm: ResMut<GameTilemap>) {
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
                        button(
                            "Vertical",
                            ButtonType::Editor(TileType::EnemyMap(EnemyTile::Vertical))
                        ),
                        button(
                            "Hotizontal",
                            ButtonType::Editor(TileType::EnemyMap(EnemyTile::Horizontal))
                        ),
                    ]
                ),
                // second Row
                (
                    Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    children![
                        button(
                            "Top Left",
                            ButtonType::Editor(TileType::EnemyMap(EnemyTile::TopLeft))
                        ),
                        button(
                            "Top Right",
                            ButtonType::Editor(TileType::EnemyMap(EnemyTile::TopRight))
                        ),
                    ]
                ),
                // third Row
                (
                    Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    children![
                        button(
                            "Bottom Left",
                            ButtonType::Editor(TileType::EnemyMap(EnemyTile::BottomLeft))
                        ),
                        button(
                            "Bottom Right",
                            ButtonType::Editor(TileType::EnemyMap(EnemyTile::BottomRight))
                        ),
                    ]
                ),
                // Fourth Row
                (
                    Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    children![
                        button("Blocked", ButtonType::Editor(TileType::Blocked)),
                        button("Ground", ButtonType::Editor(TileType::Free)),
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
    mut buttons: Query<
        (&ButtonType, &mut BackgroundColor, &Interaction),
        (Changed<Interaction>, With<Button>),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut minitile_state: ResMut<NextState<MiniTileState>>,
    minitile: Query<Entity, With<MiniTile>>,
) {
    for (button_type, mut _color, interaction) in buttons.iter_mut() {
        let mut tile_type = &TileType::Free;
        match button_type {
            ButtonType::Editor(tt) => tile_type = tt,
            _ => (),
        }
        match interaction {
            Interaction::Pressed => {
                // first need to despawn the existing mt
                for mt in minitile {
                    commands.entity(mt).despawn();
                }

                // then spawn a new one
                spawn_minitile(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &tile_type,
                    &mut minitile_state,
                );
            }
            _ => (),
        }
    }
}

fn spawn_minitile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    tile_type: &TileType,
    minitile_state: &mut ResMut<NextState<MiniTileState>>,
) {
    let material = tiletype_to_mesh3d(tile_type, materials);
    info!("Spawning MiniTile");
    commands.spawn((
        Mesh3d::from(meshes.add(Cuboid::new(2., 1., 2.))),
        material,
        Transform::from_translation(Vec3::new(1., 1., 1.)),
        MiniTile,
        tile_type.clone(),
    ));
    minitile_state.set(MiniTileState::Spawned);
}

fn tiletype_to_mesh3d(
    tiletype: &TileType,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> MeshMaterial3d<StandardMaterial> {
    match tiletype {
        TileType::EnemyMap(_enemy_tile) => MeshMaterial3d::from(materials.add(ENEMY_TILE_COLOR)),
        TileType::Blocked => MeshMaterial3d::from(materials.add(BLOCKED_TILE_COLOR)),
        TileType::Free => MeshMaterial3d::from(materials.add(GROUND_TILE_COLOR)),
        TileType::Tower(_tower_type) => MeshMaterial3d::from(materials.add(GROUND_TILE_COLOR)),
    }
}

/// Despawn the MiniTile that follows the cursor
fn despawn_minitile(mut commands: Commands, minitile_query: Query<Entity, With<MiniTile>>) {
    for e in &minitile_query {
        commands.entity(e).despawn();
    }
}

/// MiniTile cursor following after spawned
fn minitile_cursor_follow(
    mut minitile: Query<&mut Transform, With<MiniTile>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // FIXME how to properly scale where the cursor is compared to the world
    let mut transform = minitile.single_mut().expect("No minitile found..");
    match window_query.single() {
        Ok(win) => match win.cursor_position() {
            Some(pos) => {
                transform.translation.x = (pos.x / 10. - 34.) * 160. / 115.;
                transform.translation.z = (pos.y / 10. - 3.) * 120. / 84.;
            }
            None => info!("Cursor Position not found.. "),
        },
        _ => (),
    }
}

fn save_map(tile_query: Query<(&TileType, &TileLocation)>) {
    let mut file = File::create("map_save.txt").expect("unable to create file.. ");
    for (tt, t_loc) in tile_query.iter() {
        match tt {
            TileType::EnemyMap(enemy_tile) => match enemy_tile {
                EnemyTile::Start => {
                    let _ =
                        file.write(&format!("S, ({},{})\n", t_loc.0.x, t_loc.0.y).into_bytes());
                }
                EnemyTile::TopLeft => {
                    let _ = file.write(&format!("TL, ({},{})\n", t_loc.0.x, t_loc.0.y).into_bytes());
                },
                EnemyTile::TopRight => {
                    let _ = file.write(&format!("TR, ({},{})\n", t_loc.0.x, t_loc.0.y).into_bytes());
                },
                EnemyTile::BottomLeft => {
                    let _ = file.write(&format!("BL, ({},{})\n", t_loc.0.x, t_loc.0.y).into_bytes());
                },
                EnemyTile::BottomRight => {
                    let _ = file.write(&format!("BR, ({},{})\n", t_loc.0.x, t_loc.0.y).into_bytes());
                },
                EnemyTile::Horizontal => {
                    let _ = file.write(&format!("H, ({},{})\n", t_loc.0.x, t_loc.0.y).into_bytes());
                },
                EnemyTile::Vertical => {
                    let _ = file.write(&format!("V, ({},{})\n", t_loc.0.x, t_loc.0.y).into_bytes());
                },
                EnemyTile::Finish => {
                    let _ = file.write(&format!("F, ({},{})\n", t_loc.0.x, t_loc.0.y).into_bytes());
                },
            },
            TileType::Blocked => {
                let _ = file.write(&format!("B, ({},{})\n", t_loc.0.x, t_loc.0.y).into_bytes());
            }
            TileType::Free => {
                let _ = file.write(&format!("F, ({},{})\n", t_loc.0.x, t_loc.0.y).into_bytes());
            }
            _ => (),
        }
    }
}

use std::{
    collections::HashMap,
    env::current_dir,
    fs::File,
    io::{Read, Write},
};

use crate::{
    tilemap::{
        update_gametilemap, EnemyPath, EnemyTile, GameTilemap, MapState, TileLocation, TileType, UpdateColorMap, BLOCKED_TILE_COLOR, ENEMY_TILE_COLOR, GROUND_TILE_COLOR
    }, ui::{button, ButtonType, MenuType, PreviousButtonState}, AppState
};
use bevy::{prelude::*, window::PrimaryWindow};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// TODO improve MiniTile (add border, fix offset and movement)

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Debug, Component, Event)]
struct SaveMapEvent;

#[derive(Debug, Component, Event)]
struct LoadMapEvent;

#[derive(Debug, Component, Event)]
struct ClearMapEvent;

#[derive(Debug, Component)]
struct EditorUI;

#[derive(Debug, Component)]
pub struct MiniTile;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SavedTileMap(#[serde_as(as = "Vec<(_, _)>")] pub HashMap<TileType, Vec<IVec2>>);

impl SavedTileMap {
    pub fn new() -> Self {
        SavedTileMap(HashMap::new())
    }
}


/// Usage:
/// Click a Tile Type (Enemy Path, Free, Rock, Water, etc.) then a small version of that tile follows the cursor while selected
/// when clicking a tile the tile type is applied
pub struct Editor;

impl Plugin for Editor {
    fn build(&self, app: &mut App) {
        app.init_state::<MiniTileState>()
            .add_event::<SaveMapEvent>()
            .add_event::<LoadMapEvent>()
            .add_event::<ClearMapEvent>()
            .add_systems(Update, setup)
            .add_systems(
                Update,
                (editor_buttons, save_map, load_map, clear_map).run_if(in_state(AppState::InEditor)),
            )
            .add_systems(
                Update,
                minitile_cursor_follow.run_if(in_state(MiniTileState::Spawned)),
            )
            .add_systems(
                // TODO alter despawn to utilize an event system instead of a state system
                Update,
                despawn_minitile.run_if(in_state(MiniTileState::Despawn)),
            )
            .add_systems(
                Update,
                map_verify.run_if(in_state(MapState::NeedsVerify))
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
                            "Start",
                            ButtonType::Editor(TileType::EnemyMap(EnemyTile::Start))
                        ),
                        button(
                            "Finish",
                            ButtonType::Editor(TileType::EnemyMap(EnemyTile::Finish))
                        ),
                    ]
                ),
                // Second Row
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
                // Third Row
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
                // Fourth Row
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
                // Fifth Row
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
                // Sixth Row
                (
                    Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    children![
                        button("Clear", ButtonType::Menu(MenuType::Clear)),
                    ]
                ),
                // Seventh Row
                (
                    Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    children![
                        button("Save", ButtonType::Menu(MenuType::Save)),
                        button("Load", ButtonType::Menu(MenuType::Load)),
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
        (&ButtonType, &mut BackgroundColor, &Interaction, &PreviousButtonState),
        (Changed<Interaction>, With<Button>),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut minitile_state: ResMut<NextState<MiniTileState>>,
    minitile: Query<Entity, With<MiniTile>>,
    mut ev_save_map: EventWriter<SaveMapEvent>,
    mut ev_load_map: EventWriter<LoadMapEvent>,
    mut ev_clear_map: EventWriter<ClearMapEvent>,
) {
    for (button_type, mut _color, interaction, prev_butt_state) in buttons.iter_mut() {

        // button press handling
        match button_type {
            ButtonType::Editor(tt) => {
                match interaction {
                    Interaction::Pressed => {
                        for mt in minitile {
                            commands.entity(mt).despawn();
                        }

                        // then spawn a new one
                        spawn_minitile(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            tt,
                            &mut minitile_state,
                        );
                    }
                    _ => (),
                }
            },
            ButtonType::Menu(menu) => match menu {
                MenuType::Save => match interaction {
                    Interaction::Pressed => {
                        ev_save_map.write(SaveMapEvent);
                    }
                    _ => (),
                },
                MenuType::Load => match interaction {
                    Interaction::Pressed => {
                        // info!("LoadButton Pressed; prev_button State: {:?}",prev_butt_state.0);
                        if prev_butt_state.0 != Interaction::Pressed {
                            // info!("Loading Map");
                            ev_load_map.write(LoadMapEvent);
                        }
                    }
                    _ => (),
                },
                MenuType::Clear => match interaction {
                    Interaction::Pressed => {
                        if prev_butt_state.0 != Interaction::Pressed {
                            ev_clear_map.write(ClearMapEvent);
                        }
                    },
                    _ => (),
                }
                _ => (),
            },
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
            None => (),
        },
        _ => (),
    }
}

fn save_map(tile_query: Query<(&TileType, &TileLocation)>, ev_save_map: EventReader<SaveMapEvent>) {
    if ev_save_map.is_empty() {
        return;
    }

    let tilemap: SavedTileMap = tile_query
        .iter()
        .map(|(tt, t_loc)| (tt.clone(), t_loc.clone().0))
        .collect::<Vec<(TileType, IVec2)>>()
        .into_iter()
        .fold(SavedTileMap::new(), |mut acc, (tt, t_loc)| {
            acc.0.entry(tt).or_insert_with(Vec::new).push(t_loc);
            acc
        });

    if let Some(mut file) = File::create("maps/map_save.txt").ok() {
        let _ = file.write(serde_json::to_string(&tilemap).unwrap().as_bytes());
    } else {
        info!("Unable to save file 'maps/map_save.txt'");
    }
}

fn load_map(
    mut ev_load_map: EventReader<LoadMapEvent>,
    mut enemy_path: ResMut<EnemyPath>,
    mut map_nextstate: ResMut<NextState<MapState>>,
    mut gtm: ResMut<GameTilemap>,
    mut ev_update_colormap: EventWriter<UpdateColorMap>,
) {
    if ev_load_map.is_empty() {
        return;
    }
    // info!("ev_load_map: {:?}",ev_load_map);

    // get file and read contents
    // FIXME replace .expects with if let Some()
    let cwd = current_dir().expect("unable to get current directory.. ");
    let file_path = FileDialog::new()
        .add_filter("text", &["txt"])
        .set_directory(cwd.to_str().unwrap())
        .pick_file()
        .expect("unable to open file.. ");

    let mut file = File::open(
        file_path
        .to_str()
        .unwrap()
    ).expect("unable to open file.. ");
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("unable to read file.. ");

    let tilemap: SavedTileMap =
        serde_json::from_str(&contents)
        .expect(&format!("Unable to parse file; file: {:?}", file));

    // format as GameTilemap
    let mut new_gtm = GameTilemap::default();
    for (tt, locs) in tilemap.0.iter() {
        for loc in locs {
            new_gtm.0.insert(*loc, tt.clone());
        }
    }

    // update the gametilemap
    update_gametilemap(
        &mut gtm,
        &mut enemy_path,
        &new_gtm,
        &mut map_nextstate,
        &mut ev_update_colormap,
    );

    // clear event to not trigger this function again
    ev_load_map.clear();
}

fn map_verify(
    gtm: Res<GameTilemap>,
    mut map_nextstate: ResMut<NextState<MapState>>,
) {
    // FIXME need to check latest loaded map (maybe temp version of GTM intead of game version of GTM)
    // collect enemy positions
    let mut enemy_tiles = gtm.0.iter()
            .filter_map(|(loc, tt)| {
                if let TileType::EnemyMap(enemy_tile) = tt {
                    Some((loc, enemy_tile))
                } else {
                    None
                }
            })
            .collect::<Vec<(&IVec2, &EnemyTile)>>();
    
    // find start
    let start_idx = enemy_tiles.iter().position(|(_loc, et)| *et == &EnemyTile::Start);
    if let Some(idx) = start_idx {
        // pop the start location from enemy_tiles (saves 1 iteration?)
        let start = enemy_tiles.remove(idx);
        if check_valid_neighbor(start.0, &mut enemy_tiles) {
            map_nextstate.set(MapState::Reloaded);
            info!("Map Valid");
        }
        else {
            map_nextstate.set(MapState::VerifyFailed);
            info!("Map Not Valid");
        }
    }
}

fn check_valid_neighbor(loc: &IVec2, tiles: &mut Vec<(&IVec2, &EnemyTile)>
) -> bool {
    // info!("Checking: {:?}", loc);
    for (i_loc, tile) in tiles.clone() {
        // info!("Checking Tile: {:?}, {:?}", i_loc, tile);
        if ((loc.x - i_loc.x).abs() <= 1 && (loc.y - i_loc.y).abs() == 0) ||
           ((loc.x - i_loc.x).abs() == 0 && (loc.y - i_loc.y).abs() <= 1)
        {
            if *tile == EnemyTile::Finish {
                return true
            } else {
                let new_loc = i_loc;
                if let Some(idx) = tiles.iter().position(|(l,t)| l == &i_loc && t == &tile) {
                    tiles.remove(idx);
                }
                return check_valid_neighbor(new_loc, tiles);
            }
        }
    }
    info!("No valid neighbor found");
    return false;
}

fn clear_map(
    ev_clear_map: EventReader<ClearMapEvent>,
    mut gtm: ResMut<GameTilemap>,
    mut ev_update_colormap: EventWriter<UpdateColorMap>,
) {
    if ev_clear_map.is_empty() {
        return
    }

    // clear GTM
    for (_loc, tt) in gtm.0.iter_mut() {
        *tt = TileType::Free;
    }

    // update map visuals
    ev_update_colormap.write(UpdateColorMap);


}

use crate::{editor::{MiniTile, MiniTileState}, StartGameEvent};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const BLOCKED_TILE_COLOR: Color = Color::srgb(0.88, 0.88, 0.88);
pub const GROUND_TILE_COLOR: Color = Color::srgb(0.15, 0.75, 0.25);
pub const ENEMY_TILE_COLOR: Color = Color::srgb(0.75, 0.35, 0.25);
pub const HOVER_COLOR: Color = Color::srgb(0.1, 0.65, 0.2);
pub const TILE_SCALE: f32 = 10.0;
pub const MAP_SIZE: i32 = 12;


#[derive(Debug, Clone, Event )]
pub struct UpdateColorMap;

/// enum for tile types
#[derive(Debug, Component, Clone, Default, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TileType {
    EnemyMap(EnemyTile),
    Blocked,
    #[default]
    Free,
    Tower(TowerType),
}

#[derive(Debug, Component, Clone, PartialEq, Eq)]
pub struct TileLocation(pub IVec2);

#[derive(Debug, Clone, States, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MapState {
    #[default]
    NotSpawned,
    Spawned,
    Reloaded,
    NeedsVerify,
    VerifyFailed,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Copy, Hash, Serialize, Deserialize)]
pub enum EnemyTile {
    Start,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Horizontal,
    #[default]
    Vertical,
    Finish,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
pub enum TowerType {
    T1,
    T2,
    T3,
}

/// Tilemap resource
/// Holds the tilemap data for 1 Global Tilemap
#[derive(Debug, Resource, Clone, Default)]
pub struct GameTilemap(pub HashMap<IVec2, TileType>);

impl GameTilemap {
    pub fn new(size: i32) -> Self {
        let mut gtm = GameTilemap::default();
        for i in 0..size {
            for j in 0..size {
                gtm.0.insert(IVec2::new(i, j), TileType::Free);
            }
        }
        gtm
    }

    pub fn reset_map(&mut self) {
        for tile in &mut self.0 {
            *tile.1 = TileType::Free;
        }
    }
}

pub struct Tilemap;
impl Plugin for Tilemap {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameTilemap::new(MAP_SIZE))
            .insert_resource(EnemyPath(None))
            .init_state::<MapState>()
            .add_event::<UpdateColorMap>()
            .add_systems(Startup, setup_tilemap)
            .add_systems(Update, (spawn_map, update_tile_colors));
    }
}

fn spawn_map(
    mut commands: Commands,
    mut ev_start_game: EventReader<StartGameEvent>,
    gtm: Res<GameTilemap>,
    map_state: Res<State<MapState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_map_state: ResMut<NextState<MapState>>,
) {
    for _ev in ev_start_game.read() {
        if map_state.as_ref() != &MapState::Spawned {
            // Spawn Ground Tiles
            let map = gtm.0.clone();

            for (v, tile) in map.iter() {
                let tile_color: Color;
                match tile {
                    TileType::EnemyMap(_et) => {
                        tile_color = ENEMY_TILE_COLOR;
                    }
                    TileType::Blocked => {
                        tile_color = GROUND_TILE_COLOR;
                    }
                    TileType::Free => {
                        tile_color = GROUND_TILE_COLOR;
                    }
                    TileType::Tower(_tt) => {
                        tile_color = GROUND_TILE_COLOR;
                    }
                }

                commands
                    .spawn((
                        Mesh3d(meshes.add(Cuboid::new(1.0 * TILE_SCALE, 0.1, 1.0 * TILE_SCALE))),
                        MeshMaterial3d(materials.add(tile_color)),
                        Transform::from_xyz(v.x as f32 * TILE_SCALE, 0.0, v.y as f32 * TILE_SCALE),
                        TileLocation(IVec2::new(v.x, v.y)),
                        tile.clone(),
                    ))
                    .observe(alter_tile::<Pointer<Pressed>>())
                    .observe(recolor::<Pointer<Over>>(0.15))
                    .observe(recolor::<Pointer<Out>>(0.0));

                // Spawn Ambient Light
                commands.insert_resource(AmbientLight {
                    color: Color::WHITE,
                    brightness: 1000.0,
                    ..default()
                });

                next_map_state.set(MapState::Spawned);
            }
        }
    }
}

/// Scale color up and down
fn recolor<E>(
    scale: f32,
) -> impl Fn(
    Trigger<E>,
    Query<(&TileType, &mut MeshMaterial3d<StandardMaterial>)>,
    ResMut<Assets<StandardMaterial>>,
) {
    move |trigger, mut query, mut materials| {
        let ent = trigger.target();
        let (tt_ent, mut mat) = query
            .get_mut(ent)
            .expect(&format!("Expected (TileType, Material) found in Entity: {:?}", ent));

        match *tt_ent {
            TileType::EnemyMap(_enemy_tile) => {
                mat.0 = materials.add(ENEMY_TILE_COLOR.darker(scale));
            }
            TileType::Blocked => {
                mat.0 = materials.add(BLOCKED_TILE_COLOR);
            }
            _ => {
                mat.0 = materials.add(GROUND_TILE_COLOR.darker(scale));
            }
        }
    }
}

fn alter_tile<E>() -> impl Fn(
    Trigger<E>,
    Query<&mut MeshMaterial3d<StandardMaterial>>,
    ResMut<Assets<StandardMaterial>>,
    Query<&TileType, With<MiniTile>>,
    Res<State<MiniTileState>>,
    Query<&mut TileType, Without<MiniTile>>,
) {
    move |trigger, mut query, mut materials, tile_type, minitile_state, mut tt_query | {
        if minitile_state.get() == &MiniTileState::Spawned {
            let selected_tt = tile_type.single().expect("no TileType found..");
            let ent = trigger.target();
            let mut mat = query.get_mut(ent).expect("No Mat found for ent");

            let mut tiletype = tt_query.get_mut(ent).expect("No TileType Found for ent.. ");
            *tiletype = selected_tt.clone();

            match selected_tt {
                TileType::EnemyMap(_enemy_tile) => mat.0 = materials.add(ENEMY_TILE_COLOR),
                TileType::Blocked => mat.0 = materials.add(BLOCKED_TILE_COLOR),
                TileType::Free => mat.0 = materials.add(GROUND_TILE_COLOR),
                TileType::Tower(_tower_type) => mat.0 = materials.add(GROUND_TILE_COLOR),
            }
            
            // TODO need GTM update in order to retain the tile_type after mouse leaves hover 
        }
    }
}

#[derive(Debug, Resource, Clone)]
pub struct EnemyPath(Option<Vec<IVec2>>);

//// System to setup the tilemap on Startup
fn setup_tilemap(
    mut gtm: ResMut<GameTilemap>,
    mut enemy_path: ResMut<EnemyPath>
) {
    let default_path = Some(vec![
        IVec2::new(1, 1),
        IVec2::new(1, 2),
        IVec2::new(1, 3),
        IVec2::new(1, 4),
        IVec2::new(1, 5),
        IVec2::new(2, 5),
        IVec2::new(2, 6),
        IVec2::new(3, 6),
        IVec2::new(4, 6),
        IVec2::new(5, 6),
        IVec2::new(5, 7),
        IVec2::new(5, 8),
        IVec2::new(6, 8),
        IVec2::new(7, 8),
        IVec2::new(8, 8),
    ]);

    if enemy_path.0.is_none() {
        enemy_path.0 = default_path;
    }

    // First tile is Start and last Tile is finish
    for (idx, tile) in enemy_path.0.clone().expect("no path..").iter().enumerate() {
        if idx == 0 {
            gtm.0.insert(*tile, TileType::EnemyMap(EnemyTile::Start));
        } else if idx == enemy_path.0.clone().unwrap().len() - 1 {
            gtm.0.insert(*tile, TileType::EnemyMap(EnemyTile::Finish));
        } else {
            gtm.0.insert(*tile, TileType::EnemyMap(EnemyTile::Vertical));
        }
    }
}

/// Callable function to update the GameTilemap and EnemyPath
/// Load Map Sequence
/// FIXME improve below sequence
/// 1. Map File Selected
/// 2. Load Map, update GTM and Enemy Path -> MapState::NeedsVerify
/// 3. Map Verification Occurs -> MapState::Reloaded
pub fn update_gametilemap(
    gtm: &mut ResMut<GameTilemap>,
    enemy_path: &mut ResMut<EnemyPath>,
    loaded_map: &GameTilemap,
    map_nextstate: &mut ResMut<NextState<MapState>>, 
    ev_update_colormap: &mut EventWriter<UpdateColorMap>,
) {
    gtm.0 = loaded_map.0.clone();
    map_nextstate.set(MapState::NeedsVerify);
    enemy_path.0 = Some(vec![]);

    // update EnemyPath with latest GTM
    for (t_loc, tt) in gtm.0.iter() {
        if let TileType::EnemyMap(_et) = tt {
            enemy_path.0.as_mut().unwrap().push(*t_loc);
        }
    }

    ev_update_colormap.write(UpdateColorMap);
}

/// System to update the colors of the tiles based on their type
/// Triggered by the UpdateColorMap event
pub fn update_tile_colors(
    ev_update_colormap: EventReader<UpdateColorMap>,
    gtm: Res<GameTilemap>,
    mut query: Query<(&TileLocation, &mut TileType, &mut MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_map_state: ResMut<NextState<MapState>>,
) {
    if ev_update_colormap.is_empty() {
        return;
    }

    // Update the colors of the tiles based on their type
    for (tile_loc, mut tt, mut mat) in query.iter_mut() {
        let tile = gtm.0.get(&tile_loc.0).unwrap();
        match tile {
            TileType::EnemyMap(_et) => {
                mat.0 = materials.add(ENEMY_TILE_COLOR);
            }
            TileType::Blocked => {
                mat.0 = materials.add(BLOCKED_TILE_COLOR);
            }
            TileType::Free => {
                mat.0 = materials.add(GROUND_TILE_COLOR);
            }
            TileType::Tower(_tt) => {
                mat.0 = materials.add(GROUND_TILE_COLOR);
            }
        }
        *tt = tile.clone();
    }

    next_map_state.set(MapState::Spawned);
}

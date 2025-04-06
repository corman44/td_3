use crate::StartGameEvent;
use bevy::prelude::*;
use std::collections::HashMap;

pub const GROUND_TILE_COLOR: Color = Color::srgb(0.15, 0.75, 0.25);
pub const ENEMY_TILE_COLOR: Color = Color::srgb(0.75, 0.35, 0.25);
pub const HOVER_COLOR: Color = Color::srgb(0.1, 0.65, 0.2);
pub const TILE_SCALE: f32 = 10.0;
pub const MAP_SIZE: i32 = 12;

/// enum for tile types
#[derive(Debug, Component, Clone, Default, PartialEq, Eq, Copy)]
pub enum TileType {
    EnemyMap(EnemyTile),
    Blocked,
    #[default]
    Free,
    Tower(TowerType),
}

#[derive(Debug, Component, Clone, PartialEq, Eq)]
pub struct TileLocation(IVec2);

#[derive(Debug, Clone, States, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MapState {
    #[default]
    NotSpawned,
    Spawned,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Copy)]
pub enum EnemyTile {
    Start,
    #[default]
    Path,
    Finish,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum TowerType {
    T1,
    T2,
    T3,
}

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
        app.insert_resource(GameTilemap::new(MAP_SIZE))
            .insert_resource(EnemyPath(None))
            .init_state::<MapState>()
            .add_systems(Startup, setup_tilemap)
            .add_systems(Update, spawn_map);
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
            // let size = gtm.1;
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
                    // .observe(|trigger: Trigger<Pointer<Click>>| { info!("{:?} was clicked!", trigger.target)})
                    .observe(recolor::<Pointer<Over>>(0.15))
                    .observe(recolor::<Pointer<Out>>(0.0));

            // Spawn Ambient Light
            commands.insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 1000.0,
            });

            next_map_state.set(MapState::Spawned);
        }
    }
}
}

/// Scale color up and down
fn recolor<E>(
   scale: f32 
) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial3d<StandardMaterial>>, ResMut<Assets<StandardMaterial>>, Query<&TileType>) {
    move |trigger, mut query, mut materials, tile_type| {

        let ent = trigger.entity();
        let mut mat = query.get_mut(ent).expect(&format!("Expected material found in Entity: {:?}", ent));
        let tile_type = tile_type.get(ent).expect(&format!("no TileType found for Entity: {:?}",ent));
        
        match tile_type {
            TileType::EnemyMap(_enemy_tile) => {
                mat.0 = materials.add(ENEMY_TILE_COLOR.darker(scale));
            },
            _ => {
                mat.0 = materials.add(GROUND_TILE_COLOR.darker(scale));

            }
        }
    }
}

#[derive(Debug, Resource, Clone)]
pub struct EnemyPath(Option<Vec<IVec2>>);

fn setup_tilemap(
    mut gtm: ResMut<GameTilemap>,
    mut enemy_path: ResMut<EnemyPath>,
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
            gtm.0.insert(*tile, TileType::EnemyMap(EnemyTile::Path));
        }
    }
    // info!(
        // "Enemy Path len: {}",
        // gtm.0
            // .iter()
            // .filter(|(_, x)| matches!(x, TileType::EnemyMap(_)))
            // .count()
    // );
}

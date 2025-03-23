use std::collections::HashMap;
use bevy::{prelude::*, render::camera::ScalingMode};
use crate::StartGameEvent;

const GROUND_TILE_COLOR: Color =  Color::srgb(0.15, 0.75, 0.25);
const ENEMY_TILE_COLOR: Color =  Color::srgb(0.75, 0.35, 0.25);
const TILE_SCALE: f32 = 10.0;

#[derive(Debug, Component, Clone, Default, PartialEq, Eq)]
pub enum TileType {
    EnemyMap(EnemyTile),
    Blocked,
    #[default]
    Free,
    Tower(TowerType),
}

#[derive(Debug, Clone, States, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum MapState {
    #[default]
    NotSpawned,
    Spawned,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
enum EnemyTile {
    Start,
    #[default]
    Path,
    Finish,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TowerType {
    T1,
    T2,
    T3,
}

#[derive(Debug, Resource, Clone, Default)]
pub struct GameTilemap(HashMap<IVec2,TileType>);

impl GameTilemap {
    pub fn new(size: i32) -> Self {
        let mut gtm = GameTilemap::default();
        for i in 0..size {
            for j in 0..size {
                gtm.0.insert(IVec2::new(i, j),TileType::Free);
            }
        }
        gtm
    }
}

pub struct Tilemap;

impl Plugin for Tilemap {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameTilemap::new(10))
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
            info!("Spawning Map..");

            // Spawn Ground Tiles
            let map = gtm.0.clone();
            // let size = gtm.1;
            for (v, tile) in map.iter() {
                let mut tile_color: Color;
                match tile {
                    TileType::EnemyMap(_et) => {
                        tile_color = ENEMY_TILE_COLOR;
                    },
                    TileType::Blocked => {
                        tile_color = GROUND_TILE_COLOR;
                    },
                    TileType::Free => {
                        tile_color = GROUND_TILE_COLOR;
                    },
                    TileType::Tower(_tt) => {
                        tile_color = GROUND_TILE_COLOR; 
                    },
                }
                commands.spawn((
                    // BorderColor(Color::BLACK),
                    Mesh3d(meshes.add(Cuboid::new(1.0 * TILE_SCALE, 0.1, 1.0 * TILE_SCALE))),
                    MeshMaterial3d(materials.add(tile_color)),
                    Transform::from_xyz(v.x as f32 * TILE_SCALE, 0.0, v.y as f32 * TILE_SCALE),
                    tile.clone(),
                ));
            }
        
            // Spawn 3d Camera
            commands.spawn((
                Camera3d {
                    ..default()
                },
                Projection::Orthographic(
                    OrthographicProjection {
                        scaling_mode: ScalingMode::FixedVertical { viewport_height: 10.0 * TILE_SCALE },
                        ..OrthographicProjection::default_3d()
                    }),
                Transform::from_xyz(120.0, 75.0, 120.0).looking_at(Vec3::ZERO, Vec3::Y),
            ));

            // Spawn Ambient Light
            commands.insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 1000.0,
            });

            // Spawn grid overlay on ground
            // TODO how to add border to all blocks?

            next_map_state.set(MapState::Spawned);
        }
    }
}

#[derive(Debug, Resource, Clone)]
pub struct EnemyPath(Option<Vec<IVec2>>);

fn setup_tilemap(
    mut gtm: ResMut<GameTilemap>,
    mut enemy_path: ResMut<EnemyPath>,
) {
    let default_path = Some(
        vec![
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
        ]
    );

    if enemy_path.0.is_none() {
        // info!("Enemy Path not created yet.. Loading Default");
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
    info!("Enemy Path len: {}", gtm.0.iter().filter(|(_, x)| matches!(x, TileType::EnemyMap(_))).count());
}
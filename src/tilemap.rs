use std::collections::HashMap;

use bevy::{prelude::*};

#[derive(Debug, Component, Clone, Default)]
pub enum TileType {
    EnemyMap,
    Blocked,
    #[default]
    Free,
    Tower(TowerType),
}

#[derive(Debug, Clone)]
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
            .insert_resource(GameTilemap::new(15))
            .insert_resource(EnemyPath(None))
            .add_systems(Startup, setup_tilemap);
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
        info!("Enemy Path not created yet.. Loading Default");
        enemy_path.0 = default_path;
    }

    for tile in enemy_path.0.clone().expect("no path..").iter() {
        gtm.0.insert(*tile, TileType::EnemyMap);
    }
    info!("Enemy Path len: {}", gtm.0.len());
}
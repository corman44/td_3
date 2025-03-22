use bevy::{prelude::*, utils::HashMap};

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
            .add_systems(Startup, setup_tilemap);
    }
}

fn setup_tilemap(
    mut gtm: ResMut<GameTilemap>,
) {
    info!("gtm size: {:?}", &gtm.0.len());
}
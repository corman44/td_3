use bevy::prelude::*;

use crate::{tilemap::{EnemyPath, TILE_SCALE}, AppState};

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyWaves>()
            .init_resource::<EnemySpawnTimer>()
            .add_systems(Startup, setup_enemy_waves)
            .add_systems(
                Update,
                (spawn_enemies, move_enemies, tick_enemy_spawn_timer)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Resource, Debug, Clone)]
pub struct EnemySpawnTimer{
    pub timer: Timer,
}


// FIXME enable updating timer value based on the wave seperation time
impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Basic,
    Fast,
    Tanky,
    Boss,
}

#[derive(Debug, Clone, Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub health: f32,
    pub speed: f32,
    pub spawned: bool,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, health: f32, speed: f32) -> Self {
        Self {
            enemy_type,
            health,
            speed,
            spawned: false,
        }
    }
}

/// This is a struct that contain a enemy wave
#[derive(Debug, Clone)]
pub struct EnemyWave {
    // TODO allow custom spawn rates of each enemie (ie. 1 basic enemy every 2 seconds, 1 fast enemy every 5 seconds)
    pub enemies: Vec<Enemy>,
    pub spawn_count: usize,
    /// The time in seconds between each enemy spawn
    pub spawn_time: f32,
}

impl EnemyWave {
    pub fn new(enemies: Vec<Enemy>, spawn_time: f32) -> Self {
        Self {
            enemies,
            spawn_time,
            spawn_count: 0,
        }
    }
}

#[derive(Debug, Clone, Resource, Default)]
pub struct EnemyWaves {
    pub waves: Vec<EnemyWave>,
    pub current_wave: usize,
}

impl EnemyWaves {
    pub fn new(waves: Vec<EnemyWave>) -> Self {
        Self {
            waves,
            current_wave: 0,
        }
    }
}

/// system for moving enemies
fn move_enemies(mut enemies: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemies.iter_mut() {
        if enemy.spawned {
            // TODO add movement logic
            //      - need to move in the direction of the next enemy path, until reached
            transform.translation.x += enemy.speed * time.delta_secs();
        }
    }
}

fn setup_enemy_waves(mut enemy_waves: ResMut<EnemyWaves>) {
    // TODO add more waves and balance them
    let wave1 = EnemyWave::new(
        vec![
            ENEMY_BASIC,
            ENEMY_BASIC,
            ENEMY_BASIC,
            ENEMY_BASIC,
            ENEMY_BASIC,
        ],
        2.0,
    );
    let wave2 = EnemyWave::new(
        vec![
            ENEMY_BASIC,
            ENEMY_BASIC,
            ENEMY_BASIC,
            ENEMY_FAST,
            ENEMY_BASIC,
            ENEMY_BASIC,
        ],
        2.0,
    );
    enemy_waves.waves.push(wave1);
    enemy_waves.waves.push(wave2);
}

fn tick_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    enemy_spawn_timer.timer.tick(time.delta());
}

fn spawn_enemies(
    mut commands: Commands,
    mut enemy_waves: ResMut<EnemyWaves>,
    enemy_path: Res<EnemyPath>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.just_finished() {
        let start_pos = enemy_path.0.clone().expect("No Enemy Path Defined..")[0]; 
        if enemy_waves.current_wave < enemy_waves.waves.len() {
            let curr_wave = enemy_waves.current_wave;
            let wave = &mut enemy_waves.waves[curr_wave];
            if wave.spawn_count < wave.enemies.len() {
                info!("Starting location: {:?}", start_pos);
                info!("Spawn Count: {:?}", wave.spawn_count);
                let enemy = &mut wave.enemies[wave.spawn_count];
                enemy.spawned = true;

                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(3., 3., 3.))),
                    MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
                    Transform::from_translation(Vec3::new(start_pos.x as f32 * TILE_SCALE , start_pos.y as f32 * TILE_SCALE, 0.5)),
                    enemy.clone(),
                ));
                wave.spawn_count += 1;
            }
        }
    }
}

const ENEMY_TANKY: Enemy = Enemy {
    enemy_type: EnemyType::Tanky,
    health: 100.0,
    speed: 10.0,
    spawned: false,
};

const ENEMY_BOSS: Enemy = Enemy {
    enemy_type: EnemyType::Boss,
    health: 500.0,
    speed: 5.0,
    spawned: false,
};

const ENEMY_FAST: Enemy = Enemy {
    enemy_type: EnemyType::Fast,
    health: 50.0,
    speed: 20.0,
    spawned: false,
};

const ENEMY_BASIC: Enemy = Enemy {
    enemy_type: EnemyType::Basic,
    health: 25.0,
    speed: 15.0,
    spawned: false,
};

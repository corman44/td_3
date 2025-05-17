use bevy::{prelude::*, render::mesh};

use crate::{tilemap::EnemyPath, AppState};

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyWaves>()
            .add_systems(Startup, setup_enemy_waves)
            .add_systems(
                Update,
                (spawn_enemies, move_enemies).run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Resource, Debug, Clone)]
pub struct EnemySpawnTimer{
    pub timer: Timer,
}

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
    pub spwaned: bool,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, health: f32, speed: f32) -> Self {
        Self {
            enemy_type,
            health,
            speed,
            spwaned: false,
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
        if enemy.spwaned {
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
    enemy_waves: Res<EnemyWaves>,
    enemy_path: Res<EnemyPath>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let start_pos = enemy_path.0.clone().expect("No Enemy Path Defined..")[0]; 
    if enemy_waves.current_wave < enemy_waves.waves.len() {
        let wave = &enemy_waves.waves[enemy_waves.current_wave];
        if wave.spawn_count < wave.enemies.len() {
            let enemy = &wave.enemies[wave.spawn_count];
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(3., 3., 3.))),
                MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
                Transform::from_translation(Vec3::new(start_pos.x as f32, start_pos.y as f32, 0.5)),
                enemy.clone(),
            ));
        }
    }
}

const ENEMY_TANKY: Enemy = Enemy {
    enemy_type: EnemyType::Tanky,
    health: 100.0,
    speed: 1.0,
    spwaned: false,
};

const ENEMY_BOSS: Enemy = Enemy {
    enemy_type: EnemyType::Boss,
    health: 500.0,
    speed: 0.5,
    spwaned: false,
};

const ENEMY_FAST: Enemy = Enemy {
    enemy_type: EnemyType::Fast,
    health: 50.0,
    speed: 2.0,
    spwaned: false,
};

const ENEMY_BASIC: Enemy = Enemy {
    enemy_type: EnemyType::Basic,
    health: 25.0,
    speed: 1.5,
    spwaned: false,
};

use bevy::{input::common_conditions::input_just_pressed, log::tracing_subscriber::field::debug, prelude::*};
use bevy_rapier3d::prelude::*;
use td_3::tilemap::{self, Tilemap};

#[derive(Component)]
struct Ball;


fn main() {
    let _app = App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            Tilemap
        ))
        .insert_resource(ClearColor(Color::srgb(0.1,0.1,0.1)))
        .add_systems(Startup, (setup_graphics, setup_physics))
        .add_systems(Update, toggle_debug.run_if(input_just_pressed(KeyCode::Space)))
        .run(); 
}

fn toggle_debug(
    keeb: Res<ButtonInput<KeyCode>>,
    mut debug_rend_context: ResMut<DebugRenderContext>,
) {
    if keeb.just_pressed(KeyCode::Space) {
        debug_rend_context.enabled = !debug_rend_context.enabled;
    }
}

fn setup_graphics(
    mut commands: Commands
) {
    // Add a camera so we can see the debug-render.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5.0, 5.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Add a light source
    commands.spawn((
        DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_physics(
    mut commands: Commands,
    mut rapier_config: Query<&mut RapierConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    /* Create the ground. */
    commands
    .spawn(Collider::cuboid(25.0, 0.1, 25.0))
    .insert(Mesh3d(meshes.add(Cuboid::new(50., 0.2, 50.0))))
    .insert(MeshMaterial3d(materials.add(Color::srgb(0.5, 0.5, 0.5))))
    .insert(RigidBody::Fixed)
    .insert(Restitution::coefficient(1.0))
    .insert(Friction::coefficient(0.0))
    .insert(Transform::from_xyz(0.0, 0.0, 0.0));

/* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))))
        .insert(MeshMaterial3d(materials.add(Color::srgb(0.5, 0.75, 0.5))))
        .insert(Restitution::coefficient(1.0))
        .insert(Friction::coefficient(0.0))
        .insert(Ball)
        .insert(Transform::from_xyz(0.0, 3.0, 0.0));


    // rapier_config.single_mut().gravity = Vec3::ZERO;
}

fn print_ball_altitude(
    positions: Query<&Transform, With<Ball>>
) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

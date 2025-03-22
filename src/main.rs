use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_rapier3d::prelude::*;
use td_3::tilemap::Tilemap;

// Overall TODOs
// TODO create a level editor (save and loading levels)

#[derive(Debug, States, Default, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum AppState {
    #[default]
   StartMenu,
   InGame,
   PauseMenu,
   Exit, 
}

fn main() {
    let _app = App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            Tilemap
        ))
        .init_state::<AppState>()
        .insert_resource(ClearColor(Color::srgb(0.1,0.1,0.1)))
        .add_systems(Startup, display_menu)
        .add_systems(Update, menu_button_system.run_if(in_state(AppState::StartMenu)))
        // .add_systems(Startup, (setup_graphics, setup_physics))
        // .add_systems(Update, toggle_debug.run_if(input_just_pressed(KeyCode::Space)))
        .run(); 
}
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

/// starting menu displayed when launching game
/// Screen Flow: ..booting -> StartingMenu -> Game / Level Editor -> Pause -> Settings / Starting / Exit
fn display_menu(
    mut commands: Commands,
) {
    // TODO display simple buttons for starting the game
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            is_active: true,
            ..default()
        },
    ));

    // Spawn Game Button
    commands.spawn(
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    ).with_children( |parent| {
        parent.spawn((
            Button,
            ButtonType::StartGame,
            Text::new("Start Game"),
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
        ));
        // Spawn Exit Button
        parent.spawn((
            Button,
            ButtonType::Exit,
            Text::new("Exit"),
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
        ));
    });

}

#[derive(Debug, Component, Clone)]
pub enum ButtonType {
    StartGame,
    Settings,
    LevelEdit,
    Exit,
}

fn menu_button_system(
    mut buttons: Query<(&Interaction, &mut BackgroundColor, &ButtonType), (Changed<Interaction>, With<Button>)>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color, button_type) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match button_type {
                    ButtonType::StartGame => {
                        info!("Starting Game!");
                    },
                    ButtonType::Settings => {
                        info!("Settings");
                    },
                    ButtonType::LevelEdit => {
                        info!("Level Editting");
                    },
                    ButtonType::Exit => { 
                        info!("Goodbye!");
                        exit.send(AppExit::Success);
                    },
                }
            },
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            },
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            },
        }
    }
}

/// despawn buttons and hide 2d camera
fn despawn_menu(
    mut commands: Commands,
    mut cam2d: Query<&mut Camera, With<Camera2d>>,
    mut buttons: Query<Entity, With<Button>>,
) {
    cam2d.single_mut().is_active = false;
    for each in buttons.iter() {
        commands.entity(each).despawn();
    }
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
        // .insert(Ball)
        .insert(Transform::from_xyz(0.0, 3.0, 0.0));


    // rapier_config.single_mut().gravity = Vec3::ZERO;
}

// fn print_ball_altitude(
//     positions: Query<&Transform, With<Ball>>
// ) {
//     for transform in positions.iter() {
//         println!("Ball altitude: {}", transform.translation.y);
//     }
// }

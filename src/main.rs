use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use td_3::{setup, tilemap::Tilemap, ui::Ui, AppState};

// Overall TODOs
// TODO create a level editor (save and loading levels)

fn main() {
    let _app = App::new()
        .add_plugins((
            DefaultPlugins,
            // RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
            MeshPickingPlugin,
            Tilemap,
            Ui,
            WorldInspectorPlugin::new(),
        ))
        .init_state::<AppState>()
        .insert_resource(ClearColor(Color::srgb(0.1,0.1,0.1)))
        .add_systems(PreStartup, setup)
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

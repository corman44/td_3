use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use td_3::{cam_ctrl::CamCtrl, editor::Editor, game_debug::GameDebug, tilemap::Tilemap, ui::Ui, AppState};

// Overall TODOs
// TODO create a level editor (save and loading levels)

fn main() {
    let _app = App::new()
        .add_plugins((
            DefaultPlugins,
            CamCtrl,
            // RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
            Editor,
            MeshPickingPlugin,
            Tilemap,
            Ui,
            // WorldInspectorPlugin::new(),
            GameDebug,
        ))
        .init_state::<AppState>()
        .insert_resource(ClearColor(Color::srgb(0.53,0.80,0.922)))
        // .add_systems(Update, toggle_debug.run_if(input_just_pressed(KeyCode::Space)))
        .run(); 
}

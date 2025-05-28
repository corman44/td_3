use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use td_3::{cam_ctrl::CamCtrl, editor::Editor, enemy::EnemyPlugin, game_debug::GameDebug, tilemap::Tilemap, ui::Ui, AppState};

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
            EnemyPlugin,
            MeshPickingPlugin,
            Tilemap,
            Ui,
            GameDebug,
            EguiPlugin { enable_multipass_for_primary_context: true},
            WorldInspectorPlugin::new(),
        ))
        .init_state::<AppState>()
        .insert_resource(ClearColor(Color::srgb(0.2,0.2,0.2)))
        .run(); 
}

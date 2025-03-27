
use bevy::prelude::*;

use crate::AppState;

// TODO create UI Buttons for selecting type of path (Verical, Horizontal, corners, etc)
// TODO add functionality to place the paths on existing
// TODO add save functionality (and define format)
// TODO add Load Map functionality 

pub struct Editor;

impl Plugin for Editor {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup);
    }
}

fn setup(
    mut commands: Commands,
    app_state: Res<State<AppState>>,
) {
    if app_state.is_changed() && &AppState::InEditor == app_state.get() {
        // transition to InEditor detected, launch editor
        info!("Launching Editor");
    }
}

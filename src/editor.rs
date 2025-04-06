use bevy::prelude::*;

use crate::{tilemap::GameTilemap, AppState};

// TODO create UI Buttons for selecting type of path (Verical, Horizontal, corners, etc)
// TODO add functionality to place the paths on existing
// TODO add save functionality (and define format)
// TODO don't allow saving unless a path is defined
    // TODO determine if Enemy Path is valid
    // TODO display message of reason for failed save
// TODO add Load Map functionality 

/// Usage
/// Click a Tile Type (Enemy Path, Free, Rock, Water, etc.) then a small version of that tile follows the cursor while selected
/// when clicking a tile the tile type is applied 
pub struct Editor;

impl Plugin for Editor {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup);
    }
}

/// Setup Map Editor
fn setup(
    app_state: Res<State<AppState>>,
    mut commands: Commands,
    mut gtm: ResMut<GameTilemap>,
) {
    if app_state.is_changed() && &AppState::InEditor == app_state.get() {
        // transition to InEditor detected, launch editor
        
        // Reset Map and Redraw it
        gtm.reset_map();
    }
}

use bevy::prelude::*;

use crate::{cam_ctrl::CamState, tilemap::MapState, AppState};


#[derive(Debug, Clone)]
pub struct GameDebug;


impl Plugin for GameDebug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_state_changes);
    }
}

fn print_state_changes(
    app_state: Res<State<AppState>>,
    cam_state: Res<State<CamState>>,
    map_state: Res<State<MapState>>,
) {
    if app_state.is_changed() {
        info!("DBG: AppState {:?}", app_state.get());
    }
    if cam_state.is_changed() {
        info!("DBG: CamState {:?}", cam_state.get());
    }
    if map_state.is_changed() {
        info!("DBG: MapState {:?}", map_state.get());
    }

}
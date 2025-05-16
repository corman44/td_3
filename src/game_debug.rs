use bevy::prelude::*;

use crate::{cam_ctrl::CamState, editor::MiniTileState, tilemap::MapState, AppState};


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
    mt_state:  Res<State<MiniTileState>>,
) {
    if app_state.is_changed() {
        info!("AppState: {:?}", app_state.get());
    }
    if cam_state.is_changed() {
        info!("CamState: {:?}", cam_state.get());
    }
    if map_state.is_changed() {
        info!("MapState: {:?}", map_state.get());
    }
    if mt_state.is_changed() {
        info!("MiniTileState: {:?}", mt_state.get());
    }

}
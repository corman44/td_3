use bevy::prelude::*;

pub mod cam_ctrl;
pub mod editor;
pub mod game_debug;
pub mod tilemap;
pub mod ui;

#[derive(Debug, States, Default, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum AppState {
    #[default]
   StartMenu,
   InGame,
   InEditor,
   PauseMenu,
   Settings,
   ToEditor,
   ToGame,
   Exit, 
}

#[derive(Debug, Event)]
pub struct StartGameEvent;

use bevy::prelude::*;

pub mod tilemap;
pub mod ui;


#[derive(Debug, States, Default, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum AppState {
    #[default]
   StartMenu,
   InGame,
   PauseMenu,
   Exit, 
}

#[derive(Debug, Event)]
pub struct StartGameEvent;
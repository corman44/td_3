use bevy::prelude::*;

use crate::{tilemap::GameTilemap, ui::NORMAL_BUTTON, AppState};

// TODO create UI Buttons for selecting type of path (Verical, Horizontal, corners, etc)
// TODO add functionality to place the paths on existing
// TODO add save functionality (and define format)
// TODO don't allow saving unless a path is defined
    // TODO determine if Enemy Path is valid
    // TODO display message of reason for failed save
// TODO add Load Map functionality 


#[derive(Debug, Component)]
pub enum TilePath {
    Vertical,
    Horizontal,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

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

        // TODO create nodes that put 2 buttons in a row aligned to the bottom left
        commands.spawn((
            Node {
                width: Val::Percent(15.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::End,
                align_content: AlignContent::End,
                justify_items: JustifyItems::Center,
                flex_direction: FlexDirection::Column,
                column_gap: Val::Px(10.0),
                ..default()
            },
        )).with_children(|parent| {
            // Row 1
            parent.spawn(Node {
                flex_direction: FlexDirection::Row,
                ..default()
            }).with_children(|parent| {
                parent.spawn((
                    Button,
                    TilePath::Vertical,
                    Text::new("Vertical"),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ));

                parent.spawn((
                    Button,
                    TilePath::Horizontal,
                    Text::new("Horizontal"),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ));
            });

            // Row 2
            parent.spawn(Node {
                flex_direction: FlexDirection::Row,
                ..default()
            }).with_children(|parent| {
                parent.spawn((
                    Button,
                    TilePath::TopLeft,
                    Text::new("Top Left"),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ));

                parent.spawn((
                    Button,
                    TilePath::TopRight,
                    Text::new("Top Right"),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ));
            });

            // Row 3
            parent.spawn(Node {
                flex_direction: FlexDirection::Row,
                ..default()
            }).with_children(|parent| {
                parent.spawn((
                    Button,
                    TilePath::BottomLeft,
                    Text::new("Bottom Left"),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ));

                parent.spawn((
                    Button,
                    TilePath::BottomRight,
                    Text::new("Bottom Right"),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ));
            });
        }); 
        
        // Reset Map and Redraw it
        gtm.reset_map();
    }
}

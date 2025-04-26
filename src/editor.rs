use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{tilemap::GameTilemap, AppState};

// TODO add functionality to place the paths on existing
// TODO add save functionality (and define format)
// TODO don't allow saving unless a path is defined
    // TODO determine if Enemy Path is valid
    // TODO display message of reason for failed save
// TODO add Load Map functionality 

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Debug, Component)]
struct EditorUI;

#[derive(Debug, Component)]
struct MiniTile;

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
        app.add_systems(Update, setup)
            .add_systems(Update, editor_buttons.run_if(in_state(AppState::InEditor)));
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

        commands.spawn((
            Node {
                width: Val::Percent(20.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::End,
                align_content: AlignContent::End,
                justify_items: JustifyItems::Center,
                flex_direction: FlexDirection::Column,
                column_gap: Val::Px(10.0),
                ..default()
            },
            children![
                // first Row
                (
                    Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    children![
                        (
                            Button,
                            TilePath::Vertical,
                            Text::new("Vertical"),
                            BorderColor(Color::BLACK),
                            BorderRadius::MAX,
                            BackgroundColor(NORMAL_BUTTON),
                        ),
                        (
                            Button,
                            TilePath::Horizontal,
                            Text::new("Horizontal"),
                            BorderColor(Color::BLACK),
                            BorderRadius::MAX,
                            BackgroundColor(NORMAL_BUTTON),
                        ),
                    ]
                ),
                // second Row
                (
                    Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    children![
                        (
                            Button,
                            TilePath::TopLeft,
                            Text::new("Top Left"),
                            BorderColor(Color::BLACK),
                            BorderRadius::MAX,
                            BackgroundColor(NORMAL_BUTTON),
                        ),
                        (
                            Button,
                            TilePath::TopRight,
                            Text::new("Top Right"),
                            BorderColor(Color::BLACK),
                            BorderRadius::MAX,
                            BackgroundColor(NORMAL_BUTTON),
                        ),
                    ]
                ),
                // third Row
                (
                    Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    children![
                        (
                            Button,
                            TilePath::BottomLeft,
                            Text::new("Bottom Left"),
                            BorderColor(Color::BLACK),
                            BorderRadius::MAX,
                            BackgroundColor(NORMAL_BUTTON),
                        ),
                        (
                            Button,
                            TilePath::BottomRight,
                            Text::new("Bottom Right"),
                            BorderColor(Color::BLACK),
                            BorderRadius::MAX,
                            BackgroundColor(NORMAL_BUTTON),
                        ),
                    ]
                ),

            ],
        ));
        
        // Reset Map and Redraw it
        gtm.reset_map();
    }
}

fn editor_buttons(
    mut commands: Commands,
    mut buttons: Query<(&TilePath, &mut BackgroundColor, &Interaction), (Changed<Interaction>, With<EditorUI>, With<Button>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (tile_path, mut color, interaction) in buttons.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                // TODO create 'mini' tile that follows the cursor
                spawn_minitile(&mut commands, &mut meshes, tile_path);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(HOVERED_BUTTON);
            }
            Interaction::None => {
                commands.insert_resource(ClearColor(NORMAL_BUTTON));
            }
        }
    }
}

fn spawn_minitile(
    mut commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    tile_path: &TilePath,
) {
    commands.spawn((
        Mesh2d::from(meshes.add(Rectangle::new(2., 2.))),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        // MiniTile,
        // tile_path.clone(),
    ));
}

fn minitile_cursor_follow(
    mut cursor_mover: Query<&mut Transform, With<MiniTile>>,
    mut cursor_pos: EventReader<MouseMotion>,
) {
    for ev in cursor_pos.read() {
        
    }
    for mut transform in cursor_mover.iter_mut() {
        transform.translation.x += cursor_pos.read().next().unwrap().delta.x;
        transform.translation.z += cursor_pos.read().next().unwrap().delta.y;
    }
}
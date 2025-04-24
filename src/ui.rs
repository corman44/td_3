use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{AppState, StartGameEvent};

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct Ui;

#[derive(Debug, Component)]
struct MenuUI;

impl Plugin for Ui {
    fn build(&self, app: &mut App) {
        app.add_event::<StartGameEvent>()
            .add_systems(PostStartup, display_menu)
            .add_systems(
                Update,
                (menu_button_system)
                    .run_if(in_state(AppState::StartMenu).or(in_state(AppState::PauseMenu))),
            )
            .add_systems(
                Update,
                pause_menu.run_if(
                    not(in_state(AppState::PauseMenu))
                        .and(not(in_state(AppState::StartMenu)))
                        .and(
                            input_just_pressed(KeyCode::KeyP)
                                .or(input_just_pressed(KeyCode::Escape)),
                        ),
                ),
            );
    }
}

fn pause_menu(
    mut app_state: ResMut<NextState<AppState>>,
    mut buttons: Query<&mut Visibility, With<Button>>,
    mut nodes: Query<&mut Node, With<MenuUI>>,
) {
    app_state.set(AppState::PauseMenu);
    for mut node in nodes.iter_mut() {
        node.display = Display::default();
    }
    for mut butt in buttons.iter_mut() {
        butt.toggle_visible_hidden();
    }
}

/// starting menu displayed when launching game
/// Screen Flow: ..booting -> StartingMenu -> Game / Level Editor -> Pause -> Settings / Starting / Exit
fn display_menu(
    mut commands: Commands,
) {
    info!("Displayng Menu");
    // Spawn Game Button
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(10.0),
                ..default()
            },
            Pickable {
                should_block_lower: true,
                ..default()
            },
            MenuUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Button,
                ButtonType::StartGame,
                Text::new("Start Game"),
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
                MenuUI,
            ));
            parent.spawn((
                Button,
                ButtonType::LevelEdit,
                Text::new("LevelEditor"),
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
                MenuUI,
            ));
            // Spawn Settings Button
            parent.spawn((
                Button,
                ButtonType::Settings,
                Text::new("Settings"),
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
                MenuUI,
            ));
            // Spawn Exit Button
            parent.spawn((
                Button,
                ButtonType::Exit,
                Text::new("Exit"),
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
                MenuUI,
            ));
        });
}

#[derive(Debug, Component, Clone)]
pub enum ButtonType {
    StartGame,
    Settings,
    LevelEdit,
    Exit,
}

fn menu_button_system(
    mut app_state: ResMut<NextState<AppState>>,
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor, &ButtonType, &mut Visibility),
        (Changed<Interaction>, With<Button>, With<MenuUI>),
    >,
    mut ev_desp_menu: EventWriter<StartGameEvent>,
    mut exit: EventWriter<AppExit>,
    mut nodes: Query<&mut Node, With<MenuUI>>,
) {
    for (interaction, mut color, button_type, mut vis) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match button_type {
                    ButtonType::StartGame => {
                        // info!("Starting Game!");
                        app_state.set(AppState::ToGame);
                        ev_desp_menu.send(StartGameEvent);
                        despawn_menu(&mut nodes, &mut vis);
                    }
                    ButtonType::Settings => {
                        // info!("Settings");
                        app_state.set(AppState::Settings);
                        despawn_menu(&mut nodes, &mut vis);
                    }
                    ButtonType::LevelEdit => {
                        // info!("Level Editting");
                        app_state.set(AppState::ToEditor); 
                        despawn_menu(&mut nodes, &mut vis);
                    }
                    ButtonType::Exit => {
                        info!("Goodbye!");
                        app_state.set(AppState::Exit);
                        exit.send(AppExit::Success);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

/// Hide buttons and UI Nodes
fn despawn_menu(
    // mut buttons: &mut Query<&mut Visibility, With<Button>>,
    nodes: &mut Query<&mut Node, With<MenuUI>>,
    vis: &mut Mut< '_, Visibility>,
) {
    vis.toggle_visible_hidden();
    for mut each in nodes.iter_mut() {
        each.display = Display::None;

    }
}

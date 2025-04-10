use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{AppState, StartGameEvent};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct Ui;

impl Plugin for Ui {
    fn build(&self, app: &mut App) {
        app.add_event::<StartGameEvent>()
            .add_systems(Startup, display_menu)
            .add_systems(
                Update,
                (menu_button_system)
                    .run_if(in_state(AppState::StartMenu).or(in_state(AppState::PauseMenu))),
            )
            .add_systems(Update, despawn_menu)
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
    mut nodes: Query<&mut Node>,
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
    // Spawn Game Button
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(10.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Button,
                ButtonType::StartGame,
                Text::new("Start Game"),
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ));
            // Spawn Settings Button
            parent.spawn((
                Button,
                ButtonType::Settings,
                Text::new("Settings"),
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ));
            // Spawn Exit Button
            parent.spawn((
                Button,
                ButtonType::Exit,
                Text::new("Exit"),
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
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
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor, &ButtonType),
        (Changed<Interaction>, With<Button>),
    >,
    mut ev_desp_menu: EventWriter<StartGameEvent>,
    mut exit: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
    keeb: Res<ButtonInput<KeyCode>>, 
) {
    for (interaction, mut color, button_type) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match button_type {
                    ButtonType::StartGame => {
                        info!("Starting Game!");
                        app_state.set(AppState::InGame);
                        ev_desp_menu.send(StartGameEvent);
                    }
                    ButtonType::Settings => {
                        info!("Settings");
                    }
                    ButtonType::LevelEdit => {
                        info!("Level Editting");
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

    if keeb.just_pressed(KeyCode::KeyG) {
        info!("Starting Game!");
        app_state.set(AppState::InGame);
        ev_desp_menu.send(StartGameEvent);
    }
}

/// Hide buttons and UI Nodes
fn despawn_menu(
    mut buttons: Query<&mut Visibility, With<Button>>,
    mut ev_desp_menu: EventReader<StartGameEvent>,
    mut nodes: Query<&mut Node>,
) {
    for _ev in ev_desp_menu.read() {
        for mut each in buttons.iter_mut() {
            each.toggle_visible_hidden();
        }
        for mut each in nodes.iter_mut() {
            each.display = Display::None;
        }
    }
}

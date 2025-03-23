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
                        .and(input_just_pressed(KeyCode::KeyP)),
                ),
            );
    }
}

fn pause_menu(
    mut app_state: ResMut<NextState<AppState>>,
    mut cam2d: Query<&mut Camera, With<Camera2d>>,
    mut buttons: Query<&mut Visibility, With<Button>>,
) {
    app_state.set(AppState::PauseMenu);
    cam2d.single_mut().is_active = true;
    for mut butt in buttons.iter_mut() {
        butt.toggle_visible_hidden();
    }
}

/// starting menu displayed when launching game
/// Screen Flow: ..booting -> StartingMenu -> Game / Level Editor -> Pause -> Settings / Starting / Exit
fn display_menu(mut commands: Commands) {
    // TODO display simple buttons for starting the game
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            is_active: true,
            ..default()
        },
    ));

    // Spawn Game Button
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
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
}

/// despawn buttons and hide 2d camera
fn despawn_menu(
    mut buttons: Query<&mut Visibility, With<Button>>,
    mut cam2d: Query<&mut Camera, With<Camera2d>>,
    mut ev_desp_menu: EventReader<StartGameEvent>,
) {
    for _ev in ev_desp_menu.read() {
        cam2d.single_mut().is_active = false;
        for mut each in buttons.iter_mut() {
            each.toggle_visible_hidden();
        }
    }
}

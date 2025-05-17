use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use serde_with::de;

use crate::{cam_ctrl::{CamMoveDir, CamState}, tilemap::TileType, AppState, StartGameEvent};

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct Ui;

#[derive(Debug, Component)]
struct MenuUI;

impl Plugin for Ui {
    fn build(&self, app: &mut App) {
        app.add_event::<StartGameEvent>()
            .add_systems(PostStartup, (display_menu, update_prev_button_state))
            .add_systems(
                Update,
                menu_button_system.run_if(
                    in_state(AppState::StartMenu)
                        .or(in_state(AppState::PauseMenu))
                        .or(in_state(AppState::InEditor)),
                ),
            )
            .add_systems(
                Update,
                pause_menu.run_if(
                    not(in_state(AppState::PauseMenu))
                        .and(not(in_state(AppState::StartMenu)))
                        .and(input_just_pressed(KeyCode::Escape)),
                ),
            )
            .add_systems(
                Update,
                escape_active_menu.run_if(
                    in_state(AppState::PauseMenu)
                    .and(input_just_pressed(KeyCode::Escape))
                )
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
fn display_menu(mut commands: Commands) {
    // Spawn Game Button
    commands.spawn((
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
        children![
            button("Start Game", ButtonType::Menu(MenuType::StartGame)),
            button("Level Editor", ButtonType::Menu(MenuType::LevelEdit)),
            button("Settings", ButtonType::Menu(MenuType::Settings)),
            button("Exit", ButtonType::Menu(MenuType::Exit)),
        ],
    ));
}

#[derive(Debug, Component, Clone)]
pub enum ButtonType {
    Menu(MenuType),
    Editor(TileType),
}

#[derive(Debug, Component, Clone)]
pub enum MenuType {
    StartGame,
    Settings,
    LevelEdit,
    Save,
    Load,
    Exit,
    Clear,
}

#[derive(Debug, Component, Default)]
pub struct PreviousButtonState(pub Interaction);

fn menu_button_system(
    mut app_state: ResMut<NextState<AppState>>,
    mut buttons: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonType,
            &mut Visibility,
        ),
        (Changed<Interaction>, With<Button>),
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
                    ButtonType::Menu(menu) => match menu {
                        MenuType::StartGame => {
                            app_state.set(AppState::ToGame);
                            ev_desp_menu.write(StartGameEvent);
                            despawn_menu(&mut nodes, &mut vis);
                        }
                        MenuType::Settings => {
                            app_state.set(AppState::Settings);
                            despawn_menu(&mut nodes, &mut vis);
                        }
                        MenuType::LevelEdit => {
                            app_state.set(AppState::ToEditor);
                            despawn_menu(&mut nodes, &mut vis);
                        }
                        MenuType::Exit => {
                            info!("Goodbye!");
                            app_state.set(AppState::Exit);
                            exit.write(AppExit::Success);
                        }
                        _ => (),
                    },
                    _ => (),
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
fn despawn_menu(nodes: &mut Query<&mut Node, With<MenuUI>>, vis: &mut Mut<'_, Visibility>) {
    vis.toggle_visible_hidden();
    for mut each in nodes.iter_mut() {
        each.display = Display::None;
    }
}

/// Button creation function for cleaner UI Code
pub fn button<T: Into<String>>(text: T, typ: ButtonType) -> impl Bundle {
    (
        Button,
        typ,
        Text::new(text.into()),
        BorderColor(Color::BLACK),
        BorderRadius::MAX,
        BackgroundColor(NORMAL_BUTTON),
        PreviousButtonState::default(),
    )
}

/// runs Post Update to ensure all systems have an accurate PreviousButtonState
fn update_prev_button_state(
    button_query: Query<(&mut PreviousButtonState, &Interaction), With<Button>>,
) {
    for (mut prev_butt_stat, interaction) in button_query {
        prev_butt_stat.0 = interaction.clone();
    }
}

fn escape_active_menu(
    mut app_state: ResMut<NextState<AppState>>,
    cam_state: Res<State<CamState>>,
    mut nodes: Query<&mut Node, With<MenuUI>>,
    mut vis: Query<&mut Visibility, With<Button>>,
) {
    for mut visabitility in vis.iter_mut() {
        despawn_menu(&mut nodes, &mut visabitility);
    }

    match cam_state.get() {
        CamState::EditorView => app_state.set(AppState::InEditor),
        CamState::GameView => app_state.set(AppState::InGame),
        CamState::Moving(cam_move_dir) => match cam_move_dir {
            CamMoveDir::MoveToEditor => app_state.set(AppState::ToEditor),
            CamMoveDir::MoveToGame => app_state.set(AppState::ToGame),
        },
    }
}

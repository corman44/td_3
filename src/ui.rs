use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{tilemap::TileType, AppState, StartGameEvent};

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
                    .run_if(in_state(AppState::StartMenu)
                    .or(in_state(AppState::PauseMenu))
                    .or(in_state(AppState::InEditor))),
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
}

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
                        _ => ()
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
    )
}

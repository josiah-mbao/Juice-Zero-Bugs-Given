use bevy::prelude::*;

use crate::game_state::{AppState, BossType, GameConfig, Winner};
use crate::player::{ControlType, Health, Player};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_ui)
            .add_systems(
                Update,
                (
                    update_health_bars.run_if(in_state(AppState::InGame)),
                    handle_pause_button.run_if(in_state(AppState::InGame)),
                    handle_p_key_pause.run_if(in_state(AppState::InGame)),
                ),
            )
            .add_systems(OnEnter(AppState::Paused), setup_pause_screen)
            .add_systems(
                Update,
                handle_pause_menu_buttons.run_if(in_state(AppState::Paused)),
            )
            .add_systems(OnExit(AppState::Paused), (cleanup_pause_screen,))
            .add_systems(OnEnter(AppState::MainMenu), reset_winner_on_menu)
            .add_systems(OnExit(AppState::InGame), cleanup_pause_button)
            .add_systems(OnEnter(AppState::GameOver), setup_game_over_screen)
            .add_systems(OnExit(AppState::GameOver), cleanup_game_over_screen);
    }
}

// -- Components (for tagging UI elements) --

#[derive(Component)]
struct HealthBar(u8); // Holds the player ID (1 or 2)

#[derive(Component)]
struct GameOverScreen;

#[derive(Component)]
struct PauseButton;

#[derive(Component)]
struct PauseScreen;

#[derive(Component)]
struct ResumeButton;

#[derive(Component)]
struct ExitButton;

// -- Helper Functions --

fn boss_name(b: BossType) -> &'static str {
    match b {
        BossType::NullPointer => "Null Pointer",
        BossType::UndefinedBehavior => "Undefined Behavior",
        BossType::DataRace => "Data Race",
        BossType::UseAfterFree => "Use After Free",
        BossType::BufferOverflow => "Buffer Overflow",
    }
}

// -- Systems --

fn setup_ui(mut commands: Commands, player_query: Query<(&Player, &ControlType)>) {
    // Player 1 Health Container
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(40.0),
                height: Val::Px(60.0),
                left: Val::Percent(5.0),
                top: Val::Percent(2.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(5.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Label - Find the label for player 1
            let label = player_query
                .iter()
                .find(|(player, _)| player.id == 1)
                .map(|(_, control)| match control {
                    ControlType::Human => "PLAYER",
                    ControlType::AI(_) => "BOSS",
                })
                .unwrap_or("PLAYER");

            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Health bar container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(30.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    border_color: Color::WHITE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::srgb(0.1, 0.9, 0.1).into(),
                            ..default()
                        },
                        HealthBar(1),
                    ));
                });
        });

    // Player 2 Health Container
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(40.0),
                height: Val::Px(60.0),
                right: Val::Percent(5.0),
                top: Val::Percent(2.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(5.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Label - Find the label for player 2
            let label = player_query
                .iter()
                .find(|(player, _)| player.id == 2)
                .map(|(_, control)| match control {
                    ControlType::Human => "PLAYER",
                    ControlType::AI(boss_type) => boss_name(*boss_type),
                })
                .unwrap_or("BOSS");

            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Health bar container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(30.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    border_color: Color::WHITE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::srgb(0.1, 0.9, 0.1).into(),
                            ..default()
                        },
                        HealthBar(2),
                    ));
                });
        });

    // Pause Button - More Centrally Positioned
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(80.0),
                    height: Val::Px(40.0),
                    top: Val::Percent(1.0),
                    left: Val::Percent(47.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                border_color: Color::WHITE.into(),
                border_radius: BorderRadius::all(Val::Px(5.0)),
                ..default()
            },
            PauseButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "PAUSE",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn update_health_bars(
    player_query: Query<(&Health, &Player)>,
    mut health_bar_query: Query<(&mut Style, &HealthBar)>,
) {
    for (mut style, health_bar) in health_bar_query.iter_mut() {
        for (health, player) in player_query.iter() {
            if player.id == health_bar.0 {
                style.width = Val::Percent((health.current as f32 / health.max as f32) * 100.0);
            }
        }
    }
}

fn setup_game_over_screen(mut commands: Commands, winner: Res<Winner>, game_config: Res<GameConfig>) {
    let winner_text = match winner.is_human_winner {
        Some(true) => "PLAYER WINS!".to_string(),
        Some(false) => format!("{} WINS!", boss_name(game_config.boss).to_uppercase()),
        None => "GAME OVER".to_string(),
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            GameOverScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                &winner_text,
                TextStyle {
                    font_size: 80.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            parent.spawn(TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font_size: 60.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Press SPACE to Restart",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn cleanup_game_over_screen(mut commands: Commands, query: Query<Entity, With<GameOverScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_pause_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PauseButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(AppState::Paused);
            }
            Interaction::Hovered => {
                *background_color = Color::srgb(0.4, 0.4, 0.4).into();
            }
            Interaction::None => {
                *background_color = Color::srgb(0.2, 0.2, 0.2).into();
            }
        }
    }
}

fn setup_pause_screen(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(30.0),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                ..default()
            },
            PauseScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "PAUSED",
                TextStyle {
                    font_size: 100.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            parent.spawn(TextBundle::from_section(
                "Escape to Resume",
                TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ));

            // Resume Button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(60.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.2, 0.5, 0.2).into(),
                        border_color: Color::WHITE.into(),
                        border_radius: BorderRadius::all(Val::Px(5.0)),
                        ..default()
                    },
                    ResumeButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "RESUME",
                        TextStyle {
                            font_size: 24.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            // Exit Button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(60.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.5, 0.2, 0.2).into(),
                        border_color: Color::WHITE.into(),
                        border_radius: BorderRadius::all(Val::Px(5.0)),
                        ..default()
                    },
                    ExitButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "EXIT TO MENU",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });
}

fn handle_pause_menu_buttons(
    mut buttons_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&ResumeButton>, Option<&ExitButton>),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut background_color, resume_button, exit_button) in &mut buttons_query {
        let is_resume = resume_button.is_some();
        let is_exit = exit_button.is_some();

        if is_resume {
            match *interaction {
                Interaction::Pressed => {
                    next_state.set(AppState::InGame);
                }
                Interaction::Hovered => {
                    *background_color = Color::srgb(0.4, 0.7, 0.4).into();
                }
                Interaction::None => {
                    *background_color = Color::srgb(0.2, 0.5, 0.2).into();
                }
            }
        } else if is_exit {
            match *interaction {
                Interaction::Pressed => {
                    next_state.set(AppState::MainMenu);
                }
                Interaction::Hovered => {
                    *background_color = Color::srgb(0.7, 0.4, 0.4).into();
                }
                Interaction::None => {
                    *background_color = Color::srgb(0.5, 0.2, 0.2).into();
                }
            }
        }
    }
}

fn cleanup_pause_screen(mut commands: Commands, query: Query<Entity, With<PauseScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_p_key_pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        next_state.set(AppState::Paused);
    }
}

fn reset_winner_on_menu(mut winner: ResMut<Winner>) {
    winner.player_id = None;
    winner.is_human_winner = None;
}

// Cleanup pause button when transitioning away from in-game
fn cleanup_pause_button(mut commands: Commands, query: Query<Entity, With<PauseButton>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

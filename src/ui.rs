use bevy::prelude::*;
use std::time::Duration;

use crate::game_state::{AppState, BossType, GameConfig, Winner};
use crate::menu::BossDisplay;
use crate::player::{ControlType, Health, Player};
use crate::{GameAssets, VictoryDefeatMusic};

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
                    update_combo_counter.run_if(in_state(AppState::InGame)),
                    update_damage_numbers.run_if(in_state(AppState::InGame)),
                    spawn_damage_number.run_if(in_state(AppState::InGame)),
                ),
            )
            .add_systems(OnEnter(AppState::Paused), setup_pause_screen)
            .add_systems(
                Update,
                handle_pause_menu_buttons.run_if(in_state(AppState::Paused)),
            )
            .add_systems(OnExit(AppState::Paused), (cleanup_pause_screen,))
            .add_systems(OnEnter(AppState::MainMenu), reset_winner_on_menu)
            .add_systems(
                OnExit(AppState::InGame),
                (cleanup_pause_button, cleanup_game_ui),
            )
            .add_systems(OnEnter(AppState::GameOver), setup_game_over_screen)
            .add_systems(OnExit(AppState::GameOver), cleanup_game_over_screen)
            .add_systems(OnEnter(AppState::Credits), setup_credits_screen)
            .add_systems(
                Update,
                (update_credits_scroll, handle_credits_input).run_if(in_state(AppState::Credits)),
            )
            .add_systems(OnExit(AppState::Credits), cleanup_credits_screen);
    }
}

// -- Components (for tagging UI elements) --

#[derive(Component)]
struct HealthBar(u8); // Holds the player ID (1 or 2)

#[derive(Component)]
struct HealthBarContainer; // For cleanup of health bar UI containers

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

// Combo and Damage Display
#[derive(Component)]
struct ComboCounter;

#[derive(Component)]
struct DamageNumber {
    timer: Timer,
    velocity: Vec2,
}

#[derive(Component)]
struct CreditsScreen;

#[derive(Component)]
struct CreditsText;

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

#[allow(clippy::type_complexity)]
fn setup_ui(mut commands: Commands, player_query: Query<(&Player, &ControlType)>) {
    // Player 1 Health Container
    commands
        .spawn((
            NodeBundle {
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
            },
            HealthBarContainer,
        ))
        .with_children(|parent| {
            // Label - Find the label for player 1
            let label = player_query
                .iter()
                .find(|(player, _)| player.id == 1)
                .map(|(_, control)| match control {
                    ControlType::Human => "PLAYER 1",
                    ControlType::AI(_) => "BOSS",
                })
                .unwrap_or("PLAYER 1");

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
        .spawn((
            NodeBundle {
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
            },
            HealthBarContainer,
        ))
        .with_children(|parent| {
            // Label - Find the label for player 2
            let label = player_query
                .iter()
                .find(|(player, _)| player.id == 2)
                .map(|(_, control)| match control {
                    ControlType::Human => "PLAYER 2",
                    ControlType::AI(boss_type) => boss_name(*boss_type),
                })
                .unwrap_or("PLAYER 2");

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

    // Central Boss Display (only if vs AI)
    let has_ai = player_query
        .iter()
        .any(|(_, control)| matches!(control, ControlType::AI(_)));
    if has_ai {
        commands
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(50.0),
                    height: Val::Px(40.0),
                    top: Val::Percent(1.0),
                    left: Val::Percent(25.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // Find the boss name
                let boss_label = player_query
                    .iter()
                    .find_map(|(_, control)| match control {
                        ControlType::AI(boss_type) => Some(format!("VS {}", boss_name(*boss_type))),
                        _ => None,
                    })
                    .unwrap_or("VS BOSS".to_string());

                parent.spawn((
                    TextBundle::from_section(
                        boss_label,
                        TextStyle {
                            font_size: 24.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    BossDisplay,
                ));
            });
    }

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

fn setup_game_over_screen(
    mut commands: Commands,
    winner: Res<Winner>,
    _game_config: Res<GameConfig>,
    assets: Res<GameAssets>,
) {
    let (winner_text, text_color) = match (winner.player_id, winner.is_human_winner) {
        (Some(1), Some(true)) | (Some(2), Some(true)) => {
            ("BUG FIXED!".to_string(), Color::srgb(0.0, 1.0, 0.0))
        } // Green for victory
        (Some(_), Some(false)) => ("SEGFAULT".to_string(), Color::srgb(1.0, 0.0, 0.0)), // Red for defeat
        _ => ("DRAW!".to_string(), Color::WHITE),
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
                    color: text_color,
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

    // Play victory or defeat music
    let music_source = match (winner.player_id, winner.is_human_winner) {
        (Some(1), Some(true)) | (Some(2), Some(true)) => assets.victory_music.clone(),
        (Some(_), Some(false)) => assets.defeat_music.clone(),
        _ => assets.victory_music.clone(), // Default to victory for draw
    };

    commands.spawn((
        AudioBundle {
            source: music_source,
            settings: PlaybackSettings::DESPAWN, // One-time sting, auto-despawn when finished
        },
        VictoryDefeatMusic,
    ));
}

fn cleanup_game_over_screen(mut commands: Commands, query: Query<Entity, With<GameOverScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[allow(clippy::type_complexity)]
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

#[allow(clippy::type_complexity)]
fn handle_pause_menu_buttons(
    mut buttons_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&ResumeButton>,
            Option<&ExitButton>,
        ),
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

fn update_combo_counter(
    mut commands: Commands,
    combo_query: Query<Entity, With<ComboCounter>>,
    mut text_query: Query<&mut Text, With<ComboCounter>>,
    mut damage_events: EventReader<crate::combat::DamageEvent>,
) {
    let mut combo_count = 0;

    // Count recent damage events (simplified combo tracking)
    for _ in damage_events.read() {
        combo_count += 1;
    }

    if combo_count > 0 {
        // Update or create combo counter
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!("COMBO: {combo_count}x");
        } else {
            // Create combo counter if it doesn't exist
            commands
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(10.0),
                        left: Val::Percent(45.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            format!("COMBO: {combo_count}x"),
                            TextStyle {
                                font_size: 32.0,
                                color: Color::srgb(1.0, 1.0, 0.0), // Yellow
                                ..default()
                            },
                        ),
                        ComboCounter,
                    ));
                });
        }
    } else {
        // Remove combo counter if combo ended
        for entity in combo_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_damage_number(
    mut commands: Commands,
    mut damage_events: EventReader<crate::combat::DamageEvent>,
    player_query: Query<(&Transform, &Player)>,
) {
    for event in damage_events.read() {
        // Find the player that was damaged by checking if the target entity has a Player component
        if let Ok((transform, _)) = player_query.get(event.target) {
            // Spawn floating damage number as a sprite in world space
            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        format!("-{}", event.damage),
                        TextStyle {
                            font_size: 24.0,
                            color: Color::srgb(1.0, 0.0, 0.0), // Red
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(
                        transform.translation.x + 20.0,
                        transform.translation.y + 50.0,
                        10.0, // Above other sprites
                    ),
                    ..default()
                },
                DamageNumber {
                    timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
                    velocity: Vec2::new(0.0, 50.0), // Float upward
                },
            ));
        }
    }
}

fn update_damage_numbers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut DamageNumber, &mut Text)>,
) {
    for (entity, mut transform, mut damage_number, mut text) in query.iter_mut() {
        // Update timer
        damage_number.timer.tick(time.delta());

        // Move the damage number upward
        transform.translation.y += damage_number.velocity.y * time.delta_seconds();

        // Fade out as timer progresses
        let alpha = 1.0
            - (damage_number.timer.elapsed_secs() / damage_number.timer.duration().as_secs_f32());
        if let Some(section) = text.sections.first_mut() {
            section.style.color = section.style.color.with_alpha(alpha);
        }

        // Remove when timer finishes
        if damage_number.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// Cleanup all game UI elements when exiting InGame state
#[allow(clippy::type_complexity)]
fn cleanup_game_ui(
    mut commands: Commands,
    query: Query<
        Entity,
        Or<(
            With<HealthBar>,
            With<HealthBarContainer>,
            With<BossDisplay>,
            With<ComboCounter>,
            With<DamageNumber>,
        )>,
    >,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// -- Credits Systems --

fn setup_credits_screen(mut commands: Commands, assets: Res<GameAssets>) {
    // Play victory music (loop for credits duration)
    commands.spawn((
        AudioBundle {
            source: assets.victory_music.clone(),
            settings: PlaybackSettings::LOOP,
        },
        VictoryDefeatMusic,
    ));

    // Create credits screen with scrolling text
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
                    row_gap: Val::Px(40.0),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                ..default()
            },
            CreditsScreen,
        ))
        .with_children(|parent| {
            // Title - start at bottom
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Juice: Zero Bugs Given",
                        TextStyle {
                            font_size: 60.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(800.0),
                        ..default()
                    },
                    ..default()
                },
                CreditsText,
            ));

            // Thanks - slightly above title
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Thanks for playing!",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(750.0),
                        ..default()
                    },
                    ..default()
                },
                CreditsText,
            ));

            // Developer
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Developed by 🇿🇲 Josiah Mbao 🇿🇲",
                        TextStyle {
                            font_size: 35.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(680.0),
                        ..default()
                    },
                    ..default()
                },
                CreditsText,
            ));

            // Built with
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Built with Rust & Bevy",
                        TextStyle {
                            font_size: 35.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(620.0),
                        ..default()
                    },
                    ..default()
                },
                CreditsText,
            ));

            // Inspired by
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Inspired by software bugs,",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::srgb(0.8, 0.8, 0.8),
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(550.0),
                        ..default()
                    },
                    ..default()
                },
                CreditsText,
            ));

            // Crab emoji line
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "built with zero bugs given 🦀",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::srgb(0.8, 0.8, 0.8),
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(500.0),
                        ..default()
                    },
                    ..default()
                },
                CreditsText,
            ));

            // Assets
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Assets:",
                        TextStyle {
                            font_size: 28.0,
                            color: Color::srgb(0.7, 0.7, 0.7),
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(430.0),
                        ..default()
                    },
                    ..default()
                },
                CreditsText,
            ));

            // Asset sources
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Kenney.nl",
                        TextStyle {
                            font_size: 25.0,
                            color: Color::srgb(0.6, 0.6, 0.6),
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(380.0),
                        ..default()
                    },
                    ..default()
                },
                CreditsText,
            ));

            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "OpenGameArt.org",
                        TextStyle {
                            font_size: 25.0,
                            color: Color::srgb(0.6, 0.6, 0.6),
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(340.0),
                        ..default()
                    },
                    ..default()
                },
                CreditsText,
            ));

            // Special thanks
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Special thanks:",
                        TextStyle {
                            font_size: 28.0,
                            color: Color::srgb(0.7, 0.7, 0.7),
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(270.0),
                        ..default()
                    },
                    ..default()
                },
                CreditsText,
            ));

            // Communities
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "The Rust & Bevy communities",
                        TextStyle {
                            font_size: 25.0,
                            color: Color::srgb(0.6, 0.6, 0.6),
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(220.0),
                        ..default()
                    },
                    ..default()
                },
                CreditsText,
            ));

            // Copyright
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "© 2025 AfroDev",
                        TextStyle {
                            font_size: 25.0,
                            color: Color::srgb(0.5, 0.5, 0.5),
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(170.0),
                        ..default()
                    },
                    ..default()
                },
                CreditsText,
            ));

            // Exit instruction at bottom (fixed position, not scrolling)
            parent.spawn(TextBundle::from_section(
                "Press SPACE to return to menu",
                TextStyle {
                    font_size: 20.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ));
        });
}

fn update_credits_scroll(
    time: Res<Time>,
    mut query: Query<&mut Style, With<CreditsText>>,
) {
    let scroll_speed = 30.0; // pixels per second
    let delta_y = scroll_speed * time.delta_seconds();

    for mut style in query.iter_mut() {
        if let Val::Px(current) = style.top {
            style.top = Val::Px(current - delta_y);
        }
    }
}

fn handle_credits_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(AppState::MainMenu);
    }
}

fn cleanup_credits_screen(mut commands: Commands, query: Query<Entity, With<CreditsScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

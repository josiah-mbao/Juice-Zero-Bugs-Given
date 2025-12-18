use crate::game_state::{AppState, ArenaType, BossType, Difficulty, GameConfig, PlayerProgress};
use bevy::prelude::*;

fn next_boss(current: BossType) -> BossType {
    match current {
        BossType::NullPointer => BossType::UndefinedBehavior,
        BossType::UndefinedBehavior => BossType::DataRace,
        BossType::DataRace => BossType::UseAfterFree,
        BossType::UseAfterFree => BossType::BufferOverflow,
        BossType::BufferOverflow => BossType::NullPointer,
    }
}

fn prev_boss(current: BossType) -> BossType {
    match current {
        BossType::NullPointer => BossType::BufferOverflow,
        BossType::UndefinedBehavior => BossType::NullPointer,
        BossType::DataRace => BossType::UndefinedBehavior,
        BossType::UseAfterFree => BossType::DataRace,
        BossType::BufferOverflow => BossType::UseAfterFree,
    }
}

fn next_difficulty(current: Difficulty) -> Difficulty {
    match current {
        Difficulty::Easy => Difficulty::Normal,
        Difficulty::Normal => Difficulty::Hard,
        Difficulty::Hard => Difficulty::Easy,
    }
}

fn prev_difficulty(current: Difficulty) -> Difficulty {
    match current {
        Difficulty::Easy => Difficulty::Hard,
        Difficulty::Normal => Difficulty::Easy,
        Difficulty::Hard => Difficulty::Normal,
    }
}

fn boss_name(b: BossType) -> &'static str {
    match b {
        BossType::NullPointer => "Null Pointer",
        BossType::UndefinedBehavior => "Undefined Behavior",
        BossType::DataRace => "Data Race",
        BossType::UseAfterFree => "Use After Free",
        BossType::BufferOverflow => "Buffer Overflow",
    }
}

fn difficulty_name(d: Difficulty) -> &'static str {
    match d {
        Difficulty::Easy => "Easy",
        Difficulty::Normal => "Normal",
        Difficulty::Hard => "Hard",
    }
}

fn next_arena(current: ArenaType) -> ArenaType {
    match current {
        ArenaType::Default => ArenaType::DataRace,
        ArenaType::DataRace => ArenaType::UndefinedBehavior,
        ArenaType::UndefinedBehavior => ArenaType::BufferOverflow,
        ArenaType::BufferOverflow => ArenaType::Default,
    }
}

fn prev_arena(current: ArenaType) -> ArenaType {
    match current {
        ArenaType::Default => ArenaType::BufferOverflow,
        ArenaType::DataRace => ArenaType::Default,
        ArenaType::UndefinedBehavior => ArenaType::DataRace,
        ArenaType::BufferOverflow => ArenaType::UndefinedBehavior,
    }
}

fn arena_name(a: ArenaType) -> &'static str {
    match a {
        ArenaType::Default => "Default",
        ArenaType::DataRace => "Data Race",
        ArenaType::UndefinedBehavior => "Undefined Behavior",
        ArenaType::BufferOverflow => "Buffer Overflow",
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                spawn_menu_background.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(
                Update,
                main_menu_interaction.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(
                Update,
                update_menu_display.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(
                Update,
                menu_button_color.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(
                Update,
                animate_menu_background.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), cleanup_menu)
            .add_systems(OnExit(AppState::MainMenu), cleanup_menu_background)
            .add_systems(OnEnter(AppState::Statistics), setup_statistics_screen)
            .add_systems(
                Update,
                statistics_screen_interaction.run_if(in_state(AppState::Statistics)),
            )
            .add_systems(OnExit(AppState::Statistics), cleanup_statistics_screen);
    }
}

// -- Components --

#[derive(Component)]
struct MenuButtonAction {
    action: MenuAction,
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
pub struct MenuBackgroundCircle;

#[derive(Component)]
pub struct BossDisplay;

#[derive(Component)]
struct DifficultyDisplay;

#[derive(Component)]
struct ModeDisplay;

#[derive(Component)]
struct ArenaDisplay;

#[derive(Component)]
struct ArenaPreview;

#[derive(Debug, Clone, Copy)]
enum MenuAction {
    StartGame,
    Quit,
    NextBoss,
    PrevBoss,
    NextDifficulty,
    PrevDifficulty,
    TogglePlayer2,
    ShowStatistics,
    NextArena,
    PrevArena,
}

// A type alias for the filter used in button interaction queries.
// This avoids the `type_complexity` warning.
type InteractingButtonFilter = (Changed<Interaction>, With<Button>);

// -- Systems --

fn setup_main_menu(mut commands: Commands) {
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
                    padding: UiRect::top(Val::Px(50.0)), // Add padding to move content down
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(), // Transparent to show background image
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "JUICE: ZERO BUGS GIVEN",
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE, // Changed to white
                    ..default()
                },
            ));

            // Additional tagline
            parent.spawn(TextBundle::from_section(
                "Fight the bugs that Rust was designed to defeat!",
                TextStyle {
                    font_size: 18.0, // Increased by 2 points
                    color: Color::WHITE, // Changed to white
                    ..default()
                },
            ));

            // Boss Selection
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(10.0),
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    spawn_menu_button(parent, "<", MenuAction::PrevBoss);
                    parent.spawn((
                        TextBundle::from_section(
                            "BOSS: Null Pointer",
                            TextStyle {
                                font_size: 30.0, // Increased by 2 points
                                color: Color::WHITE, // Changed to white
                                ..default()
                            },
                        ),
                        BossDisplay,
                    ));
                    spawn_menu_button(parent, ">", MenuAction::NextBoss);
                });

            // Difficulty Selection
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    spawn_menu_button(parent, "<", MenuAction::PrevDifficulty);
                    parent.spawn((
                        TextBundle::from_section(
                            "DIFFICULTY: Normal",
                            TextStyle {
                                font_size: 30.0, // Increased by 2 points
                                color: Color::WHITE, // Changed to white
                                ..default()
                            },
                        ),
                        DifficultyDisplay,
                    ));
                    spawn_menu_button(parent, ">", MenuAction::NextDifficulty);
                });

            // Mode Selection
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "MODE: vs AI",
                            TextStyle {
                                font_size: 34.0, // Increased by 2 points
                                color: Color::WHITE, // Changed to white
                                ..default()
                            },
                        ),
                        ModeDisplay,
                    ));
                    spawn_menu_button(parent, "TOGGLE", MenuAction::TogglePlayer2);
                });

            // Arena Selection
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    spawn_menu_button(parent, "<", MenuAction::PrevArena);
                    parent.spawn((
                        TextBundle::from_section(
                            "ARENA: Default",
                            TextStyle {
                                font_size: 30.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        ArenaDisplay,
                    ));
                    spawn_menu_button(parent, ">", MenuAction::NextArena);
                });

            // Arena Preview
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(10.0),
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Px(200.0),
                                height: Val::Px(120.0),
                                ..default()
                            },
                            image: UiImage::default(),
                            ..default()
                        },
                        ArenaPreview,
                    ));
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(20.0),
                        margin: UiRect::top(Val::Px(40.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Start Game Button
                    spawn_menu_button(parent, "START GAME", MenuAction::StartGame);

                    // Statistics Button
                    spawn_menu_button(parent, "STATISTICS", MenuAction::ShowStatistics);

                    // Quit Button
                    spawn_menu_button(parent, "QUIT", MenuAction::Quit);
                });

            // Controls info
            parent.spawn(TextBundle::from_section(
                "Controls: Player 1 - A/D/W/S to move/jump/block, F to attack | Player 2 - Arrows to move/jump, L to attack, Down to block",
                TextStyle {
                    font_size: 14.0, // Increased by 2 points
                    color: Color::WHITE, // Changed to white
                    ..default()
                },
            ));
        });
}

fn spawn_menu_button(parent: &mut ChildBuilder, text: &str, action: MenuAction) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgb(0.15, 0.15, 0.2).into(),
                ..default()
            },
            MenuButtonAction { action },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 26.0, // Increased by 2 points
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn main_menu_interaction(
    mut interaction_query: Query<(&Interaction, &MenuButtonAction), InteractingButtonFilter>,
    mut config: ResMut<GameConfig>,
    mut app_state: ResMut<NextState<AppState>>,
    progress: Res<PlayerProgress>,
) {
    for (interaction, button_action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action.action {
                MenuAction::StartGame => {
                    // Only allow starting if the selected boss is unlocked
                    if progress.is_boss_unlocked(config.boss) {
                        tracing::info!("Starting game!");
                        app_state.set(AppState::InGame);
                    } else {
                        tracing::warn!("Cannot start game with locked boss!");
                    }
                }
                MenuAction::Quit => {
                    tracing::info!("Quitting game!");
                    std::process::exit(0);
                }
                MenuAction::NextBoss => {
                    config.boss = next_boss(config.boss);
                }
                MenuAction::PrevBoss => {
                    config.boss = prev_boss(config.boss);
                }
                MenuAction::NextDifficulty => {
                    config.difficulty = next_difficulty(config.difficulty);
                }
                MenuAction::PrevDifficulty => {
                    config.difficulty = prev_difficulty(config.difficulty);
                }
                MenuAction::TogglePlayer2 => {
                    config.player2_is_human = !config.player2_is_human;
                }
                MenuAction::ShowStatistics => {
                    app_state.set(AppState::Statistics);
                }
                MenuAction::NextArena => {
                    config.arena = next_arena(config.arena);
                    tracing::info!("Arena changed to: {:?}", config.arena);
                }
                MenuAction::PrevArena => {
                    config.arena = prev_arena(config.arena);
                    tracing::info!("Arena changed to: {:?}", config.arena);
                }
            }
        }
    }
}

fn menu_button_color(
    mut query: Query<(&Interaction, &mut BackgroundColor), InteractingButtonFilter>,
) {
    for (interaction, mut color) in &mut query {
        *color = match *interaction {
            Interaction::Pressed => Color::srgb(0.25, 0.25, 0.35).into(),
            Interaction::Hovered => Color::srgb(0.2, 0.2, 0.3).into(),
            Interaction::None => Color::srgb(0.15, 0.15, 0.2).into(),
        };
    }
}

pub fn spawn_menu_background(
    mut commands: Commands,
    assets: Res<crate::GameAssets>,
    query: Query<&MenuBackgroundCircle>,
) {
    // Only spawn if background doesn't already exist
    if query.is_empty() {
        // Spawn fullscreen menu background image
        commands.spawn((
            SpriteBundle {
                texture: assets.menu_background.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(2000.0, 1200.0)), // Even larger to ensure full screen coverage
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, -5.0), // Behind menu UI but above default
                ..default()
            },
            MenuBackgroundCircle, // Reusing component name for cleanup
        ));
    }
}

fn animate_menu_background(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MenuBackgroundCircle>>,
) {
    let t = time.elapsed_seconds();
    for (i, mut transform) in query.iter_mut().enumerate() {
        // Create bounded oscillating movement around center
        let base_x = 0.0;
        let base_y = 0.0;
        let amplitude_x = 25.0; // Move up to 25 units left/right
        let amplitude_y = 20.0; // Move up to 20 units up/down
        let speed = 0.2; // Very slow, gentle movement

        transform.translation.x = base_x + (t * speed + i as f32 * 0.5).sin() * amplitude_x;
        transform.translation.y = base_y + (t * speed + i as f32 * 0.5).cos() * amplitude_y;
    }
}

#[allow(clippy::type_complexity)]
fn update_menu_display(
    mut set: ParamSet<(
        Query<&mut Text, With<BossDisplay>>,
        Query<&mut Text, With<DifficultyDisplay>>,
        Query<&mut Text, With<ModeDisplay>>,
        Query<&mut Text, With<ArenaDisplay>>,
    )>,
    config: Res<GameConfig>,
    progress: Res<PlayerProgress>,
    assets: Res<crate::GameAssets>,
    mut preview_query: Query<&mut UiImage, With<ArenaPreview>>,
) {
    // Update boss display
    for mut text in set.p0().iter_mut() {
        let boss_name_str = boss_name(config.boss);
        let is_unlocked = progress.is_boss_unlocked(config.boss);
            let display_text = if is_unlocked {
                format!("BOSS: {boss_name_str}")
            } else {
                format!("BOSS: {boss_name_str} (LOCKED)")
            };
        text.sections[0].value = display_text;
        text.sections[0].style.color = if is_unlocked {
            Color::WHITE
        } else {
            Color::srgb(0.6, 0.6, 0.6) // Gray for locked
        };
    }

    // Update difficulty display
    for mut text in set.p1().iter_mut() {
        text.sections[0].value = format!("DIFFICULTY: {}", difficulty_name(config.difficulty));
        text.sections[0].style.color = Color::WHITE; // Changed to white for consistency
    }

    // Update mode display
    for mut text in set.p2().iter_mut() {
        let mode = if config.player2_is_human {
            "vs Player"
        } else {
            "vs AI"
        };
        text.sections[0].value = format!("MODE: {mode}");
        text.sections[0].style.color = Color::WHITE; // Changed to white for better readability
        text.sections[0].style.font_size = 32.0; // Slightly larger for better visibility
    }

    // Update arena display
    for mut text in set.p3().iter_mut() {
        text.sections[0].value = format!("ARENA: {}", arena_name(config.arena));
        text.sections[0].style.color = Color::WHITE;
    }

    // Update arena preview
    for mut preview_image in preview_query.iter_mut() {
        let arena_texture = match config.arena {
            ArenaType::Default => assets.arena_backgrounds.null_pointer.clone(),
            ArenaType::DataRace => assets.arena_backgrounds.data_race.clone(),
            ArenaType::UndefinedBehavior => assets.arena_backgrounds.undefined_behavior.clone(),
            ArenaType::BufferOverflow => assets.arena_backgrounds.buffer_overflow.clone(),
        };
        preview_image.texture = arena_texture;
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn cleanup_menu_background(
    mut commands: Commands,
    query: Query<Entity, With<MenuBackgroundCircle>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// -- Statistics Screen --

#[derive(Component)]
struct StatisticsScreen;

#[derive(Component)]
struct BackToMenuButton;

fn setup_statistics_screen(mut commands: Commands, progress: Res<PlayerProgress>) {
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
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.9).into(),
                ..default()
            },
            StatisticsScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "PLAYER STATISTICS",
                TextStyle {
                    font_size: 50.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Global Stats
            parent.spawn(TextBundle::from_section(
                format!("Total Fights: {}", progress.statistics.total_fights),
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            parent.spawn(TextBundle::from_section(
                format!(
                    "Total Wins: {} ({:.1}%)",
                    progress.statistics.total_wins,
                    if progress.statistics.total_fights > 0 {
                        (progress.statistics.total_wins as f32
                            / progress.statistics.total_fights as f32)
                            * 100.0
                    } else {
                        0.0
                    }
                ),
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            parent.spawn(TextBundle::from_section(
                format!(
                    "Win Streak: {} (Best: {})",
                    progress.statistics.current_win_streak, progress.statistics.best_win_streak
                ),
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Boss Stats
            parent.spawn(TextBundle::from_section(
                "PER-BOSS STATISTICS",
                TextStyle {
                    font_size: 35.0,
                    color: Color::srgb(1.0, 1.0, 0.0), // Yellow
                    ..default()
                },
            ));

            // Display stats for each boss
            for boss in [
                BossType::NullPointer,
                BossType::UndefinedBehavior,
                BossType::DataRace,
                BossType::UseAfterFree,
                BossType::BufferOverflow,
            ]
            .iter()
            {
                if let Some(stats) = progress.statistics.boss_stats.get(boss) {
                    parent.spawn(TextBundle::from_section(
                        format!(
                            "{}: {}W/{}L, Best Combo: {}, Fastest: {:.1}s",
                            boss_name(*boss),
                            stats.wins,
                            stats.losses,
                            stats.best_combo,
                            stats.fastest_victory_seconds.unwrap_or(0.0)
                        ),
                        TextStyle {
                            font_size: 25.0,
                            color: Color::srgb(0.8, 0.8, 0.8), // Light gray
                            ..default()
                        },
                    ));
                }
            }

            // Back Button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(60.0),
                            margin: UiRect::top(Val::Px(40.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.2, 0.2, 0.3).into(),
                        ..default()
                    },
                    BackToMenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "BACK TO MENU",
                        TextStyle {
                            font_size: 24.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });
}

fn statistics_screen_interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackToMenuButton>)>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            app_state.set(AppState::MainMenu);
        }
    }
}

fn cleanup_statistics_screen(mut commands: Commands, query: Query<Entity, With<StatisticsScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

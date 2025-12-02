use crate::game_state::{AppState, GameConfig, BossType, Difficulty};
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

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(OnEnter(AppState::MainMenu), spawn_menu_background)
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
            .add_systems(OnExit(AppState::MainMenu), cleanup_menu_background);
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
struct MenuBackgroundCircle;

#[derive(Component)]
pub struct BossDisplay;

#[derive(Component)]
struct DifficultyDisplay;

#[derive(Component)]
struct ModeDisplay;

#[derive(Debug, Clone, Copy)]
enum MenuAction {
    StartGame,
    Quit,
    NextBoss,
    PrevBoss,
    NextDifficulty,
    PrevDifficulty,
    TogglePlayer2,
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
                    ..default()
                },
                background_color: Color::srgb(0.1, 0.1, 0.15).into(),
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
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Subtitle
            parent.spawn(TextBundle::from_section(
                "Fight the bugs that Rust was designed to defeat!",
                TextStyle {
                    font_size: 24.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
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
                                font_size: 28.0,
                                color: Color::WHITE,
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
                                font_size: 28.0,
                                color: Color::WHITE,
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
                                font_size: 28.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        ModeDisplay,
                    ));
                    spawn_menu_button(parent, "TOGGLE", MenuAction::TogglePlayer2);
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(15.0),
                        margin: UiRect::top(Val::Px(40.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Start Game Button
                    spawn_menu_button(parent, "START GAME", MenuAction::StartGame);

                    // Quit Button
                    spawn_menu_button(parent, "QUIT", MenuAction::Quit);
                });

            // Controls info
            parent.spawn(TextBundle::from_section(
                "Controls: Player 1 - A/D to move, F to attack, S to block | Player 2 - Arrows to move, L to attack, Down to block",
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.6, 0.6, 0.6),
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
                    font_size: 24.0,
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
) {
    for (interaction, button_action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action.action {
                MenuAction::StartGame => {
                    tracing::info!("Starting game!");
                    app_state.set(AppState::InGame);
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

fn spawn_menu_background(mut commands: Commands) {
    use rand::Rng;
    // Corrected: Reverted to the non-deprecated function names for your rand version.
    let mut rng = rand::rng();
    for _ in 0..20 {
        let x = rng.random_range(-600.0..600.0);
        let y = rng.random_range(-300.0..300.0);
        let scale = rng.random_range(0.5..2.0);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.3, 0.6, 1.0, 0.15),
                    custom_size: Some(Vec2::splat(80.0 * scale)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, -1.0),
                ..default()
            },
            MenuBackgroundCircle,
        ));
    }
}

fn animate_menu_background(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MenuBackgroundCircle>>,
) {
    let t = time.elapsed_seconds();
    for (i, mut transform) in query.iter_mut().enumerate() {
        transform.translation.y += (t.sin() + i as f32 * 0.1).sin() * 0.5;
        transform.translation.x += (t.cos() + i as f32 * 0.1).cos() * 0.2;
    }
}

fn update_menu_display(
    mut set: ParamSet<(
        Query<&mut Text, With<BossDisplay>>,
        Query<&mut Text, With<DifficultyDisplay>>,
        Query<&mut Text, With<ModeDisplay>>,
    )>,
    config: Res<GameConfig>,
) {
    // Update boss display
    for mut text in set.p0().iter_mut() {
        text.sections[0].value = format!("BOSS: {}", boss_name(config.boss));
    }

    // Update difficulty display
    for mut text in set.p1().iter_mut() {
        text.sections[0].value = format!("DIFFICULTY: {}", difficulty_name(config.difficulty));
    }

    // Update mode display
    for mut text in set.p2().iter_mut() {
        let mode = if config.player2_is_human { "vs Player" } else { "vs AI" };
        text.sections[0].value = format!("MODE: {}", mode);
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

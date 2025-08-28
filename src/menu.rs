use crate::game_state::AppState;
use bevy::prelude::*;

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

#[derive(Debug, Clone, Copy)]
enum MenuAction {
    StartGame,
    Quit,
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
                "Controls: Player 1 - A/D to move, F to attack | Player 2 - Arrows to move, L to attack",
                TextStyle {
                    font_size: 16.0,
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
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button_action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action.action {
                MenuAction::StartGame => {
                    println!("Starting game!");
                    app_state.set(AppState::InGame);
                }
                MenuAction::Quit => {
                    println!("Quitting game!");
                    std::process::exit(0);
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

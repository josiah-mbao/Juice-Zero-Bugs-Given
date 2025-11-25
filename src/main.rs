use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use std::time::Duration;
use tracing_subscriber;

// Import our modules
mod combat;
mod game_state;
mod menu;
mod player;
mod ui;

use combat::CombatPlugin;
use game_state::{AppState, Winner};
use menu::MenuPlugin;
use player::{
    AttackCooldown, BossType, ControlType, FacingDirection, Health, MoveSpeed, Player, PlayerPlugin,
};
use ui::UiPlugin;

fn main() {
    tracing_subscriber::fmt::init();

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Juice: Zero Bugs Given".into(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            // PhysicsDebugPlugin::default(),
            PlayerPlugin,
            CombatPlugin,
            UiPlugin,
            MenuPlugin,
        ))
        .init_state::<AppState>()
        .insert_resource(Winner::default())
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(AppState::InGame), setup) // <-- Add this line
        .add_systems(Update, restart_game.run_if(in_state(AppState::GameOver)))
        .run();
}

fn setup(mut commands: Commands) {
    // Removed: commands.spawn(Camera2dBundle::default());

    // Some ground for the players to stand on
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                // Corrected: Use .srgb() instead of .rgb()
                color: Color::srgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(1200.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -200.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        // Corrected: Use .rectangle() instead of .cuboid()
        Collider::rectangle(1200.0, 50.0),
    ));

    // Left boundary wall
    commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(-650.0, 0.0, 0.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 800.0), // Thin wall, tall enough for screen
    ));

    // Right boundary wall
    commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(650.0, 0.0, 0.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 800.0), // Thin wall, tall enough for screen
    ));

    // -- Player 1 (Human) --
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            transform: Transform::from_xyz(-200.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        // Corrected: Use .rectangle() instead of .cuboid()
        Collider::rectangle(50.0, 100.0),
        combat::Hurtbox,
        Player { id: 1 },
        ControlType::Human,
        Health {
            current: 100,
            max: 100,
        },
        MoveSpeed(300.0),
        FacingDirection::Right,
        AttackCooldown {
            timer: Timer::new(Duration::from_millis(300), TimerMode::Once),
        },
    ));

    // -- Player 2 (AI Boss - starting with NullPointer) --
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 0.0, 1.0),
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            transform: Transform::from_xyz(200.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        // Corrected: Use .rectangle() instead of .cuboid()
        Collider::rectangle(50.0, 100.0),
        combat::Hurtbox,
        Player { id: 2 },
        ControlType::AI(BossType::NullPointer),
        Health {
            current: 100,
            max: 100,
        },
        MoveSpeed(300.0),
        FacingDirection::Left,
        AttackCooldown {
            timer: Timer::new(Duration::from_millis(300), TimerMode::Once),
        },
    ));
}

#[allow(dead_code)]
fn handle_pause_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match app_state.get() {
            AppState::InGame => {
                tracing::info!("Game paused");
                next_state.set(AppState::Paused);
            }
            AppState::Paused => {
                tracing::info!("Game resumed");
                next_state.set(AppState::InGame);
            }
            _ => {}
        }
    }
}

fn restart_game(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut winner: ResMut<Winner>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        winner.player_id = None;
        winner.is_human_winner = None; // Reset winner
        next_state.set(AppState::MainMenu);
        tracing::info!("Returning to main menu!");
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

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
use game_state::{AppState, BossType, GameConfig, Winner};
use menu::MenuPlugin;
use player::{
    AIState, AttackCooldown, BlockState, ControlType, FacingDirection, Health, MoveSpeed, Player, PlayerPlugin,
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
        .insert_resource(GameConfig::default())
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(AppState::InGame), setup) // <-- Add this line
        .add_systems(Update, restart_game.run_if(in_state(AppState::GameOver)))
        .run();
}

fn setup(mut commands: Commands, game_config: Res<GameConfig>) {
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
        BlockState {
            is_blocking: false,
            block_timer: Timer::new(Duration::from_millis(500), TimerMode::Once),
            cooldown_timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
        },
    ));

    // Determine Player 2 sprite and control type
    let (player2_sprite, player2_control, player2_health) = if game_config.player2_is_human {
        (
            Sprite {
                color: Color::srgb(1.0, 0.5, 0.0), // Orange for Player 2
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            ControlType::Human,
            Health {
                current: 100,
                max: 100,
            },
        )
    } else {
        let boss_sprite = match game_config.boss {
            BossType::NullPointer => Sprite {
                color: Color::srgb(0.0, 0.0, 1.0), // Blue
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            BossType::UndefinedBehavior => Sprite {
                color: Color::srgb(0.5, 1.0, 0.5), // Jagged green
                custom_size: Some(Vec2::new(70.0, 100.0)), // Wider for jagged look
                ..default()
            },
            BossType::DataRace => Sprite {
                color: Color::srgb(1.0, 0.0, 0.5), // Red
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            BossType::UseAfterFree => Sprite {
                color: Color::srgb(0.5, 0.0, 1.0), // Purple
                custom_size: Some(Vec2::new(45.0, 110.0)), // Taller
                ..default()
            },
            BossType::BufferOverflow => Sprite {
                color: Color::srgb(1.0, 0.5, 0.0), // Orange
                custom_size: Some(Vec2::new(60.0, 90.0)), // Shorter/wider
                ..default()
            },
        };
        let health_mult = game_config.difficulty.health_multiplier();
        (
            boss_sprite,
            ControlType::AI(game_config.boss),
            Health {
                current: (100.0 * health_mult) as i32,
                max: (100.0 * health_mult) as i32,
            },
        )
    };

    // -- Player 2 --
    let mut player2_entity = commands.spawn((
        SpriteBundle {
            sprite: player2_sprite,
            transform: Transform::from_xyz(200.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        // Corrected: Use .rectangle() instead of .cuboid()
        Collider::rectangle(50.0, 100.0),
        combat::Hurtbox,
        Player { id: 2 },
        player2_control.clone(),
        player2_health,
        MoveSpeed(300.0),
        FacingDirection::Left,
        AttackCooldown {
            timer: Timer::new(Duration::from_millis(300), TimerMode::Once),
        },
        BlockState {
            is_blocking: false,
            block_timer: Timer::new(Duration::from_millis(500), TimerMode::Once),
            cooldown_timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
        },
    ));

    // Add AI state if it's AI
    if matches!(player2_control, ControlType::AI(_)) {
        player2_entity.insert(AIState::default());
    }
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

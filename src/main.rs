use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use std::time::Duration;

// Asset loading
#[derive(Resource)]
pub struct GameAssets {
    pub player_animations: PlayerAnimations,
    pub boss_animations: BossAnimations,
    pub attack_sfx: Handle<AudioSource>,
    pub hit_sfx: Handle<AudioSource>,
    pub jump_sfx: Handle<AudioSource>,
    pub block_sfx: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct PlayerAnimations {
    pub idle: Vec<Handle<Image>>,
    pub walk: Vec<Handle<Image>>,
    pub attack: Vec<Handle<Image>>,
    pub jump: Vec<Handle<Image>>,
    pub hurt: Vec<Handle<Image>>,
}

#[derive(Resource)]
pub struct BossAnimations {
    pub idle: Vec<Handle<Image>>,
    pub attack: Vec<Handle<Image>>,
    pub hurt: Vec<Handle<Image>>,
}

// Animation Components
#[derive(Component)]
pub struct AnimationState {
    pub current_animation: AnimationType,
    pub current_frame: usize,
    pub timer: Timer,
    pub frame_duration: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AnimationType {
    Idle,
    Walking,
    Attacking,
    Jumping,
    Hurt,
}

// Import our modules
mod combat;
mod game_state;
mod menu;
mod player;
mod ui;

use combat::CombatPlugin;
use game_state::{AppState, GameConfig, Winner};
use menu::MenuPlugin;
use player::{
    AIState, AttackCooldown, BlockState, ControlType, FacingDirection, Grounded, Health, MoveSpeed,
    Player, PlayerPlugin,
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
        .add_systems(Startup, (setup_camera, setup_assets))
        .add_systems(OnEnter(AppState::InGame), setup) // <-- Add this line
        .add_systems(Update, (update_animation_state, animate_sprite).run_if(in_state(AppState::InGame)))
        .add_systems(Update, restart_game.run_if(in_state(AppState::GameOver)))
        .run();
}

fn setup(mut commands: Commands, game_config: Res<GameConfig>, assets: Res<GameAssets>) {
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

    // Upper boundary wall (ceiling)
    commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(0.0, 350.0, 0.0)),
        RigidBody::Static,
        Collider::rectangle(1400.0, 50.0), // Wide ceiling, thin enough
    ));

    // -- Player 1 (Human) --
    commands.spawn((
        SpriteBundle {
            texture: assets.player_animations.idle[0].clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            transform: Transform::from_xyz(-200.0, 0.0, 0.0),
            ..default()
        },
        AnimationState {
            current_animation: AnimationType::Idle,
            current_frame: 0,
            timer: Timer::new(Duration::from_secs_f32(0.15), TimerMode::Repeating),
            frame_duration: 0.15,
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
        Grounded(true), // Start grounded
    ));

    // Determine Player 2 sprite and control type
    let (player2_initial_texture, player2_control, player2_health) = if game_config.player2_is_human {
        (
            assets.player_animations.idle[0].clone(), // Use player sprite for human P2
            ControlType::Human,
            Health {
                current: 100,
                max: 100,
            },
        )
    } else {
        let health_mult = game_config.difficulty.health_multiplier();
        (
            assets.boss_animations.idle[0].clone(), // Use boss sprite for AI
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
            texture: player2_initial_texture,
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            transform: Transform::from_xyz(200.0, 0.0, 0.0),
            ..default()
        },
        AnimationState {
            current_animation: AnimationType::Idle,
            current_frame: 0,
            timer: Timer::new(Duration::from_secs_f32(0.15), TimerMode::Repeating),
            frame_duration: 0.15,
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
        Grounded(true), // Start grounded
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

fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_animations = PlayerAnimations {
        idle: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_idle.png")],
        walk: vec![
            asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_walk1.png"),
            asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_walk2.png"),
        ],
        attack: vec![
            asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_kick.png"),
            asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_action1.png"),
            asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_action2.png"),
        ],
        jump: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_jump.png")],
        hurt: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_hurt.png")],
    };

    let boss_animations = BossAnimations {
        idle: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_idle.png")],
        attack: vec![
            asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_action1.png"),
            asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_action2.png"),
        ],
        hurt: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_back.png")],
    };

    let assets = GameAssets {
        player_animations,
        boss_animations,
        // Sound effects - using OGG format for better Bevy compatibility
        attack_sfx: asset_server.load("audio/attack.ogg"),
        hit_sfx: asset_server.load("audio/hit.ogg"),
        jump_sfx: asset_server.load("audio/jump.ogg"),
        block_sfx: asset_server.load("audio/hit.ogg"), // Reuse hit sound for blocks
    };

    commands.insert_resource(assets);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
pub struct AttackInput {
    pub is_attacking: bool,
}

// Animation Systems
fn update_animation_state(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &mut AnimationState,
        &LinearVelocity,
        &Grounded,
        &AttackCooldown,
        &Health,
        &Player,
        &ControlType,
    )>,
) {
    for (mut animation_state, velocity, grounded, attack_cooldown, health, player, control_type) in query.iter_mut() {
        // Check immediate input states for responsive animations
        let (attack_pressed, jump_pressed) = match control_type {
            ControlType::Human => {
                let attack_key = if player.id == 1 { KeyCode::KeyF } else { KeyCode::KeyL };
                let jump_key = if player.id == 1 { KeyCode::KeyW } else { KeyCode::ArrowUp };
                (
                    keyboard_input.pressed(attack_key) || !attack_cooldown.timer.finished(), // Show attack animation while cooling down too
                    keyboard_input.pressed(jump_key)
                )
            },
            ControlType::AI(_) => (
                !attack_cooldown.timer.finished(), // AI uses cooldown
                false // AI jumping is random, not input-based
            ),
        };

        // Determine new animation based on state with higher priority for immediate actions
        let new_animation = if attack_pressed {
            AnimationType::Attacking
        } else if jump_pressed || !grounded.0 {
            AnimationType::Jumping
        } else if health.current < health.max / 3 {
            AnimationType::Hurt
        } else if velocity.x.abs() > 1.0 { // Much lower threshold for immediate walking response
            AnimationType::Walking
        } else {
            AnimationType::Idle
        };

        // Change animation if different (immediate response)
        if animation_state.current_animation != new_animation {
            animation_state.current_animation = new_animation;
            animation_state.current_frame = 0;
            animation_state.timer.reset();
        }

        // Update animation timer
        animation_state.timer.tick(time.delta());
    }
}

fn animate_sprite(
    assets: Res<GameAssets>,
    mut query: Query<(
        &mut Handle<Image>,
        &mut AnimationState,
        &ControlType,
    )>,
) {
    for (mut texture, mut animation_state, control_type) in query.iter_mut() {
        // Check if it's time to advance frame
        if animation_state.timer.finished() {
            // Store frame duration before borrowing animation_state mutably
            let frame_duration = animation_state.frame_duration;

            // Get the animation frames based on type
            let frames = match control_type {
                ControlType::Human => match animation_state.current_animation {
                    AnimationType::Idle => &assets.player_animations.idle,
                    AnimationType::Walking => &assets.player_animations.walk,
                    AnimationType::Attacking => &assets.player_animations.attack,
                    AnimationType::Jumping => &assets.player_animations.jump,
                    AnimationType::Hurt => &assets.player_animations.hurt,
                },
                ControlType::AI(_) => match animation_state.current_animation {
                    AnimationType::Idle => &assets.boss_animations.idle,
                    AnimationType::Walking => &assets.boss_animations.idle, // Boss doesn't have walk anim
                    AnimationType::Attacking => &assets.boss_animations.attack,
                    AnimationType::Jumping => &assets.boss_animations.idle, // Boss doesn't have jump anim
                    AnimationType::Hurt => &assets.boss_animations.hurt,
                },
            };

            // Advance to next frame
            animation_state.current_frame = (animation_state.current_frame + 1) % frames.len();

            // Update texture
            *texture = frames[animation_state.current_frame].clone();

            // Reset timer for next frame
            animation_state.timer.set_duration(Duration::from_secs_f32(frame_duration));
            animation_state.timer.reset();
        }
    }
}

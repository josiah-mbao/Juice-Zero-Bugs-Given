use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use std::time::Duration;

// Asset loading
#[derive(Resource)]
pub struct GameAssets {
    pub player_animations: PlayerAnimations,
    pub boss_animations: BossAnimations,
    pub all_character_animations: AllCharacterAnimations,
    pub menu_background: Handle<Image>,
    pub arena_background: Handle<Image>,
    pub menu_music: Handle<AudioSource>,
    pub attack_sfx: Handle<AudioSource>,
    pub hit_sfx: Handle<AudioSource>,
    pub jump_sfx: Handle<AudioSource>,
    pub block_sfx: Handle<AudioSource>,
    pub victory_music: Handle<AudioSource>,
    pub defeat_music: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct CharacterAnimations {
    pub idle: Vec<Handle<Image>>,
    pub walk: Vec<Handle<Image>>,
    pub attack: Vec<Handle<Image>>,
    pub jump: Vec<Handle<Image>>,
    pub hurt: Vec<Handle<Image>>,
    pub block: Vec<Handle<Image>>,
    pub victory: Vec<Handle<Image>>,
    pub fall: Vec<Handle<Image>>,
    pub special: Vec<Handle<Image>>,
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

#[derive(Resource)]
pub struct AllCharacterAnimations {
    pub player: CharacterAnimations,
    pub zombie: CharacterAnimations,
    pub adventurer: CharacterAnimations,
    pub female: CharacterAnimations,
    pub soldier: CharacterAnimations,
}

// Animation Components
#[derive(Component)]
pub struct AnimationState {
    pub current_animation: AnimationType,
    pub current_frame: usize,
    pub timer: Timer,
    pub frame_duration: f32,
    pub character_type: CharacterType,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AnimationType {
    Idle,
    Walking,
    Attacking,
    Jumping,
    Hurt,
    Blocking,
    Victory,
    Falling,
    SpecialAttack,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CharacterType {
    Player,
    Zombie,
    Adventurer,
    Female,
    Soldier,
}

impl CharacterType {
    pub fn from_boss_type(boss_type: game_state::BossType) -> Self {
        match boss_type {
            game_state::BossType::NullPointer => CharacterType::Zombie,
            game_state::BossType::UndefinedBehavior => CharacterType::Adventurer,
            game_state::BossType::DataRace => CharacterType::Female,
            game_state::BossType::UseAfterFree => CharacterType::Soldier,
            game_state::BossType::BufferOverflow => CharacterType::Player, // Use player as alternate boss
        }
    }
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
        .add_systems(OnEnter(AppState::InGame), (setup, stop_victory_defeat_music))
        .add_systems(
            Update,
            play_menu_music.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(OnExit(AppState::MainMenu), stop_menu_music)
        .add_systems(
            Update,
            (update_animation_state, animate_sprite).run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, restart_game.run_if(in_state(AppState::GameOver)))
        .run();
}

fn setup(mut commands: Commands, game_config: Res<GameConfig>, assets: Res<GameAssets>) {
    // Arena background - positioned behind everything
    commands.spawn(SpriteBundle {
        texture: assets.arena_background.clone(),
        transform: Transform::from_xyz(0.0, 0.0, -10.0), // Behind all game elements
        ..default()
    });

    // Removed: commands.spawn(Camera2dBundle::default());

    // Invisible ground for the players to stand on (physics only, no visual)
    commands.spawn((
        RigidBody::Static,
        // Corrected: Use .rectangle() instead of .cuboid()
        Collider::rectangle(1200.0, 50.0),
        Transform::from_xyz(0.0, -200.0, 0.0),
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
            character_type: CharacterType::Player, // Human player always uses Player character
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
    let (player2_initial_texture, player2_control, player2_health, player2_character_type) = if game_config.player2_is_human
    {
        (
            assets.player_animations.idle[0].clone(), // Use player sprite for human P2
            ControlType::Human,
            Health {
                current: 100,
                max: 100,
            },
            CharacterType::Player, // Human P2 uses Player character
        )
    } else {
        let health_mult = game_config.difficulty.health_multiplier();
        let character_type = CharacterType::from_boss_type(game_config.boss);
        let initial_texture = match character_type {
            CharacterType::Player => assets.all_character_animations.player.idle[0].clone(),
            CharacterType::Zombie => assets.all_character_animations.zombie.idle[0].clone(),
            CharacterType::Adventurer => assets.all_character_animations.adventurer.idle[0].clone(),
            CharacterType::Female => assets.all_character_animations.female.idle[0].clone(),
            CharacterType::Soldier => assets.all_character_animations.soldier.idle[0].clone(),
        };
        (
            initial_texture, // Use character-specific sprite for AI
            ControlType::AI(game_config.boss),
            Health {
                current: (100.0 * health_mult) as i32,
                max: (100.0 * health_mult) as i32,
            },
            character_type,
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
            character_type: player2_character_type,
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
        idle: vec![asset_server
            .load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_idle.png")],
        walk: vec![
            asset_server
                .load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_walk1.png"),
            asset_server
                .load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_walk2.png"),
        ],
        attack: vec![
            asset_server
                .load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_kick.png"),
            asset_server
                .load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_action1.png"),
            asset_server
                .load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_action2.png"),
        ],
        jump: vec![asset_server
            .load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_jump.png")],
        hurt: vec![asset_server
            .load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_hurt.png")],
    };

    let boss_animations = BossAnimations {
        idle: vec![asset_server
            .load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_idle.png")],
        attack: vec![
            asset_server
                .load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_action1.png"),
            asset_server
                .load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_action2.png"),
        ],
        hurt: vec![asset_server
            .load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_back.png")],
    };

    // Load all character animations
    let all_character_animations = AllCharacterAnimations {
        player: CharacterAnimations {
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
            block: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_back.png")],
            victory: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_cheer1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_cheer2.png"),
            ],
            fall: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_fall.png")],
            special: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_kick.png")],
        },
        zombie: CharacterAnimations {
            idle: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_idle.png")],
            walk: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_walk1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_walk2.png"),
            ],
            attack: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_kick.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_action1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_action2.png"),
            ],
            jump: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_jump.png")],
            hurt: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_hurt.png")],
            block: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_back.png")],
            victory: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_cheer1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_cheer2.png"),
            ],
            fall: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_fall.png")],
            special: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_kick.png")],
        },
        adventurer: CharacterAnimations {
            idle: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_idle.png")],
            walk: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_walk1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_walk2.png"),
            ],
            attack: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_kick.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_action1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_action2.png"),
            ],
            jump: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_jump.png")],
            hurt: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_hurt.png")],
            block: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_back.png")],
            victory: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_cheer1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_cheer2.png"),
            ],
            fall: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_fall.png")],
            special: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_kick.png")],
        },
        female: CharacterAnimations {
            idle: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_idle.png")],
            walk: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_walk1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_walk2.png"),
            ],
            attack: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_kick.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_action1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_action2.png"),
            ],
            jump: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_jump.png")],
            hurt: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_hurt.png")],
            block: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_back.png")],
            victory: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_cheer1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_cheer2.png"),
            ],
            fall: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_fall.png")],
            special: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_kick.png")],
        },
        soldier: CharacterAnimations {
            idle: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_idle.png")],
            walk: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_walk1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_walk2.png"),
            ],
            attack: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_kick.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_action1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_action2.png"),
            ],
            jump: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_jump.png")],
            hurt: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_hurt.png")],
            block: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_back.png")],
            victory: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_cheer1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_cheer2.png"),
            ],
            fall: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_fall.png")],
            special: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_kick.png")],
        },
    };

    let assets = GameAssets {
        player_animations,
        boss_animations,
        all_character_animations,
        menu_background: asset_server.load("home-screen.png"),
        arena_background: asset_server.load("a-vibrant-2d-fighting-game-arena.png"),
        menu_music: asset_server.load("audio/menu_music.ogg"),
        // Sound effects - using OGG format for better Bevy compatibility
        attack_sfx: asset_server.load("audio/attack.ogg"),
        hit_sfx: asset_server.load("audio/hit.ogg"),
        jump_sfx: asset_server.load("audio/jump.ogg"),
        block_sfx: asset_server.load("audio/hit.ogg"), // Reuse hit sound for blocks
        victory_music: asset_server.load("audio/victory_sting.ogg"),
        defeat_music: asset_server.load("audio/game_over.ogg"),
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
    winner: Res<crate::game_state::Winner>,
    mut query: Query<(
        &mut AnimationState,
        &LinearVelocity,
        &Grounded,
        &AttackCooldown,
        &Health,
        &Player,
        &ControlType,
        Option<&BlockState>,
    )>,
) {
    for (mut animation_state, velocity, grounded, attack_cooldown, health, player, control_type, block_state) in
        query.iter_mut()
    {
        // Check immediate input states for responsive animations
        let (attack_pressed, jump_pressed) = match control_type {
            ControlType::Human => {
                let attack_key = if player.id == 1 {
                    KeyCode::KeyF
                } else {
                    KeyCode::KeyL
                };
                let jump_key = if player.id == 1 {
                    KeyCode::KeyW
                } else {
                    KeyCode::ArrowUp
                };
                (
                    keyboard_input.pressed(attack_key) || !attack_cooldown.timer.finished(), // Show attack animation while cooling down too
                    keyboard_input.pressed(jump_key),
                )
            }
            ControlType::AI(_) => (
                !attack_cooldown.timer.finished(), // AI uses cooldown
                false,                             // AI jumping is random, not input-based
            ),
        };

        // Check for blocking
        let is_blocking = block_state.map(|bs| bs.is_blocking).unwrap_or(false);

        // Check for victory (human winner and this is a human player)
        let is_victorious = winner.is_human_winner.unwrap_or(false) && matches!(control_type, ControlType::Human);

        // Determine new animation based on state with higher priority for immediate actions
        let new_animation = if is_victorious {
            AnimationType::Victory
        } else if is_blocking {
            AnimationType::Blocking
        } else if attack_pressed {
            AnimationType::Attacking
        } else if jump_pressed || !grounded.0 {
            AnimationType::Jumping
        } else if health.current < health.max / 3 {
            AnimationType::Hurt
        } else if velocity.x.abs() > 1.0 {
            // Much lower threshold for immediate walking response
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
    mut query: Query<(&mut Handle<Image>, &mut AnimationState)>,
) {
    for (mut texture, mut animation_state) in query.iter_mut() {
        // Check if it's time to advance frame
        if animation_state.timer.finished() {
            // Store frame duration before borrowing animation_state mutably
            let frame_duration = animation_state.frame_duration;

            // Get the animation frames based on character type and animation type
            let frames = match animation_state.character_type {
                CharacterType::Player => match animation_state.current_animation {
                    AnimationType::Idle => &assets.all_character_animations.player.idle,
                    AnimationType::Walking => &assets.all_character_animations.player.walk,
                    AnimationType::Attacking => &assets.all_character_animations.player.attack,
                    AnimationType::Jumping => &assets.all_character_animations.player.jump,
                    AnimationType::Hurt => &assets.all_character_animations.player.hurt,
                    AnimationType::Blocking => &assets.all_character_animations.player.block,
                    AnimationType::Victory => &assets.all_character_animations.player.victory,
                    AnimationType::Falling => &assets.all_character_animations.player.fall,
                    AnimationType::SpecialAttack => &assets.all_character_animations.player.special,
                },
                CharacterType::Zombie => match animation_state.current_animation {
                    AnimationType::Idle => &assets.all_character_animations.zombie.idle,
                    AnimationType::Walking => &assets.all_character_animations.zombie.walk,
                    AnimationType::Attacking => &assets.all_character_animations.zombie.attack,
                    AnimationType::Jumping => &assets.all_character_animations.zombie.jump,
                    AnimationType::Hurt => &assets.all_character_animations.zombie.hurt,
                    AnimationType::Blocking => &assets.all_character_animations.zombie.block,
                    AnimationType::Victory => &assets.all_character_animations.zombie.victory,
                    AnimationType::Falling => &assets.all_character_animations.zombie.fall,
                    AnimationType::SpecialAttack => &assets.all_character_animations.zombie.special,
                },
                CharacterType::Adventurer => match animation_state.current_animation {
                    AnimationType::Idle => &assets.all_character_animations.adventurer.idle,
                    AnimationType::Walking => &assets.all_character_animations.adventurer.walk,
                    AnimationType::Attacking => &assets.all_character_animations.adventurer.attack,
                    AnimationType::Jumping => &assets.all_character_animations.adventurer.jump,
                    AnimationType::Hurt => &assets.all_character_animations.adventurer.hurt,
                    AnimationType::Blocking => &assets.all_character_animations.adventurer.block,
                    AnimationType::Victory => &assets.all_character_animations.adventurer.victory,
                    AnimationType::Falling => &assets.all_character_animations.adventurer.fall,
                    AnimationType::SpecialAttack => &assets.all_character_animations.adventurer.special,
                },
                CharacterType::Female => match animation_state.current_animation {
                    AnimationType::Idle => &assets.all_character_animations.female.idle,
                    AnimationType::Walking => &assets.all_character_animations.female.walk,
                    AnimationType::Attacking => &assets.all_character_animations.female.attack,
                    AnimationType::Jumping => &assets.all_character_animations.female.jump,
                    AnimationType::Hurt => &assets.all_character_animations.female.hurt,
                    AnimationType::Blocking => &assets.all_character_animations.female.block,
                    AnimationType::Victory => &assets.all_character_animations.female.victory,
                    AnimationType::Falling => &assets.all_character_animations.female.fall,
                    AnimationType::SpecialAttack => &assets.all_character_animations.female.special,
                },
                CharacterType::Soldier => match animation_state.current_animation {
                    AnimationType::Idle => &assets.all_character_animations.soldier.idle,
                    AnimationType::Walking => &assets.all_character_animations.soldier.walk,
                    AnimationType::Attacking => &assets.all_character_animations.soldier.attack,
                    AnimationType::Jumping => &assets.all_character_animations.soldier.jump,
                    AnimationType::Hurt => &assets.all_character_animations.soldier.hurt,
                    AnimationType::Blocking => &assets.all_character_animations.soldier.block,
                    AnimationType::Victory => &assets.all_character_animations.soldier.victory,
                    AnimationType::Falling => &assets.all_character_animations.soldier.fall,
                    AnimationType::SpecialAttack => &assets.all_character_animations.soldier.special,
                },
            };

            // Advance to next frame
            animation_state.current_frame = (animation_state.current_frame + 1) % frames.len();

            // Update texture
            *texture = frames[animation_state.current_frame].clone();

            // Reset timer for next frame
            animation_state
                .timer
                .set_duration(Duration::from_secs_f32(frame_duration));
            animation_state.timer.reset();
        }
    }
}

// Menu Music Systems
#[derive(Component)]
struct MenuMusic;

fn play_menu_music(
    mut commands: Commands,
    assets: Option<Res<GameAssets>>,
    music_query: Query<&MenuMusic>,
) {
    // Only play music if assets are loaded and no music is already playing
    if let Some(assets) = assets {
        if music_query.is_empty() {
            println!("🎵 Starting menu music..."); // Use println for guaranteed output
            commands.spawn((
                AudioBundle {
                    source: assets.menu_music.clone(),
                    settings: PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Loop,
                        volume: bevy::audio::Volume::new(10.0), // Increased volume significantly
                        ..default()
                    },
                },
                MenuMusic,
            ));
            println!("🎵 Menu music spawned!");
        }
    } else {
        println!("🎵 Waiting for assets to load...");
    }
}

fn stop_menu_music(mut commands: Commands, query: Query<Entity, With<MenuMusic>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// Victory/Defeat Music Systems
#[derive(Component)]
pub struct VictoryDefeatMusic;

fn stop_victory_defeat_music(mut commands: Commands, query: Query<Entity, With<VictoryDefeatMusic>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

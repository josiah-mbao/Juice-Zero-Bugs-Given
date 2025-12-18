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
    pub arena_backgrounds: ArenaBackgrounds,
    pub menu_music: Handle<AudioSource>,
    pub attack_sfx: Handle<AudioSource>,
    pub hit_sfx: Handle<AudioSource>,
    pub jump_sfx: Handle<AudioSource>,
    pub block_sfx: Handle<AudioSource>,
    pub victory_music: Handle<AudioSource>,
    pub defeat_music: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct ArenaBackgrounds {
    pub null_pointer: Handle<Image>,
    pub undefined_behavior: Handle<Image>,
    pub data_race: Handle<Image>,
    pub use_after_free: Handle<Image>,
    pub buffer_overflow: Handle<Image>,
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
    pub light_attack: Vec<Handle<Image>>,
    pub heavy_attack: Vec<Handle<Image>>,
    pub kick_attack: Vec<Handle<Image>>,
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
    LightAttack,
    HeavyAttack,
    KickAttack,
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
use game_state::{AppState, GameConfig, PlayerProgress, Winner};
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
        .insert_resource(GameConfig::load_config())
        .insert_resource(PlayerProgress::load_progress())
        .add_systems(Startup, (setup_camera, setup_assets))
        .add_systems(
            OnEnter(AppState::InGame),
            (cleanup_old_arenas, setup, stop_victory_defeat_music),
        )
        .add_systems(
            Update,
            (update_animation_state, animate_sprite).run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, play_menu_music.run_if(in_state(AppState::MainMenu)))
        .add_systems(OnExit(AppState::MainMenu), stop_menu_music)
        .add_systems(Update, save_config_on_change)
        .add_systems(Update, restart_game.run_if(in_state(AppState::GameOver)))
        .run();
}

#[derive(Component)]
struct ArenaBackground;

fn setup(mut commands: Commands, game_config: Res<GameConfig>, assets: Res<GameAssets>) {
    // Clean up old arena backgrounds first
    commands.spawn_empty().insert(CleanupArenaBackground);

    // Get the correct arena background based on selected arena
    let arena_texture = match game_config.arena {
        game_state::ArenaType::Default => {
            tracing::info!("SELECTED ARENA: Default - Loading null_pointer texture");
            assets.arena_backgrounds.null_pointer.clone()
        }
        game_state::ArenaType::DataRace => {
            tracing::info!("SELECTED ARENA: Data Race - Loading data_race texture");
            assets.arena_backgrounds.data_race.clone()
        }
        game_state::ArenaType::UndefinedBehavior => {
            tracing::info!(
                "SELECTED ARENA: Undefined Behavior - Loading undefined_behavior texture"
            );
            assets.arena_backgrounds.undefined_behavior.clone()
        }
        game_state::ArenaType::BufferOverflow => {
            tracing::info!("SELECTED ARENA: Buffer Overflow - Loading buffer_overflow texture");
            assets.arena_backgrounds.buffer_overflow.clone()
        }
    };

    // Arena background - positioned behind everything, zoomed out by 50%
    commands.spawn((
        SpriteBundle {
            texture: arena_texture,
            sprite: Sprite {
                custom_size: Some(Vec2::new(1200.0, 700.0)), // 50% smaller for zoom out effect
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -10.0), // Behind all game elements
            ..default()
        },
        ArenaBackground,
    ));

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
    let (player2_initial_texture, player2_control, player2_health, player2_character_type) =
        if game_config.player2_is_human {
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
                CharacterType::Adventurer => {
                    assets.all_character_animations.adventurer.idle[0].clone()
                }
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

#[derive(Component)]
struct CleanupArenaBackground;

fn cleanup_old_arenas(
    mut commands: Commands,
    query: Query<Entity, With<ArenaBackground>>,
    cleanup_query: Query<Entity, With<CleanupArenaBackground>>,
) {
    // Remove cleanup marker
    for entity in cleanup_query.iter() {
        commands.entity(entity).despawn();
    }

    // Remove old arena backgrounds that might exist from previous sessions
    // This ensures we don't have multiple arenas stacked on top of each other
    for entity in query.iter() {
        commands.entity(entity).despawn();
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
            light_attack: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_action1.png")],
            heavy_attack: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_action1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_action2.png"),
            ],
            kick_attack: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Player/Poses/player_kick.png")],
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
            light_attack: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_action1.png")],
            heavy_attack: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_action1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_action2.png"),
            ],
            kick_attack: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Zombie/Poses/zombie_kick.png")],
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
            light_attack: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_action1.png")],
            heavy_attack: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_action1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_action2.png"),
            ],
            kick_attack: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Adventurer/Poses/adventurer_kick.png")],
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
            light_attack: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_action1.png")],
            heavy_attack: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_action1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_action2.png"),
            ],
            kick_attack: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Female/Poses/female_kick.png")],
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
            light_attack: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_action1.png")],
            heavy_attack: vec![
                asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_action1.png"),
                asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_action2.png"),
            ],
            kick_attack: vec![asset_server.load("sprites/kenney_platformer-characters/PNG/Soldier/Poses/soldier_kick.png")],
        },
    };

    let arena_backgrounds = ArenaBackgrounds {
        null_pointer: asset_server.load("a-vibrant-2d-fighting-game-arena.png"), // Default arena
        undefined_behavior: asset_server.load("undefined_behaviour_arena.png"),
        data_race: asset_server.load("data_race_arena.png"),
        use_after_free: asset_server.load("buffer_overflow_arena.png"), // Use buffer_overflow for use_after_free
        buffer_overflow: asset_server.load("buffer_overflow_arena.png"), // This is correct
    };

    let assets = GameAssets {
        player_animations,
        boss_animations,
        all_character_animations,
        menu_background: asset_server.load("home-screen.png"),
        arena_backgrounds,
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
#[allow(clippy::type_complexity)]
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
    for (
        mut animation_state,
        velocity,
        grounded,
        attack_cooldown,
        health,
        player,
        control_type,
        block_state,
    ) in query.iter_mut()
    {
        // Check immediate input states for responsive animations
        let (light_attack_pressed, heavy_attack_pressed, kick_attack_pressed, jump_pressed) =
            match control_type {
                ControlType::Human => {
                    let (light_key, heavy_key, kick_key) = if player.id == 1 {
                        (KeyCode::KeyF, KeyCode::KeyR, KeyCode::KeyT) // P1: F=Light, R=Heavy, T=Kick
                    } else {
                        (KeyCode::KeyL, KeyCode::KeyO, KeyCode::KeyP) // P2: L=Light, O=Heavy, P=Kick
                    };
                    let jump_key = if player.id == 1 {
                        KeyCode::KeyW
                    } else {
                        KeyCode::ArrowUp
                    };
                    (
                        keyboard_input.pressed(light_key)
                            || (!attack_cooldown.timer.finished()
                                && matches!(
                                    animation_state.current_animation,
                                    AnimationType::LightAttack
                                )),
                        keyboard_input.pressed(heavy_key)
                            || (!attack_cooldown.timer.finished()
                                && matches!(
                                    animation_state.current_animation,
                                    AnimationType::HeavyAttack
                                )),
                        keyboard_input.pressed(kick_key)
                            || (!attack_cooldown.timer.finished()
                                && matches!(
                                    animation_state.current_animation,
                                    AnimationType::KickAttack
                                )),
                        keyboard_input.pressed(jump_key),
                    )
                }
                ControlType::AI(_) => (
                    !attack_cooldown.timer.finished()
                        && matches!(
                            animation_state.current_animation,
                            AnimationType::LightAttack
                        ),
                    !attack_cooldown.timer.finished()
                        && matches!(
                            animation_state.current_animation,
                            AnimationType::HeavyAttack
                        ),
                    !attack_cooldown.timer.finished()
                        && matches!(animation_state.current_animation, AnimationType::KickAttack),
                    false, // AI jumping is random, not input-based
                ),
            };

        // Check for blocking
        let is_blocking = block_state.map(|bs| bs.is_blocking).unwrap_or(false);

        // Check for victory (human winner and this is a human player)
        let is_victorious =
            winner.is_human_winner.unwrap_or(false) && matches!(control_type, ControlType::Human);

        // Determine new animation based on state with higher priority for immediate actions
        let new_animation = if is_victorious {
            AnimationType::Victory
        } else if is_blocking {
            AnimationType::Blocking
        } else if light_attack_pressed {
            AnimationType::LightAttack
        } else if heavy_attack_pressed {
            AnimationType::HeavyAttack
        } else if kick_attack_pressed {
            AnimationType::KickAttack
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
                    AnimationType::Attacking | AnimationType::LightAttack => {
                        &assets.all_character_animations.player.light_attack
                    }
                    AnimationType::HeavyAttack => {
                        &assets.all_character_animations.player.heavy_attack
                    }
                    AnimationType::KickAttack => {
                        &assets.all_character_animations.player.kick_attack
                    }
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
                    AnimationType::Attacking | AnimationType::LightAttack => {
                        &assets.all_character_animations.zombie.light_attack
                    }
                    AnimationType::HeavyAttack => {
                        &assets.all_character_animations.zombie.heavy_attack
                    }
                    AnimationType::KickAttack => {
                        &assets.all_character_animations.zombie.kick_attack
                    }
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
                    AnimationType::Attacking | AnimationType::LightAttack => {
                        &assets.all_character_animations.adventurer.light_attack
                    }
                    AnimationType::HeavyAttack => {
                        &assets.all_character_animations.adventurer.heavy_attack
                    }
                    AnimationType::KickAttack => {
                        &assets.all_character_animations.adventurer.kick_attack
                    }
                    AnimationType::Jumping => &assets.all_character_animations.adventurer.jump,
                    AnimationType::Hurt => &assets.all_character_animations.adventurer.hurt,
                    AnimationType::Blocking => &assets.all_character_animations.adventurer.block,
                    AnimationType::Victory => &assets.all_character_animations.adventurer.victory,
                    AnimationType::Falling => &assets.all_character_animations.adventurer.fall,
                    AnimationType::SpecialAttack => {
                        &assets.all_character_animations.adventurer.special
                    }
                },
                CharacterType::Female => match animation_state.current_animation {
                    AnimationType::Idle => &assets.all_character_animations.female.idle,
                    AnimationType::Walking => &assets.all_character_animations.female.walk,
                    AnimationType::Attacking | AnimationType::LightAttack => {
                        &assets.all_character_animations.female.light_attack
                    }
                    AnimationType::HeavyAttack => {
                        &assets.all_character_animations.female.heavy_attack
                    }
                    AnimationType::KickAttack => {
                        &assets.all_character_animations.female.kick_attack
                    }
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
                    AnimationType::Attacking | AnimationType::LightAttack => {
                        &assets.all_character_animations.soldier.light_attack
                    }
                    AnimationType::HeavyAttack => {
                        &assets.all_character_animations.soldier.heavy_attack
                    }
                    AnimationType::KickAttack => {
                        &assets.all_character_animations.soldier.kick_attack
                    }
                    AnimationType::Jumping => &assets.all_character_animations.soldier.jump,
                    AnimationType::Hurt => &assets.all_character_animations.soldier.hurt,
                    AnimationType::Blocking => &assets.all_character_animations.soldier.block,
                    AnimationType::Victory => &assets.all_character_animations.soldier.victory,
                    AnimationType::Falling => &assets.all_character_animations.soldier.fall,
                    AnimationType::SpecialAttack => {
                        &assets.all_character_animations.soldier.special
                    }
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

fn stop_victory_defeat_music(
    mut commands: Commands,
    query: Query<Entity, With<VictoryDefeatMusic>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn save_config_on_change(config: Res<GameConfig>) {
    // Only save if the config has changed
    if config.is_changed() {
        GameConfig::save_config(config.as_ref());
    }
}

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

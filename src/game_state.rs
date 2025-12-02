use bevy::prelude::*;

// This enum represents the main states of our game
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame, // For now, we start directly in the game
    GameOver,
    #[allow(dead_code)]
    Paused,
}

// Resource to track the winner
#[derive(Resource, Default)]
pub struct Winner {
    pub player_id: Option<u8>,
    pub is_human_winner: Option<bool>,
}

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty::Normal
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BossType {
    NullPointer,
    UndefinedBehavior,
    DataRace,
    UseAfterFree,
    BufferOverflow,
}

impl Default for BossType {
    fn default() -> Self {
        BossType::NullPointer
    }
}

#[derive(Resource)]
pub struct GameConfig {
    pub difficulty: Difficulty,
    pub boss: BossType,
    pub player2_is_human: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            difficulty: Difficulty::default(),
            boss: BossType::default(),
            player2_is_human: false,
        }
    }
}

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

#[derive(Debug, Clone, Copy, Default)]
pub enum Difficulty {
    Easy,
    #[default]
    Normal,
    Hard,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum BossType {
    #[default]
    NullPointer,
    UndefinedBehavior,
    DataRace,
    UseAfterFree,
    BufferOverflow,
}

#[derive(Resource, Default)]
pub struct GameConfig {
    pub difficulty: Difficulty,
    pub boss: BossType,
    pub player2_is_human: bool,
}

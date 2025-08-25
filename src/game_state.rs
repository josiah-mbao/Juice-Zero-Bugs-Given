use bevy::prelude::*;

// This enum represents the main states of our game
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    InGame, // For now, we start directly in the game
    GameOver,
}

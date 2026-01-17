use wasm_bindgen::prelude::*;

// Include shared modules
mod combat;
mod game_state;
mod menu;
mod player;
mod ui;

// Include the shared game logic
include!("shared.rs");

// For web builds, expose the main function via wasm-bindgen
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    // Call the game main function
    game_main();
}

// For native builds, the main function is in native_main.rs
#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    game_main();
}

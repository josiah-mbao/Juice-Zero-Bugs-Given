# Juice: Zero Bugs Given

[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Powered by Bevy](https://img.shields.io/badge/Powered%20by-Bevy-blue?logo=bevy)](https://bevyengine.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

<img width="1392" height="860" alt="Screenshot 2025-08-28 at 10 08 21" src="https://github.com/user-attachments/assets/ccddc26f-4967-428d-a8cd-e5d3f22b9d9e" />

**Juice: Zero Bugs Given** is a Rust-powered 2D fighter game where you play as *Juice*, battling bosses inspired by infamous programming nightmares. Each battle represents a classic bug that Rust was designed to defeat, blending coding humor with arcade-style action.

---

## 📊 Project Status

**✅ Core Features Complete**: Full single-player battle system with AI opponents implemented!

- **Combat System**: ✅ Complete with hitboxes, hurtboxes, health management, and damage calculations
- **AI Boss System**: ✅ Fully implemented with unique behaviors for each programming bug type
- **UI/UX**: ✅ Polished interface with health bars, labels, and winner announcements
- **Arena**: ✅ Contained fighting environment with boundary walls
- **Game Flow**: ✅ Complete menu system, pause, and restart functionality
- **Visual Effects**: ✅ Boss sprites, particle systems, and color coding
- **Audio Setup**: ✅ Framework ready for sound effects and music
- **Pause System**: ✅ Multi-modal pause with UI button and keyboard shortcuts

---

## 🎮 Features

* **Single Player Battles**: Face off against AI bosses in intense 1v1 combat
* **Multiple Unique Bosses**: Experience distinct AI opponents based on programming bugs:
  * **🔵 Null Pointer** – Erratic movement, sporadic attacks, vanishing tactics (Blue sprite)
  * **🟢 Undefined Behavior** – Unpredictable glitchy movement, random timing attacks (Jagged green sprite)
  * **🔴 Data Race** – Aggressive approach/retreat cycles, rapid-fire attacks when close (Crimson sprite)
  * **🟣 Use After Free** – Steady aggressive pursuit with regular interval attacks (Tall purple sprite)
  * **🟠 Buffer Overflow** – Slow but powerful movement with devastating attacks (Wide orange sprite)
* **Rich Visual Feedback**: Dynamic health bars with specific boss names (e.g., "NULL POINTER", "DATA RACE"), clear winner announcements, particle effects on damage
* **Particle Effects System**: Hit explosions with physics-based red particles for satisfying combat feedback
* **Audio Framework**: Ready for sound effects and background music (attack sounds, hit effects, victory music)
* **Advanced Pause System**: Multi-modal pause with centralized UI button, P key shortcut, and full pause menu with Resume/Quit options
* **Containment System**: Invisible arena boundaries prevent falling off screen edges
* **Difficulty Scaling**: Easy/Normal/Hard modes with speed and attack frequency adjustments
* **Intuitive Controls**: Simple keyboard controls for accessible gameplay
* **Built with Rust**: Memory-safe, high-performance architecture using Bevy game engine

---

## 🕹️ How to Play

### Controls

| Action | Player 1 | Player 2 |
| :--- | :---: | :---: |
| Move Left | `A` | `←` (Left Arrow) |
| Move Right | `D` | `→` (Right Arrow) |
| Attack | `F` | `L` |

**In-Game Controls:**
* **P** or **Click PAUSE button**: Pause game with menu options
* **Escape**: Resume from pause (keyboard alternative)
* **Space (on Game Over screen):** Return to Main Menu

---

## 🛠 Tech Stack

* **Rust** as the core language.
* [**Bevy**](https://bevyengine.org/) as the 2D game engine.
* [**bevy_xpbd**](https://github.com/jshvrsn/bevy_xpbd) for 2D physics.
* **Cargo** for dependency management.

---

## 🚀 Getting Started

### Prerequisites

1.  **Rust Toolchain**: You must have the Rust toolchain installed. You can get it from [rustup.rs](https://rustup.rs/).
2.  **OS Dependencies**: Bevy has some [OS-specific dependencies](https://bevyengine.org/learn/book/getting-started/setup/#install-os-dependencies) (e.g., for audio and windowing on Linux). Please install them for your platform.

### Installation & Running

1.  Clone the repo:
    ```bash
    git clone https://github.com/josiah-mbao/Juice-Zero-Bugs-Given.git
    cd Juice-Zero-Bugs-Given
    ```

2.  Build and run the game:
    ```bash
    cargo run
    ```
    For better performance, run in release mode:
    ```bash
    cargo run --release
    ```

---

## 🎯 Game Features Details

### Boss AI Behaviors
- **Easy Difficulty**: 30% slower movement, 50% less frequent attacks
- **Normal Difficulty**: Balanced gameplay
- **Hard Difficulty**: 30% faster movement, 30% more frequent attacks

### Visual System
- **Boss Sprites**: Each boss has unique colors and shapes representing their bug nature
- **Particle Effects**: 5 red particles spawn on hits with physics simulation
- **UI Labels**: Health bars show specific boss names instead of generic "BOSS"

### Pause System
- **Central Pause Button**: Gray button positioned 25% from top, centered
- **P Key Shortcut**: Keyboard pause during gameplay
- **Pause Menu**: Shows "PAUSED", "Escape to Resume", Resume and Exit buttons
- **State Preservation**: Game state frozen during pause

### Audio Framework
- **Integration Ready**: Bevy's audio system integrated
- **Event System**: Combat events structured for sound playback
- **Extensible**: Easy to add sound files for attacks, hits, background music

---

## 🤝 Contributing

Contributions are welcome! If you have an idea for a new 'bug' boss (like a `Segfault` or `Off-by-One`) or want to fix an issue, feel free to fork the repo and submit a pull request.

**Ideas for Enhancement:**
- Add victory/defeat music and sound effects
- Implement local multiplayer support
- Add visual effects for special attacks
- Create boss unlock progression
- Add customizable controls

---

## 📝 License

This project is licensed under the MIT License. See the `LICENSE` file for details.

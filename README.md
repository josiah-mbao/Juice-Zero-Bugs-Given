# Juice: Zero Bugs Given 🕹️

[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Powered by Bevy](https://img.shields.io/badge/Powered%20by-Bevy-blue?logo=bevy)](https://bevyengine.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![itch.io](https://img.shields.io/badge/Available%20on-itch.io-fa5c5c?logo=itch.io)](https://josiah-mbao.itch.io/juice-zero-bugs-given)

<img width="1392" height="860" alt="Screenshot 2025-08-28 at 10 08 21" src="https://github.com/user-attachments/assets/ccddc26f-4967-428d-a8cd-e5d3f22b9d9e" />

**Juice: Zero Bugs Given** is a Rust-powered 2D fighter game where you play as *Juice*, battling bosses inspired by infamous programming nightmares. Each battle represents a classic bug that Rust was designed to defeat, blending coding humor with arcade-style action and atmospheric background music.

---

## 📊 Project Status

**🎉 POLISHED & READY FOR ITCH.IO**: Complete single-player battle system with professional presentation!

### ✅ Core Combat Features
- **Combat System**: ✅ Complete with hitboxes, hurtboxes, health management, and damage calculations
- **Jumping Mechanics**: ✅ Ground-based jumping with physics and collision detection
- **Blocking System**: ✅ Defensive blocking with invulnerability and reduced knockback
- **AI Boss System**: ✅ Fully implemented with unique behaviors for each programming bug type

### ✅ Visual & Audio Polish
- **Menu Background**: ✅ Custom animated background with smooth floating effects
- **UI/UX Polish**: ✅ All text in crisp white with increased font sizes for readability
- **Background Music**: ✅ Atmospheric looping music in menu with auto-play/stop
- **Victory/Defeat Music**: ✅ Unique victory and defeat stings with proper cleanup
- **Visual Effects**: ✅ Boss sprites, particle systems, color coding, and blocking feedback
- **Arena**: ✅ Contained fighting environment with boundary walls and ceiling
- **Victory/Defeat Screen**: ✅ Full-screen overlay with "BUG FIXED"/"SEGFAULT" text

### ✅ Game Systems
- **Game Flow**: ✅ Complete menu system, pause, and restart functionality
- **Audio Framework**: ✅ Full sound effects and background music integration
- **Pause System**: ✅ Multi-modal pause with UI button and keyboard shortcuts
- **Difficulty Scaling**: ✅ Easy/Normal/Hard modes with balanced progression
- **Character Variety**: ✅ All 5 character types used with unique boss mappings
- **Animation System**: ✅ Complete fighting game animations (blocking, victory poses, etc.)

---

## 🎮 Features

* **Single Player Battles**: Face off against AI bosses in intense 1v1 combat
* **Multiple Unique Bosses**: Experience distinct AI opponents based on programming bugs, each with unique characters and animations:
  * **🔵 Null Pointer** – Erratic movement, sporadic attacks, vanishing tactics (Zombie character)
  * **🟢 Undefined Behavior** – Unpredictable glitchy movement, random timing attacks (Adventurer character)
  * **🔴 Data Race** – Aggressive approach/retreat cycles, rapid-fire attacks when close (Female character)
  * **🟣 Use After Free** – Steady aggressive pursuit with regular interval attacks (Soldier character)
  * **🟠 Buffer Overflow** – Slow but powerful movement with devastating attacks (Player character variant)
* **Rich Visual Feedback**: Dynamic health bars with specific boss names (e.g., "NULL POINTER", "DATA RACE"), clear winner announcements, particle effects on damage
* **Particle Effects System**: Hit explosions with physics-based red particles for satisfying combat feedback
* **Atmospheric Audio**: Background music in menu with sound effects for attacks, hits, and blocks
* **Polished Menu**: Custom animated background with floating effects and crisp white text
* **Advanced Pause System**: Multi-modal pause with centralized UI button, P key shortcut, and full pause menu with Resume/Quit options
* **Containment System**: Invisible arena boundaries prevent falling off screen edges
* **Difficulty Scaling**: Easy/Normal/Hard modes with speed and attack frequency adjustments
* **Intuitive Controls**: Simple keyboard controls for accessible gameplay
* **Built with Rust**: Memory-safe, high-performance architecture using Bevy game engine

---

<img width="1279" height="749" alt="2d figher" src="https://github.com/user-attachments/assets/4319f63e-2a65-4013-b27c-1bdc9b87bcb5" />


## 🕹️ How to Play

### Controls

| Action | Player 1 | Player 2 |
| :--- | :---: | :---: |
| Move Left | `A` | `←` (Left Arrow) |
| Move Right | `D` | `→` (Right Arrow) |
| Jump | `W` | `↑` (Up Arrow) |
| Block | `S` | `↓` (Down Arrow) |
| Light Attack | `F` | `L` |
| Heavy Attack | `R` | `O` |
| Kick | `T` | `P` |

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

### Combat Mechanics
- **Jumping System**: Ground-based physics jumping with 600-unit upward impulse
- **Blocking Defense**: 0.5-second invulnerability window with reduced knockback (2-second cooldown)
- **Ground Detection**: Collision-based system prevents air jumping
- **Visual Blocking Feedback**: Blue tint appears when blocking successfully

### Boss AI Behaviors
- **Easy Difficulty**: 30% slower movement, 50% less frequent attacks
- **Normal Difficulty**: Balanced gameplay
- **Hard Difficulty**: 30% faster movement, 30% more frequent attacks
- **AI Jumping**: Bosses occasionally jump for unpredictable movement (2% chance when grounded)

### Visual System
- **Boss Sprites**: Each boss has unique colors and shapes representing their bug nature
- **Blocking Effects**: Players turn blue when blocking, "Attack blocked!" messages appear
- **Particle Effects**: 5 red particles spawn on hits with physics simulation
- **UI Labels**: Health bars show specific boss names instead of generic "BOSS"

### Pause System
- **Central Pause Button**: Gray button positioned 25% from top, centered
- **P Key Shortcut**: Keyboard pause during gameplay
- **Pause Menu**: Shows "PAUSED", "Escape to Resume", Resume and Exit buttons
- **State Preservation**: Game state frozen during pause

### Audio Framework
- **Background Music**: ✅ Atmospheric looping music in main menu with auto-play/stop
- **Victory/Defeat Music**: ✅ Unique 10-second victory sting and defeat music with auto-cleanup
- **Sound Effects**: ✅ Attack sounds, hit effects, and block feedback
- **Event System**: Combat events structured for sound playback
- **Audio Management**: State-aware music system with proper cleanup

### Character & Animation System
- **Full Character Variety**: ✅ All 5 character types (Player, Zombie, Adventurer, Female, Soldier) fully utilized
- **Unique Boss Characters**: ✅ Each boss type mapped to distinct character with unique animations
- **Fighting Game Animations**: ✅ Blocking, victory poses, falling, special attacks, and hurt states
- **Animation Priority System**: ✅ Blocking > Victory > Attacking > Jumping > Hurt > Walking > Idle
- **Asset Utilization**: ✅ Increased from 9.2% to 100% of available character animations

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
